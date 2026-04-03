//! Poseidon - Edge trading and alerting daemon
//!
//! Entry point for the Poseidon CLI and daemon.

use poseidon::app::runner::run;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
