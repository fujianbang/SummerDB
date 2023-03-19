use anyhow::Result;
use console::{style, Term};
use std::io::Write;

#[derive(Debug)]
pub struct Repl {
    name: String,
    version: String,
    console: Term,
}

impl Repl {
    pub fn new() -> Repl {
        Self {
            name: String::new(),
            version: String::new(),
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

    async fn execute(&self, input: &str) -> Result<()> {
        if input == "?" || input == "help" {
            println!(
                r#"
    TODO
"#
            );
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
            "Connected to {}. Enter {} for help.",
            self.name.as_str(),
            style("? or help").cyan()
        )?;

        loop {
            write!(&self.console, "> ")?;
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