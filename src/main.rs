mod tokenizer;

use std::ops::Index;
use std::io;
use std::env;

use tokenizer::{fireflytokenizer, token};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("Usage: firefly [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(args.index(0));
    } else {
        run_prompt();
    }
}

// run file when one file is provided
fn run_file(path: &String) {
    run(path.to_string()) // how does this work?
}

// interactive prompt analogous to a "REPL" when no script is provided
fn run_prompt() ->  io::Result<()> {
    println!(">>> START interactive prompt session <<<");

    // REPL (read-eval-print loop) over user code
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        if buffer == "exit\n" || buffer == "Exit\n" || buffer == "EXIT\n" || buffer == "\n" {
            break;
        } else {
            println!("running code");
            run(buffer);
        }
    }

    println!(">>> HALT interactive prompt session <<<");
    Ok(())
}

fn run(source: String) {
    let tokenizer: fireflytokenizer::Tokenizer = fireflytokenizer::Tokenizer::new(source);
    // For now, just print the tokens
    for token in tokenizer {
        println!("{:?}", token);
    }
}

fn error(line: u16, message: String) {
    report(line, String::from(""), message);
}

fn report(line: u16, position: String, message: String) {
    eprint!("[line {}] {}: {}", line, position, message);
    // consequence ???
    todo!(how does the rest of the program (main) see the error here without me
          passing a parameter in?)
}
