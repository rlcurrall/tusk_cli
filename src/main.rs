use anyhow::{anyhow, Result};
use clap::{load_yaml, App};
use std::fs;
use std::option::Option;
use tusk_lexer::Lexer;
use tusk_parser::Parser;

fn output_file_ast(filepath: &str, format: Option<&str>, out: Option<&str>) -> Result<()> {
    let contents = fs::read_to_string(filepath)?;
    let lexer = Lexer::new(contents.as_str());
    let mut parser = Parser::new(lexer);

    let program = parser.all().unwrap();

    let ast_str = match format {
        None => format!("{:#?}", program),
        Some("json") => serde_json::to_string_pretty(&program)?,
        Some(_) => return Err(anyhow!("Unsupported format.")),
    };

    match out {
        Some(path) => fs::write(path, ast_str)?,
        None => println!("{}", ast_str),
    };

    Ok(())
}

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("ast", Some(ast_matches)) => {
            let filepath = ast_matches.value_of("file").unwrap();
            let format = ast_matches.value_of("format");
            let out = ast_matches.value_of("out");

            output_file_ast(filepath, format, out)
        }
        _ => Err(anyhow!("Unimplemented!")),
    }
}
