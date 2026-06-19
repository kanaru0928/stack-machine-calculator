# 簡易電卓言語の実装

以下を実装した。

- スタックマシンの命令セットとそれに対する評価器
- 簡易電卓言語からスタックマシンの命令へのコンパイラ

## 実行

```sh
cargo run -- program1.txt
```

## テスト

```sh
cargo test
```

## スタックマシン

[stackmachine](./src/stackmachine/) モジュールに実装した。スタックマシンの命令セットは [ops.rs](./src/stackmachine/ops.rs)、評価器は [eval.rs](./src/stackmachine/eval.rs) に実装している。

以下のように使用する。
```rs
let instructions: Vec<Op> = ...;
let mut evaluator = Evaluator::new();
evaluator.evaluation(instructions);
```

## パーザ

パーザジェネレータである [pest](https://pest.rs) を使用した。PEG は [calc.pest](./calc.pest) の通り。

## コード生成

コード生成は [trans.rs](./src/calc/trans.rs) で実装している。
