use std::io::{self, Write};

use query_parser;

enum ReplCommand {
  Quit,
  Help,
}

enum Command {
  ReplCommand(ReplCommand),
  DBCommand(String),
}

#[derive(Debug)]
pub struct Repl;

impl Repl {
  pub fn new() -> Repl {
    info!("REPL has been initialized");
    Repl
  }

  pub fn start(&self) {
    info!("REPL is on");

    print!("[DB> ");
    let _ = io::stdout().flush();

    let mut command = String::new();
    let read_res = io::stdin().read_line(&mut command);

    match read_res {
      Ok(n) => {
        info!("{} byes are read", n);

        match parse_command(&command) {
            Ok(Command::ReplCommand(repl_command)) => {
              match repl_command {
                ReplCommand::Quit => {
                  println!("Bye!");
                  return;
                }
                ReplCommand::Help => {
                  print_help();
                }
              }
            },
            Ok(Command::DBCommand(repl_command)) => {
              println!("GOT {:?}", repl_command);
            },
            Err(_) => {
              println!("Command [{:#?}] not known. Try again.", command);
            },
        }
      },
      Err(e) => {
        error!("Read error: {}", e);
        return;
      },
    }

    self.start();
  }
}

fn parse_command(command: &String) -> Result<Command, ()> {
  let slice: &str = &command[..];

  match &slice.trim().to_lowercase()[..] {
    "q" | "quit" => { return Ok(Command::ReplCommand(ReplCommand::Quit)); }
    "h" | "help" => { return Ok(Command::ReplCommand(ReplCommand::Help)); }
    _ => {},
  };

  if query_parser::QueryParser::looks_like_query(command) {
    return Ok(Command::DBCommand(command.clone()));
  }

  Err(())
}

fn print_help() {
  println!("Command list:");
  println!("\tQUIT");
  println!("\tHELP");
}
