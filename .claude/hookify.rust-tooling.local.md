---
name: rust-tooling-check
enabled: true
event: file
action: warn
conditions:
  - field: file_path
    operator: regex_match
    pattern: \.rs$
---

🦀 **Rustファイルを編集しました**

**必須**: 以下のコマンドを順番に実行してください：

1. **`cargo fmt`** - コードフォーマット
2. **`cargo clippy`** - Lintチェック
3. **`cargo check`** - 型チェック・コンパイルチェック

**重要な注意事項:**
- これらのコマンドは **必ず実行** してください
- エラーや警告が出た場合は、**必ず修正** してから次の作業に進んでください
- フォーマットエラーは `cargo fmt` で自動修正されます
- Clippy の警告は可能な限り対応してください
- `cargo check` でエラーが出る場合は、コンパイルが通るまで修正が必要です

**実行例:**
```bash
cargo fmt && cargo clippy -- -D warnings && cargo check
```

これらのチェックをパスしてから次の作業に進んでください。
