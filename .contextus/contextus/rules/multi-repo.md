# Multi-Repository Work Discipline

> L0: applies to all agents and domains.

## 作業対象を常に意識する

複数リポジトリを横断して作業するとき、**今どのリポジトリに対して操作しているか**を常に意識する。

- git コマンドは `-C <path>` で対象を明示する
- ファイル操作は絶対パスで行う
- 「さっき別のリポジトリに cd した」状態でコマンドを打たない

## 操作と構築を分ける

- **操作**（git, テスト実行, 確認）と**構築**（ファイル編集, 新規作成）を混ぜない
- 1つのリポジトリでの作業を完結させてからコミット、次のリポジトリへ移る
- 「ついでに隣のリポジトリも直しておこう」は事故の元

## upstream remote を追加したら gh-resolved を設定する

`gh` CLI は `upstream` remote があると fork 元と解釈して PR 対象を誤る。

```bash
git remote add upstream <url>
git config remote.origin.gh-resolved "base"  # 必ずセットで実行
```

## upstream 反映の確認

あるリポジトリ（例: contextus-claude）を変更したら、それに依存するリポジトリ（例: tutus）への反映を忘れない。

チェックリスト:
1. upstream リポジトリを変更・コミット・push
2. 依存リポジトリで bootstrap（`make bootstrap-claude --force` 等）
3. bootstrap 結果をコミット・push
4. **変更が実際に反映されたか grep 等で確認**
