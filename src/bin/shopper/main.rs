use std::env;

use anyhow::Result;

mod argparse;

pub(crate) fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tokio::main]
async fn main() -> Result<()> {
    teloxide::enable_logging!();

    let args: Vec<String> = env::args().collect();
    main_fn(args).await
}

async fn main_fn(args: Vec<String>) -> Result<()> {
    let cmd = argparse::flags_from_vec(args)?;
    cmd.run().await?;
    Ok(())
}
