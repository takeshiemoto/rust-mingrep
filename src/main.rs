extern crate rust_mingrep;
use rust_mingrep::Config;
use std::{env, process};

// lib.rs -> ライブラリクレート
// main.rs -> バイナリクレート
// libからmainに持ってくるのにextern crateを宣言する。

fn main() {
    let args: Vec<String> = env::args().collect();
    // unwrap_or_elseはOkならOkの値を、Errなら独自のエラー処理を定義できる。
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        // process::exitはpanic!と違い余計な出力がない。
        process::exit(1);
    });

    // if letでErr値を返したかを確認する
    // Ok値が不要なのでunwrapした値は不要。
    if let Err(e) = rust_mingrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
