use clap::{Parser, Subcommand, Args};
use core::dick::Dick;
use std::string::String;

/// Simple program to greet a person
#[derive(Parser)]
#[clap(version = "0.0.1", about, long_about = None)]
#[clap(author = "Joshua Conero")]
#[clap(name = "pinyin")]
struct Cli {
    /// Name of the person to greet
    #[clap(short, long, value_parser, default_value = "")]
    name: String,

    /// Number of times to greet
    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // 汉字转拼音工具
    Pyin(Pyin)
}

#[derive(Args)]
struct Pyin{
    #[clap(value_parser)]
    value: String,
}


fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Pyin(pyin) => {
            if !pyin.value.is_empty(){
                let dk = Dick::find_by_word(pyin.value.as_str());
                if !dk.is_none(){
                    println!("汉字转怕拼音 {:?}", dk.unwrap().py);
                }else {
                    println!("未找到 {} 拼音 ", pyin.value)
                }
            }

        }
    }
}
