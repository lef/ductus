# HANDOFF — Session Transition Notes

**Last Updated**: 2026-03-18 (session 3)
**Previous Work**: Credential architecture design + FG-PAT setup script

## Current State

### Completed (session 2 + 3)

- **SIGHUP race condition 修正**: signal 登録を `tokio::spawn` の外に移動
- **pidfile デフォルト化**: `/tmp/ductus.pid`、`--no-pidfile` で無効化
- **`--bind` オプション**: デフォルト `127.0.0.1`（安全側）
- **`.example.com` dot-domain 記法**: root + 全サブドメイン（Squid/Nginx 慣習）
- **Sandbox Credential Architecture 設計**: Option D Hybrid
- **FG-PAT setup script**: `ductus-gh-setup.sh` + `claude-sandbox.sh` GH auth mount（tutus 側）
- **gh CLI インストール**: sandbox 内に `~/.local/bin/gh` 配置済み
- **全 8 コミット push 済み**: ductus リポジトリ

### Next Session: Host-Side Actions Required

```bash
# 1. tutus を push（FG-PAT setup script + claude-sandbox.sh 修正）
cd ~/repos/tutus && git push

# 2. FG-PAT セットアップ
make gh-setup P=~/repos/ductus REPOS="lef/ductus lef/tutus lef/contextus-claude"

# 3. sandbox 再起動（GH auth が自動注入される）
make claude-ductus P=~/repos/ductus REPOS=~/repos
```

sandbox 再起動後:
- `gh auth status` で認証確認
- `git push` が透過的に動作するはず

### Not Started (priority order)

1. **sandbox 内 git push 動作確認**: FG-PAT + bind-ro injection の E2E テスト
2. **tutus `ductus-session.sh` 更新**: `--bind`, `--no-pidfile`, `--port 0` 対応
3. **`--blacklist`** — 永久ブロックリスト
4. **`--audit-log`** — 全 CONNECT リクエストを記録
5. **SELinux/AppArmor credential isolation**: 全 agent binary の credential process restriction
6. **Socket 攻撃事例の調査**: CVE/具体事例を KNOWLEDGE.md に補完

## Key Decisions Made (this session)

- **sandbox 内から push が必要**: sandbox は死ぬ + 大量生産 + コンテキストは中にしかない
- **Socket forwarding 却下**: SSH agent/Docker socket hijacking が既知の攻撃ベクター
- **FG-PAT で blast radius 最小化**: repo 限定 + 短期限 + 個別 revoke
- **Option D Hybrid**: Phase 1 = FG-PAT + bind-ro injection、Phase 2 = SELinux/AppArmor
- **Agent credential と GH credential は構造的に同じ問題**: Agent cred は既に sandbox 内にある
- **設計文書**: `.claude/plans/gleaming-zooming-bengio.md` に詳細

## Blockers / Watch Out For

- **tutus 未 push**: FG-PAT script + claude-sandbox.sh 修正がまだリモートにない
- **gh binary の配置**: sandbox 内に `~/.local/bin/gh` があるが、persistent overlay でなければ次回消える
- **.gitconfig が bind-ro**: `/usr/bin/gh` を参照。sandbox 内の `~/.local/bin/gh` と不一致。`GIT_CONFIG_GLOBAL` override が必要かも
- **contextus-dev-rust**: 1 commit 未 push（Model Switching ルール更新）

## Changed Files (this session)

### ductus (this repo)
- `.spec/KNOWLEDGE.md`: credential architecture design, socket 却下理由
- `.spec/TODO.md`: credential architecture セクション追加
- `HANDOFF.md`: 更新

### tutus (~/repos/tutus/)
- `scripts/ductus-gh-setup.sh`: NEW — per-project FG-PAT setup
- `scripts/claude-sandbox.sh`: GH config bind-ro mount 追加
- `Makefile`: `gh-setup` ターゲット追加
