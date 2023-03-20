use anyhow::{Ok, Result};
use console::{style, Term};
use std::io::Write;

pub struct Repl {
    name: String,
    version: String,
    prompt: String,
    console: Term,
}

impl Repl {
    pub fn new() -> Repl {
        Self {
            name: String::new(),
            version: String::new(),
            prompt: String::from("> "),
            console: Term::stdout(),
        }
    }

    /// Give repl a name
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Give repl a version
    pub fn with_version(mut self, version: &str) -> Self {
        self.version = version.to_string();
        self
    }

    pub fn with_prompt(mut self, prompt: &str) -> Self {
        self.prompt = prompt.to_string();
        self
    }

    async fn execute(&self, input: &str) -> Result<()> {
        if input.starts_with('!') {
            return self.execute_command(input).await;
        }

        println!("Unknown command: {}", input);
        Ok(())
    }

    /// execute meta command
    async fn execute_command(&self, command: &str) -> Result<()> {
        match command {
            "!help" => {
                println!("TODO ~");
            }
            "!exit" => {
                println!("Bye ~");
                std::process::exit(0);
            }
            _ => {
                println!("Unrecognized command: {}", command);
            }
        }
        Ok(())
    }

    /// execute sql statement
    async fn execute_statement(&self, statement: &Statement) -> Result<()> {
        match statement.statement_type {
            StatementType::Insert => {
                println!("This is where we would do an insert.");
            }
            StatementType::Select => {
                println!("This is where we would do a select.");
            }
        }
        Ok(())
    }

    pub async fn run_loop(&mut self) -> Result<()> {
        writeln!(
            &self.console,
            "{} version {}",
            style(self.name.as_str()).bold(),
            style(self.version.as_str()).red()
        )?;
        writeln!(
            &self.console,
            "Connected to {}. Enter {} for help and {} for exit.\n",
            self.name.as_str(),
            style("!help").cyan(),
            style("!exit").cyan()
        )?;

        loop {
            // print prompt
            write!(&self.console, "{}", &self.prompt)?;

            let input = self.console.read_line()?;
            self.execute(&input).await?;
        }
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}

enum PrepareResult {
    Success,
    UnrecognizedStatement,
}

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
}
