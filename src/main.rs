mod core;
mod error;
mod rpc;
mod svm;

use anyhow::Result;
use clap::{Arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("grid")
        // .version("0.1.0") // commented out to get version from manifest
        .about("Grid V2")
        .arg_required_else_help(true)
        .arg(
            Arg::new("RPC")
                .help("Base layer RPC URL")
                .long("rpc-url")
                .short('r'),
        )
        .get_matches();

    Ok(())
}
