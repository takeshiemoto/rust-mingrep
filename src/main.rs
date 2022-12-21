use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    // unwrap_or_elseはOkならOkの値を、Errなら独自のエラー処理を定義できる。
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        // process::exitはpanic!と違い余計な出力がない。
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // if letでErr値を返したかを確認する
    // Ok値が不要なのでunwrapした値は不要。
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // &'static strは文字列リテラルの型
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

// ()はユニット型。空を表す。
// dynはdynamicの略。エラーケースによって異なる型のError値を返すことができる。
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ?演算子は呼び出し元にエラーを委譲する処理の省略記法。
    let mut f = File::open(config.filename)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    println!("With text:\n{}", content);

    Ok(())
}
