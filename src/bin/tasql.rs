use anyhow::Result;
use taurus_db::Repl;

#[tokio::main]
async fn main() -> Result<()> {
    let mut repl = Repl::new().with_name("TaurusDB").with_version("0.1.0");

    repl.run_loop().await
}
