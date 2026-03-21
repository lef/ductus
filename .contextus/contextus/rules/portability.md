# Portability Rules

> L0: applies to all agents and domains.

## ハードコードされたパスの禁止（最重要）

スクリプト・スキル・プロンプト文書を問わず、絶対パスをハードコードしない。
**ドキュメントの例示でも実際の環境情報を使わない（下記参照）。**

**禁止**:
```
/home/<user>/.claude/projects/-home-<user>-repos-<project>/memory/
/home/<user>/repos/<project>/
```

**正しい書き方**:
```bash
# シェルスクリプト
~/.claude/projects/$(pwd | sed 's|/|-|g')/memory/
"$(cd "$(dirname "$0")" && pwd)"   # スクリプト自身のディレクトリ
"$HOME/.claude/..."                # ホームディレクトリ

# Claude スキル / プロンプト文書
~/.claude/projects/$(pwd | sed 's|/|-|g')/memory/
$CLAUDE_PROJECT_DIR
```

**理由**: sandbox 内ではホストのパスが存在しない。
ユーザー名・プロジェクト名・ディレクトリ構造は環境依存。

## 環境依存の symlink を git で追跡しない

環境依存の絶対パスを含む symlink は git で追跡しない。
bootstrap や setup スクリプトで環境から自動導出して生成する。

**禁止**: `.contextus/<repo> → /home/<user>/repos/<repo>`（git に commit）
**正しい**: `.gitignore` に追加し、bootstrap で PROJECT_DIR の sibling から自動検出

**理由**: sandbox と host でパスが異なる。git 追跡された symlink は
一方の環境で commit すると他方で壊れる。

## ユーザー名・マシン名のハードコード禁止

- 実際のユーザー名やマシン名をコードや文書に埋め込まない
- ドキュメントの例示には `<user>`, `<project>`, `<host>` 等のプレースホルダを使う
- ユーザー情報は `$(whoami)`, `$USER`, `$HOME` で取得する

## ドキュメント・例示での環境情報漏洩禁止

> L0 `agent-security.md` §Environment Information Leakage Prevention を参照。
> ルールファイル・設計文書の例示にも実際の環境情報を含めない。
