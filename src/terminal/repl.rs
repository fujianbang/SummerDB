use anyhow::{anyhow, Result};
use console::{style, Term};
use std::io::Write;

use super::{
    retrive_insert_sql,
    statement::{Statement, StatementType},
};

pub struct Repl {
    name: String,
    version: String,
    prompt: String,
    console: Term,
}

impl Repl {
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
        if input.is_empty() {
            return Ok(());
        }
        if input.starts_with('!') {
            let meta_commandd_result = self.execute_meta_command(input).await?;
            // TODO do something
            return match meta_commandd_result {
                MetaCommandResult::Success => Ok(()),
                MetaCommandResult::UnrecognizedCommand => Ok(()),
            };
        }

        let statement_result = self.prepare_statement(input).await;
        match statement_result {
            Ok(statement) => {
                // execute statement
                self.execute_statement(&statement).await?;
                Ok(())
            }
            Err(e) => {
                println!("Error: {}", e);
                Ok(())
            }
        }
    }

    /// execute meta command
    async fn execute_meta_command(&self, command: &str) -> Result<MetaCommandResult> {
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
                return Ok(MetaCommandResult::UnrecognizedCommand);
            }
        }
        Ok(MetaCommandResult::Success)
    }

    async fn prepare_statement(&self, input: &str) -> Result<Statement> {
        if input.starts_with("insert") {
            let statement = retrive_insert_sql(input)?;
            return Ok(statement);
        } else if input.starts_with("select") {
            return Ok(Statement::new(StatementType::Select));
        }

        Err(anyhow!(format!("Unknown statement: {}", input)))
    }

    /// execute sql statement
    async fn execute_statement(&self, statement: &Statement) -> Result<()> {
        match statement.statement_type {
            StatementType::Insert => {
                println!(
                    "executed insert: {:?}",
                    statement.row_to_insert.as_ref().unwrap()
                );
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
        Self {
            name: String::new(),
            version: String::new(),
            prompt: String::from("> "),
            console: Term::stdout(),
        }
    }
}

enum MetaCommandResult {
    Success,
    UnrecognizedCommand,
}
