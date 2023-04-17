use std::sync::Arc;

use abstract_boot::{
    AppDeployer,
    boot_core::*,
    boot_core::networks::{NetworkInfo, parse_network}
};
use semver::Version;

use abstract_etf_app::{
    boot::ETF,
    ETF_ID
};
use clap::Parser;
use tokio::runtime::Runtime;

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn deploy_etf(network: NetworkInfo) -> anyhow::Result<()> {
    let version: Version = CONTRACT_VERSION.parse().unwrap();
    let rt = Arc::new(Runtime::new()?);
    let options = DaemonOptionsBuilder::default().network(network).build();
    let (_sender, chain) = instantiate_daemon_env(&rt, options?)?;
    let mut etf = ETF::new(ETF_ID, chain.clone());

    etf.deploy(version)?;
    Ok(())
}

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network Id to deploy on
    #[arg(short, long)]
    network_id: String,
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    let args = Arguments::parse();

    let network = parse_network(&args.network_id);

    deploy_etf(network)
}
