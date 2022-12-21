use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // &'static strは文字列リテラルの型
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // ?演算子は呼び出し元にエラーを委譲する処理の省略記法。
    let mut f = File::open(config.filename)?;

    let mut content = String::new();
    f.read_to_string(&mut content)?;

    println!("With text:\n{}", content);

    Ok(())
}
