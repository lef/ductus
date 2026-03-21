# Orchestration Principles # オーケストレーション原則

> L0: applies to all projects, regardless of agent or domain.
> L0: エージェントやドメインを問わず全プロジェクトに適用。

## Sub-Agent Context # サブエージェントへの文脈伝達

When delegating to a sub-agent, always pass:
サブエージェントに委譲するとき、必ず渡す:

- What already exists (current state, relevant decisions)
  既にあるもの（現状、関連する決定）
- What must NOT change and why (deliberate constraints)
  変えてはいけないものとその理由（意図的な制約）
- The verification method expected (Structured Flow: Verify step)
  期待する検証方法（Structured Flow の Verify ステップ）

Without this context, sub-agents apply generic best practices that conflict
with project-specific decisions.
この文脈がないと、サブエージェントはプロジェクト固有の決定と矛盾する汎用ベストプラクティスを適用する。

## Oath Limitations in Sub-Agents # サブエージェントにおける誓約の限界

Session-start injection (Oath) does NOT reach sub-agents.
セッション開始時の注入（誓約）はサブエージェントに届かない。

Only tool-use level enforcement (PreToolUse hooks) reaches both main and sub-agents.
ツール使用レベルの強制（PreToolUse hooks）のみが main と sub-agent 両方に届く。

**Design implication**: critical rules must be enforced at tool-use level, not injection level.
**設計上の含意**: 致命的なルールは injection ではなく tool-use level で強制する。

## Agent Communication # エージェント間通信

- Agents communicate via stdout + exit code + git (UNIX philosophy)
  エージェントは stdout + exit code + git で通信する（UNIX 哲学）
- No direct inter-agent channels (OWASP ASI03: internal channel leakage)
  エージェント間の直接通信チャネルは作らない（OWASP ASI03）
- Git is the shared state layer — not context passing
  Git は共有状態層であり、コンテキスト受け渡しではない

## MCP Safety # MCP 安全条件

MCP servers that cross sandbox boundaries are safe when:
sandbox 境界を越える MCP server は以下の条件で安全:

- The server has no access to project code or secrets
  サーバーがプロジェクトコードや秘密情報にアクセスできない
- The server has no push capability (no supply chain attack)
  サーバーに push 能力がない（サプライチェーン攻撃不可）
- The channel is ephemeral (dies with the session)
  チャネルが一時的（セッション終了で破棄）

Security comes from sandboxing the other end, not from restricting the transport.
セキュリティは通信経路ではなく、相手側の sandbox 化で担保する。
