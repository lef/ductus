# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-18 (session 4)
**Previous Work**: FG-PAT push 成功 + gh 永続化問題の調査

## Current State

### Completed (sessions 2-4)

- **SIGHUP race condition 修正**: signal 登録を `tokio::spawn` の外に移動
- **pidfile デフォルト化**: `/tmp/ductus.pid`、`--no-pidfile` で無効化
- **`--bind` オプション**: デフォルト `127.0.0.1`（安全側）
- **`.example.com` dot-domain 記法**: root + 全サブドメイン（Squid/Nginx 慣習）
- **Sandbox Credential Architecture 設計**: Option D Hybrid（設計文書記録済み）
- **FG-PAT setup script**: `ductus-gh-setup.sh`（tutus 側、push 済み）
- **sandbox 内から git push 成功**: FG-PAT + `~/bin/gh` + `GIT_CONFIG_GLOBAL` override
- **全コミット push 済み**: ductus + tutus 両方

### Not Started (priority order)

1. **ductus-allow ファイルパス不一致（HIGH）** — tutus の ductus-session.sh が mktemp でランダム名の pidfile/session-allowlist を作るため、ductus-allow がデフォルトパスで見つけられない。複数インスタンス対応は正しいが、discovery 機構がない。案: `/tmp/ductus/<project-slug>/` にまとめる。主に tutus 側の修正（ductus 本体は --pidfile を受け取るだけで変更不要）
2. **`--blacklist`** — 永久ブロックリスト（allowlist にあっても拒否）
2. **`--audit-log`** — 全 CONNECT リクエストを記録
3. **SELinux/AppArmor credential isolation** — Phase 2（将来）

## Next Session: Push Setup

sandbox 内から push するには毎セッション以下が必要（gh バイナリが tmpfs で消えるため）:
```bash
# gh 再インストール（~/bin/ は PERSISTENT_HOME 上で永続）
curl -sL "https://github.com/cli/cli/releases/download/v2.88.1/gh_2.88.1_linux_arm64.tar.gz" \
  | tar xz -C /tmp && cp /tmp/gh_2.88.1_linux_arm64/bin/gh ~/bin/gh

# gitconfig override（.gitconfig は bind-ro で /usr/bin/gh を参照）
cp ~/.gitconfig /tmp/.gitconfig-sandbox
sed -i "s|!/usr/bin/gh|!$HOME/bin/gh|g" /tmp/.gitconfig-sandbox
export GIT_CONFIG_GLOBAL=/tmp/.gitconfig-sandbox
export PATH="$HOME/bin:$PATH"
```

恒久解決: tutus 側で rootfs に GitHub apt repo を bake → `apt install gh` → `/usr/bin/gh`

## Key Decisions

- 設計文書: `.claude/plans/gleaming-zooming-bengio.md`
- KNOWLEDGE.md: credential architecture、SIGHUP race、wildcard サーベイ

## FG-PAT Token Note

FG-PAT token が会話ログに一度露出（`cat hosts.yml`）。**revoke して再発行すべき**。

## Blockers

- **gh バイナリ揮発**: `~/.local/bin/` が tmpfs。暫定策 `~/bin/`。恒久策は tutus 側 rootfs bake
- **FG-PAT token 再発行必要**: 会話ログ露出のため
