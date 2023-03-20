use anyhow::Result;
use summer::Repl;

#[tokio::main]
async fn main() -> Result<()> {
    let mut repl = Repl::new()
        .with_name("SummerDB")
        .with_version("0.1.0")
        .with_prompt("db > ");

    repl.run_loop().await
}
