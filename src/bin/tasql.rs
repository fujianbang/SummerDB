use anyhow::Result;
use taurus_db::Repl;

#[tokio::main]
async fn main() -> Result<()> {
    let mut repl = Repl::new()
        .with_name("TaurusDB")
        .with_version("0.1.0")
        .with_prompt("db > ");

    repl.run_loop().await
}
