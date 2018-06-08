use engine_operator;
use query_parser;
use std::cell::RefCell;
use std::fs::File;
use std::io::{self, prelude::*, Write};

enum ReplCommand {
    Quit,
    Help,
}

enum Command {
    ReplCommand(ReplCommand),
    DBCommand(String),
}

enum ReplResponseAction {
    Continue,
    Finish,
}

#[derive(Debug, Default)]
pub struct Repl {
    query_parser: query_parser::QueryParser,
    engine_operator: RefCell<engine_operator::EngineOperator>,
}

impl Repl {
    pub fn new() -> Repl {
        info!("REPL has been initialized");
        Default::default()
    }

    pub fn start(&self) {
        info!("REPL is listening");

        print!("> ");
        let _ = io::stdout().flush();

        let mut command = String::new();
        let read_res = io::stdin().read_line(&mut command);

        match read_res {
            Ok(n) => {
                info!("{} byes are read", n);
                match self.execute_raw_command(&command) {
                    ReplResponseAction::Finish => return,
                    _ => {}
                }
            }
            Err(e) => {
                error!("Read error: {}", e);
                return;
            }
        }

        self.start();
    }

    pub fn read_file(&self, file_name: &String) -> Result<(), io::Error> {
        let mut f = File::open(file_name)?;
        let mut buffer = String::new();

        f.read_to_string(&mut buffer)?;

        buffer
            .split('\n')
            .map(|l| l.trim())
            .filter(|l| l.len() > 0)
            .for_each(|l| {
                // @TODO Response is ignored - expected not to terminate from file.
                self.execute_raw_command(&l.to_owned());
                ()
            });

        Ok(())
    }

    fn execute_raw_command(&self, command: &String) -> ReplResponseAction {
        match parse_command(&command) {
            Ok(Command::ReplCommand(repl_command)) => match repl_command {
                ReplCommand::Quit => {
                    println!("Bye!");
                    return ReplResponseAction::Finish;
                }
                ReplCommand::Help => {
                    print_help();
                }
            },
            Ok(Command::DBCommand(db_command)) => {
                let query = self.query_parser.parse(&db_command);
                info!("Got DB Query: {:#?}", query);

                self.engine_operator.borrow_mut().execute(query.unwrap());
            }
            Err(_) => {
                info!("Command [{:#?}] not known. Try again.", command);
            }
        }

        ReplResponseAction::Continue
    }
}

fn parse_command(command: &String) -> Result<Command, ()> {
    let slice: &str = &command[..];

    match &slice.trim().to_lowercase()[..] {
        "q" | "quit" | "exit" => {
            return Ok(Command::ReplCommand(ReplCommand::Quit));
        }
        "h" | "?" | "help" => {
            return Ok(Command::ReplCommand(ReplCommand::Help));
        }
        _ => {}
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
    println!("\tCreate table: + TABLENAME (FIELDNAME TYPE)+");
    println!("\tSelect query: ? (FIELD_NAME)+ > TABLENAME (: (FIELD_NAME OP VALUE)+)");
    println!("\tInsert query: > TABLENAME (FIELD_NAME VALUE)+");
    println!("\tDescribe database: :db");
}
