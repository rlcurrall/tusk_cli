use clap::{load_yaml, App};
use std::fs;
use tusk_lexer::Lexer;
use tusk_parser::Parser;

fn output_file_ast(filepath: &str, format: &str, out: &str) {
    match fs::read_to_string(filepath) {
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
        Ok(contents) => {
            let lexer = Lexer::new(contents.as_str());
            let mut parser = Parser::new(lexer);

            match parser.all() {
                Err(error) => {
                    eprintln!("Error: {}", error);
                    std::process::exit(1);
                }
                Ok(program) => {
                    let ast_str = match format {
                        "json" => serde_json::to_string_pretty(&program).unwrap(),
                        "debug" => format!("{:#?}", program),
                        _ => {
                            eprintln!("Unsupported format.");
                            std::process::exit(1)
                        }
                    };

                    match out {
                        "stdout" => println!("{}", ast_str),
                        _ => fs::write(out, ast_str).unwrap(),
                    }
                }
            }
        }
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("ast", Some(ast_matches)) => {
            let filepath = ast_matches.value_of("file").unwrap();
            let format = ast_matches.value_of("format").unwrap();
            let out = ast_matches.value_of("out").unwrap();

            output_file_ast(filepath, format, out);
        }
        _ => unimplemented!(),
    }
}
