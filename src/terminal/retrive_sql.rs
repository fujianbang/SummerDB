use crate::terminal::statement::{Row, StatementType};

use super::statement::Statement;
use anyhow::{anyhow, Result};
use regex::Regex;

pub fn retrive_insert_sql(sql: &str) -> Result<Statement> {
    let re = Regex::new(r"insert (\d+) (.+) (.+)")?;
    let captures = re
        .captures(sql)
        .ok_or(anyhow!("Invalid insert statement"))?;

    let id = captures
        .get(1)
        .ok_or(anyhow!("Invalid insert statement"))?
        .as_str()
        .parse::<u32>()?;

    let username = captures
        .get(2)
        .ok_or(anyhow!("Invalid insert statement"))?
        .as_str();
    let email = captures
        .get(3)
        .ok_or(anyhow!("Invalid insert statement"))?
        .as_str();

    let row = Row::new(id, username.to_string(), email.to_string());

    let statement = Statement::new(StatementType::Insert).insert_row(row);

    Ok(statement)
}
