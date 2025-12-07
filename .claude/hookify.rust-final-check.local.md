---
name: rust-final-check
enabled: true
event: stop
action: warn
pattern: .*
---

🦀 **Rust プロジェクト - 作業終了前チェックリスト**

作業を終了する前に、以下を確認してください：

**必須チェック項目:**
- [ ] `cargo fmt` を実行しましたか？
- [ ] `cargo clippy` を実行し、警告に対応しましたか？
- [ ] `cargo check` を実行し、コンパイルエラーがないことを確認しましたか？

**推奨チェック項目:**
- [ ] 変更した機能に対するテストを追加・更新しましたか？
- [ ] コードレビューに必要なコメントを追加しましたか？

すべてのチェックが完了していない場合は、完了してから作業を終了してください。

**一括実行コマンド:**
```bash
cargo fmt && cargo clippy -- -D warnings && cargo check
```
