# Spec-Driven Development — Enforcement Rules # 仕様駆動開発 — 強制ルール

> Extends L0 `sdd.md` (core SDD rules, CONSTITUTION concept, new-repo rule).
> This file (L2) adds the development-specific workflow and enforcement.
> L0 `sdd.md`（SDD コアルール、CONSTITUTION 概念）を拡張する。
> このファイル（L2）は開発固有のワークフローと強制ルールを追加する。

## The Rule # 原則

**Do not write implementation code until `.spec/TODO.md` is confirmed by the human.**
**`.spec/TODO.md` が人間に承認されるまで実装コードを書いてはならない。**

This is non-negotiable. The SDD workflow exists to prevent wasted implementation effort.
これは交渉の余地がない。SDD は実装の手戻りを防ぐために存在する。

## Workflow # ワークフロー

```
0. Confirm CONSTITUTION.md exists (create if needed, see L0 sdd.md)
   CONSTITUTION.md の存在を確認する（なければ作成、L0 sdd.md 参照）
         ↓
1. Human writes .spec/PLAN.md  (rough intent, free-form)
   人間が .spec/PLAN.md を書く（大まかな意図、自由形式）
         ↓
2. Agent asks clarifying questions
   エージェントが明確化の質問をする
         ↓
3. Agent derives design decisions from principles (see below)
   エージェントが原則から設計判断を導出する（下記参照）
         ↓
4. Agent writes .spec/SPEC.md (what & why) and optionally .spec/DESIGN-*.md (how)
   エージェントが .spec/SPEC.md（何を・なぜ）を書く。必要なら .spec/DESIGN-*.md（どうやって）も
         ↓
5. Human confirms: "OK" / "go ahead"
   人間が承認する
         ↓
6. Agent writes .spec/TODO.md  (checkbox task list, with acceptance criteria)
   エージェントが .spec/TODO.md を書く（チェックボックス + 受け入れ基準）
         ↓
7. Consistency check: SPEC ↔ TODO ↔ CONSTITUTION
   整合性チェック: SPEC ↔ TODO ↔ CONSTITUTION の矛盾がないか確認
         ↓
8. Human confirms task list
   人間がタスクリストを承認する
         ↓
   ── TDD starts here (see tdd.md, anti-patterns.md) ──
   ── ここから TDD（tdd.md, anti-patterns.md 参照）──
         ↓
9.  Write tests first (RED — confirm test fails)
    テストを先に書く（RED — テストが失敗することを確認）
         ↓
10. Minimal implementation (GREEN — smallest code to pass)
    最小限の実装（GREEN — テストが通る最小のコード）
         ↓
11. Refactor (keep GREEN while improving)
    リファクタリング（GREEN を維持しながら改善）
         ↓
    ── Verify & Commit (see git-workflow.md checklist) ──
    ── 検証してコミット（git-workflow.md チェックリスト参照）──
         ↓
12. Discoveries → .spec/KNOWLEDGE.md
    発見事項を記録する
```

## Deriving from Principles (Step 3) # 原則からの導出

Before marking something as undecided, ask: **can the project's principles decide this?**
未確定とする前に問う: **プロジェクトの原則がこれを決められないか？**

Check in order of authority (lower layer = higher authority):
権威の順にチェックする（下位レイヤー = より高い権威）:

| Layer | Source | Example |
|---|---|---|
| L0 | AGENTS.md, contextus rules | Agent-agnostic foundations / エージェント非依存の基盤 |
| L1 | Agent-specific rules | Claude Code hooks, permissions / エージェント固有の制約 |
| L2 | Development principles | KISS, YAGNI, SDD, TDD / 開発原則 |
| Project | CONSTITUTION.md | Project-specific absolute constraints / プロジェクト固有の絶対制約 |

Only what remains undecided after this derivation is a true `[NEEDS CLARIFICATION]`.
この導出の後もなお未確定なものだけが本当の `[NEEDS CLARIFICATION]` である。

## When to Stop and Write SPEC First # SPEC を先に書くべきとき

If a request involves:
以下のいずれかに該当する場合:

- A new feature (not a trivial one-liner fix) / 新機能（些末な 1 行修正ではない）
- A refactor affecting multiple files / 複数ファイルにまたがるリファクタリング
- A design decision with non-obvious tradeoffs / 自明でないトレードオフを伴う設計判断
- Anything that will take more than ~30 minutes / 30 分以上かかりそうな作業

→ Stop. Ask if there is a `.spec/PLAN.md`. If not, offer to create it together.
→ 止まる。`.spec/PLAN.md` があるか確認し、なければ一緒に作る。

## SPEC vs DESIGN # SPEC と DESIGN の分離

SPEC.md answers **what** and **why**. DESIGN.md answers **how**.
SPEC.md は「何を」「なぜ」に答える。DESIGN.md は「どうやって」に答える。

- **SPEC.md**: requirements, constraints, acceptance criteria, risk assessment
  要件、制約、受け入れ基準、リスク評価
- **DESIGN-\*.md**: architecture, implementation approach, design discussion records
  アーキテクチャ、実装方針、設計議論の記録

When to split / 分離するタイミング:
- Simple features → SPEC.md alone is enough / シンプルなら SPEC.md だけで十分
- Complex design with tradeoffs, rejected alternatives, or extended discussion
  → write DESIGN-{topic}.md separately
  トレードオフ、却下案、長い議論を伴う設計 → DESIGN-{topic}.md に分離

DESIGN files preserve the **reasoning process**, not just the conclusion.
DESIGN ファイルは結論だけでなく**思考過程**を保存する。

## [NEEDS CLARIFICATION] Marker # 未確定マーカー

Mark underspecified or ambiguous areas in SPEC.md with `[NEEDS CLARIFICATION]`.
SPEC.md 内の未確定・曖昧な箇所に `[NEEDS CLARIFICATION]` を付ける。

```markdown
## Data Model
- Users can have multiple projects
- Permission model [NEEDS CLARIFICATION: RBAC vs ACL undecided]
```

- Maximum 3 per SPEC. If more than 3, resolve with the human before proceeding
  1 つの SPEC に最大 3 つ。超えたら人間と解消してから先に進む
- **Resolve ALL markers before asking the human to confirm the SPEC (step 5)**
  **SPEC の承認を求める前（ステップ 5）に全マーカーを解消すること**
- Do not advance to TODO.md while any `[NEEDS CLARIFICATION]` remains
  マーカーが残ったまま TODO.md に進まない
- When resolved, remove the marker and record the decision rationale in KNOWLEDGE.md
  解消したらマーカーを削除し、決定理由を KNOWLEDGE.md に記録

## Ambiguity Scan # 曖昧性スキャン

After writing SPEC.md, scan for ambiguous or underspecified areas before proceeding.
Mark anything found with `[NEEDS CLARIFICATION]`.

SPEC.md を書いた後、曖昧・未定義な箇所がないかスキャンしてから先に進む。
見つけたものに `[NEEDS CLARIFICATION]` を付ける。

For a structured taxonomy of what to check, see the `clarify` command in
[github/spec-kit](https://github.com/github/spec-kit) (`templates/commands/clarify.md`).
チェック項目の構造化された分類は上記 Spec Kit の clarify コマンドを参照。

## Consistency Check (Step 7) # 整合性チェック

Before the human confirms the task list, verify:
人間がタスクリストを承認する前に確認する:

- Every TODO item traces back to a SPEC requirement / 全 TODO が SPEC の要件に対応している
- No SPEC requirement is missing from TODO / SPEC の要件が TODO から漏れていない
- No TODO item contradicts CONSTITUTION / TODO が CONSTITUTION に矛盾していない
- Acceptance criteria in TODO are testable / TODO の受け入れ基準がテスト可能である

If inconsistencies are found, fix them before asking for confirmation.
不整合が見つかったら、承認を求める前に修正する。

## Plan Mode Integration # Plan Mode との連携

If the agent supports a plan mode (e.g., Claude Code `/plan`):
エージェントが plan mode をサポートする場合（例: Claude Code `/plan`）:

- **Enter plan mode** at step 0–8 (CONSTITUTION → PLAN → SPEC → TODO).
  Plan mode is the natural fit for specification work — no implementation tools needed.
  ステップ 0〜8 で plan mode に入る。仕様作業に最適 — 実装ツール不要。
- **Exit plan mode** at step 9 (TDD starts). Implementation requires file
  editing and test execution.
  ステップ 9 で plan mode を抜ける。実装にはファイル編集とテスト実行が必要。
- Plan mode is optional. The SDD workflow applies regardless of mode.
  Plan mode は任意。SDD ワークフローはモードに関係なく適用される。

## KNOWLEDGE.md — Continuous Recording # 継続的な記録

Record in `.spec/KNOWLEDGE.md` during implementation:
実装中に `.spec/KNOWLEDGE.md` に記録する:

- Why a particular approach was chosen over alternatives / なぜその方法を選んだか
- Non-obvious constraints discovered during implementation / 実装中に発見した非自明な制約
- Rejected approaches and why they were rejected / 却下した方法とその理由

## Skip Rules # スキップルール

SDD can be skipped, but **skipping SDD does not skip TDD**.
SDD は飛ばせるが、**SDD を飛ばしても TDD は飛ばせない**。

```
SDD (SPEC → TODO)
  ↓ if trivial → must confirm with human before skipping
    些末なら → 人間に確認してから skip
TDD (test first → implement)
  ↓ bug fix / one-liner → may skip
    バグ修正・1 行修正なら skip 可
Implementation
```

**Important**: Do not judge "trivial" on your own. Always confirm with the human:
**重要**: 些末かどうかを自分で判断しない。必ず人間に確認する:

> "This seems trivial enough to skip SDD and start with TDD. OK?"
> 「これは trivial なので SDD を飛ばして TDD から始めてよいですか？」

## Exceptions (explicit override only) # 例外（明示的な override のみ）

- Human explicitly says "skip SPEC" / "just implement" → skip SDD only
  人間が明示的に「SPEC 飛ばして」「直接実装して」→ SDD のみ省略
- Human explicitly says "no tests needed" / "just do it" → skip TDD only
  人間が明示的に「テスト不要」「そのままやって」→ TDD のみ省略
- Bug fix with clear cause, contained in one file → skip SDD + TDD, fix directly
  原因が明確で 1 ファイル内に収まるバグ修正 → SDD + TDD 省略、直接修正可
