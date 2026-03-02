mod alerts;
mod inspect;
mod portfolio;
mod screen;
mod search;
mod trade;

use crate::client::IrisClient;
use rmcp::Server;

pub fn register_tools(server: &Server, client: IrisClient) -> Result<(), Box<dyn std::error::Error>> {
    search::register(server, client.clone())?;
    inspect::register(server, client.clone())?;
    screen::register(server, client.clone())?;
    portfolio::register(server, client.clone())?;
    trade::register(server, client.clone())?;
    alerts::register(server, client)?;
    Ok(())
}
