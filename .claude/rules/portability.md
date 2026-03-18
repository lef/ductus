# Portability Rules

## ハードコードされたパスの禁止（最重要）

スクリプト・スキル・プロンプト文書を問わず、絶対パスをハードコードしない。

**禁止**:
```
/home/lef/.claude/projects/-home-lef-repos-tutus/memory/
/home/lef/repos/tutus/
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
`/home/lef/` は `/home/sandbox/` になる。ユーザー名・プロジェクト名も環境依存。

## ユーザー名・マシン名のハードコード禁止

- `lef`, `<MACHINE-NAME>` 等をコードや文書に埋め込まない
- ユーザー情報は `$(whoami)`, `$USER`, `$HOME` で取得する
