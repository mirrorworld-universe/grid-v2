mod error;
mod hypergrid;
mod rpc;
mod svm;

use anyhow::Result;
use clap::{Arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("grid-v2")
        // .version("0.1.0") // commented out to get version from manifest
        .about("Grid V2")
        .arg_required_else_help(true)
        .arg(Arg::new("NAME").help("Name").long("name").short('n'))
        .get_matches();

    Ok(())
}
