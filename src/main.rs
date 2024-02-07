use clap::Parser;
use std::io;
use eyre::{eyre, Result};
use structopt::StructOpt;
use tracing::debug;

use notary_server::{
    init_tracing, parse_config_file, run_server, CliFields, NotaryServerError,
    NotaryServerProperties,
};

use suave_andromeda_revm::StatefulExecutor;

#[derive(Parser)]
struct Cli {
    /// The rpc endpoint to connect to
    #[arg(short, long, default_value_t = String::from("http://127.0.0.1:8545"))]
    rpc: String,
    #[arg(short, long, default_value_t = false)]
    trace: bool,
}

#[tokio::main]
async fn main() -> Result<(), NotaryServerError> {
    let cli_args = Cli::parse();
    let mut service = StatefulExecutor::new_with_rpc(cli_args.rpc.clone());
    let cli_fields: CliFields = CliFields::from_args();
    let config: NotaryServerProperties = parse_config_file(&cli_fields.config_file)?;

    // Set up tracing for logging
    init_tracing(&config).map_err(|err| eyre!("Failed to set up tracing: {err}"))?;

    debug!(?config, "Server config loaded");

    // TODO: probably doesnt work due to async
    loop {
        let mut input_buf = String::new();
        io::stdin().read_line(&mut input_buf).expect("");
        match service
            .execute_command(input_buf.trim(), cli_args.trace)
            .await
        {
            Ok(res) => println!("{:?}", res),
            Err(e) => println!("{:?}", e),
        }
    }
}
