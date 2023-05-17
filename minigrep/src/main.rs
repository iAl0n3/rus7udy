use std::env;
// 重构7
// use std::fs;
use std::process;
// use std::error::Error;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    // 重构1
    // let query = &args[1];
    // let file_path = &args[2];

    // 重构2
    //let (query, file_path) = parse_config(&args);

    // 重构3
    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // 重构1
    // println!("Searching for {}", query);
    // println!("In file {}", file_path);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }

    // 重构4
    // 重构6
    // run(config);

    // 重构1
    // let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // 重构4
    // let contents = fs::read_to_string(config.file_path)
    //     .expect("Should have been able to read the file");
    //
    // println!("With text:\n{contents}");
}

// 重构7
// // 重构5
// // fn build(args: &[String]) -> Result<Config, &'static str>
// fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");
//
//     println!("With text:\n{contents}");
//
//     // 重构5
//     Ok(())
// }
//
// struct Config {
//     query: String,
//     file_path: String,
// }
//
// // 重构1
// // fn parse_config(args: &[String]) -> (&str, &str) {
// //     let query = &args[1];
// //     let file_path = &args[2];
// //
// //     (query, file_path)
// // }
//
// // 重构2
// // fn parse_config(args: &[String]) -> Config {
// //     let query = &args[1];
// //     let file_path = &args[2];
// //
// //     Config {
// //         query, file_path
// //     }
// // }
//
// impl Config {
//     // 修改new为build
//     // fn new(args: &[String]) -> Config
//
//     // 重构3
//     // fn build(args: &[String]) -> Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//
//         // feature：改进报错信息
//         if args.len() < 3 {
//             return Err("Not enough arguments");
//         }
//
//         let query = args[1].clone();
//         let file_path = args[2].clone();
//
//         // Config {
//         //     query, file_path
//         // }
//
//         Ok(Config {
//             query, file_path
//         })
//     }
// }