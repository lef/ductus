# Security — Secrets and Credentials

## Never Embed Credentials in Code or Config

- Do not hardcode tokens, passwords, or API keys in any file
- Do not embed credentials in git remote URLs (e.g. `https://user:TOKEN@github.com/...`)
- Use credential helpers instead (e.g. `gh auth setup-git` for GitHub)

## Never Commit Secrets to Git

- Never commit `.env` files, credential files, or files containing tokens
- Add secret-containing files to `.gitignore` before creating them
- If you accidentally commit a secret: **rotate it immediately**, then remove from history

## Where Secrets Belong

- Environment variables: `export MY_TOKEN=...` (shell session only)
- Credential helpers: managed by the tool (e.g. `gh`, `docker login`)
- Secret managers: for production environments

## If You Find a Secret in a Repository

1. Rotate the credential immediately — assume it is compromised
2. Remove it from git history (`git filter-repo` or GitHub's secret scanning)
3. Audit access logs if possible
