#!/bin/bash
# PreToolUse: workflow-guard.sh — TDD フロー強制
#
# 実装ファイルの Edit/Write 前にテストファイルの変更が存在することを確認する。
# テストを先に書く（RED → GREEN → REFACTOR）フローを構造的に強制。
#
# 対象: scripts/*.sh（test-* を除く）
# 許可: テストファイル、ドキュメント、設定ファイル、.claude/、conf/、Makefile
# ブロック条件: git working tree にテスト関連ファイルの変更がない
#
# exit 0 = 許可, exit 2 = ブロック（Claude にメッセージを返す）
#
# Override: WORKFLOW_GUARD_SKIP=1 で全てスキップ

set -uo pipefail

# Override
[ "${WORKFLOW_GUARD_SKIP:-0}" = "1" ] && exit 0

INPUT=$(cat)

# ツール名を取得
TOOL=$(echo "$INPUT" | grep -o '"tool_name":"[^"]*"' | sed 's/"tool_name":"//;s/".*//')

# Edit/Write 以外はスルー
case "$TOOL" in
    Edit|Write) ;;
    *) exit 0 ;;
esac

# file_path を取得
FILE_PATH=$(echo "$INPUT" | grep -o '"file_path":"[^"]*"' | sed 's/"file_path":"//;s/".*//' | head -1)
[ -z "$FILE_PATH" ] && exit 0

# プロジェクトルートからの相対パスに変換
PROJECT_DIR="${CLAUDE_PROJECT_DIR:-$(pwd)}"
REL_PATH="${FILE_PATH#"$PROJECT_DIR"/}"

# PROJECT_DIR 外のファイルはスキップ（cross-repo 編集は各リポジトリの責務）
[ "$REL_PATH" = "$FILE_PATH" ] && exit 0

# --- 常に許可するパターン ---

# ドキュメント
case "$REL_PATH" in
    *.md) exit 0 ;;
    *.txt) exit 0 ;;
esac

# 設定・インフラ
case "$REL_PATH" in
    .claude/*) exit 0 ;;
    .spec/*) exit 0 ;;
    .contextus/*) exit 0 ;;
    conf/*) exit 0 ;;
    Makefile) exit 0 ;;
    *.json) exit 0 ;;
    *.toml) exit 0 ;;
    *.yaml|*.yml) exit 0 ;;
esac

# テストファイル自体
case "$REL_PATH" in
    */test-*|*/test_*|test-*|test_*) exit 0 ;;
    */tests/*|tests/*) exit 0 ;;
    *.bats) exit 0 ;;
esac

# --- 実装ファイル: テスト変更を要求 ---

# ガード対象の拡張子は L2 が定義する（.claude/hooks/workflow-guard.extensions）
# ファイルがなければガードしない（安全デフォルト）
EXTENSIONS_FILE="$PROJECT_DIR/.claude/hooks/workflow-guard.extensions"
if [ ! -f "$EXTENSIONS_FILE" ]; then
    exit 0
fi

# 拡張子を取得（コメント・空行を除外）
FILE_EXT="${REL_PATH##*.}"
if ! grep -v '^#' "$EXTENSIONS_FILE" | grep -v '^$' | grep -qxF "$FILE_EXT"; then
    exit 0  # extensions に含まれない拡張子はスルー
fi

# git working tree にテスト関連ファイルの変更があるか
cd "$PROJECT_DIR" 2>/dev/null || exit 0

test_changes=$(
    {
        git diff --name-only 2>/dev/null
        git diff --cached --name-only 2>/dev/null
    } | grep -iE '(test[-_]|\.bats$|/tests/)' | head -1
)

if [ -n "$test_changes" ]; then
    exit 0
fi

# ブロック: テスト変更なし
cat >&2 << BLOCK
TDD 違反: 実装ファイル ($REL_PATH) の編集にはテストファイルの変更が必要です。

次の手順で進めてください:
  1) テストファイルを Write（実装ではなくテストを先に書く）
  2) git add でテストファイルを stage
  3) テストを実行して RED（失敗）を確認
  4) そのあとで実装ファイルを編集
BLOCK
exit 2
