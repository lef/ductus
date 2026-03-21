#!/bin/bash
# bootstrap-claude-v2.sh — .contextus/ + .claude/ を contextus レイヤーから構築する
#
# 新設計: symlink ではなくコピー。L-prefix で出自を保持。
# .contextus/ = upstream 管理（bootstrap が完全制御）
# .claude/rules/L*- = upstream からコピー（bootstrap が管理）
# .claude/rules/*.md (L prefix なし) = ユーザー独自（bootstrap は触らない）
#
# 使い方: scripts/bootstrap-claude-v2.sh

set -euo pipefail

PROJECT_DIR="${PROJECT_DIR:-$(pwd)}"
CONTEXTUS_DIR="${PROJECT_DIR}/.contextus"

cd "$PROJECT_DIR"

# --- repos 検出 ---
# デフォルト: PROJECT_DIR の sibling（host の標準配置）
# CONTEXTUS_REPOS_BASE で override 可能（sandbox 等で repos が別の場所にある場合）
_repos_base="${CONTEXTUS_REPOS_BASE:-$(dirname "$PROJECT_DIR")}"

L0_REPO="${CONTEXTUS_L0:-contextus}"
L1_REPO="${CONTEXTUS_L1:-contextus-claude}"
L2_REPO="${CONTEXTUS_L2-contextus-dev}"
L3_REPO="${CONTEXTUS_L3-contextus-dev-sh}"

# 必須: L0, L1
for _repo in "$L0_REPO" "$L1_REPO"; do
    if [ ! -d "$_repos_base/$_repo" ]; then
        echo "error: $_repos_base/$_repo が見つかりません" >&2
        exit 1
    fi
done

# --- .contextus/ にコピー ---
echo ":: .contextus/ を構築..." >&2
mkdir -p "$CONTEXTUS_DIR"

_copy_layer() {
    local level="$1" repo="$2"
    local src="$_repos_base/$repo"
    local dst="$CONTEXTUS_DIR/$repo"

    [ -d "$src" ] || return 0

    # upstream を pull
    if [ -d "$src/.git" ]; then
        (cd "$src" && git pull -q 2>/dev/null) || true
    fi

    rm -rf "$dst"
    mkdir -p "$dst"

    # 各種ディレクトリ/ファイルをコピー
    for _dir in rules hooks agents skills specs enforcement.d; do
        [ -d "$src/$_dir" ] && cp -r "$src/$_dir" "$dst/"
    done
    for _file in settings.json statusline.sh workflow-guard.extensions registry.jsonl bootstrap.sh; do
        [ -f "$src/$_file" ] && cp "$src/$_file" "$dst/"
    done

    local commit
    commit="$(cd "$src" && git rev-parse --short HEAD 2>/dev/null || echo "unknown")"
    echo ":: $level: $repo@$commit → .contextus/$repo/" >&2
}

_copy_layer "L0" "$L0_REPO"
_copy_layer "L1" "$L1_REPO"
[ -n "$L2_REPO" ] && _copy_layer "L2" "$L2_REPO"
[ -n "$L3_REPO" ] && _copy_layer "L3" "$L3_REPO"

# --- LAYERS マニフェスト ---
echo ":: LAYERS マニフェスト生成..." >&2
_layers_file="$CONTEXTUS_DIR/layers"
: > "$_layers_file"
for _pair in "L0:$L0_REPO" "L1:$L1_REPO" "L2:$L2_REPO" "L3:$L3_REPO"; do
    _level="${_pair%%:*}"
    _repo="${_pair#*:}"
    [ -n "$_repo" ] || continue
    _src="$_repos_base/$_repo"
    [ -d "$_src" ] || continue
    _commit="$(cd "$_src" && git rev-parse --short HEAD 2>/dev/null || echo "unknown")"
    _branch="$(cd "$_src" && git rev-parse --abbrev-ref HEAD 2>/dev/null || echo "unknown")"
    echo "$_level: $_repo@$_branch $_commit $(date '+%Y-%m-%d')" >> "$_layers_file"
done

# --- .claude/ に配置 ---
echo ":: .claude/ を構築..." >&2
mkdir -p .claude/rules .claude/hooks .claude/agents .claude/skills

# L1: hooks, agents, skills, settings, statusline
_l1="$CONTEXTUS_DIR/$L1_REPO"
[ -d "$_l1/hooks" ] && cp -r "$_l1/hooks/." .claude/hooks/
[ -d "$_l1/agents" ] && cp -r "$_l1/agents/." .claude/agents/
[ -d "$_l1/skills" ] && cp -r "$_l1/skills/." .claude/skills/
[ -f "$_l1/settings.json" ] && cp "$_l1/settings.json" .claude/settings.json
[ -f "$_l1/statusline.sh" ] && cp "$_l1/statusline.sh" .claude/statusline.sh

# rules: repos 名サブディレクトリにコピー（Ownership Separation）
# bootstrap が管理するのは .claude/rules/{repos名}/ のみ
# ユーザーが .claude/rules/ トップレベルに置いたファイルは触らない

for _pair in "L0:$L0_REPO" "L1:$L1_REPO" "L2:$L2_REPO" "L3:$L3_REPO"; do
    _level="${_pair%%:*}"
    _repo="${_pair#*:}"
    [ -n "$_repo" ] || continue
    _rules_src="$CONTEXTUS_DIR/$_repo/rules"
    [ -d "$_rules_src" ] || continue

    # サブディレクトリごと削除して再コピー（stale 除去）
    rm -rf ".claude/rules/$_repo"
    cp -r "$_rules_src" ".claude/rules/$_repo"
    echo ":: rules: $_level → .claude/rules/$_repo/" >&2
done

# hooks を実行可能に
find .claude/hooks -name '*.sh' -exec chmod +x {} \;

# statusline を ~/.claude/ にも配置
cp .claude/statusline.sh "${HOME}/.claude/statusline.sh" 2>/dev/null \
    || echo "  (skip: ~/.claude/ is read-only)" >&2

echo ":: 完了" >&2
