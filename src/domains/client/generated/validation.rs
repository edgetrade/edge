// Auto-generated - do not edit manually
use super::routes::requests::agent_change_encrypted_wallets;
use super::routes::requests::agent_create_encrypted_wallet;
use super::routes::requests::agent_delete_encrypted_wallet;
use super::routes::requests::agent_get_transport_key;
use super::routes::requests::agent_list_encrypted_wallets;
use super::routes::requests::agent_proof_game;
use super::routes::requests::agent_rotate_user_encryption_key;
use super::routes::requests::intelligence_screen_tokens;
use super::routes::requests::intelligence_search_swaps;
use super::routes::requests::intelligence_search_tokens;
use super::routes::requests::orders_apply_entry_strategy;
use super::routes::requests::orders_apply_exit_strategy;
use super::routes::requests::orders_cancel;
use super::routes::requests::orders_cancel_all;
use super::routes::requests::orders_create_entry_strategy;
use super::routes::requests::orders_create_exit_strategy;
use super::routes::requests::orders_get;
use super::routes::requests::orders_list;
use super::routes::requests::orders_list_entry_strategies;
use super::routes::requests::orders_list_exit_strategies;
use super::routes::requests::orders_place_limit_order;
use super::routes::requests::orders_place_spot_order;
use super::routes::requests::orders_remove_entry_strategy;
use super::routes::requests::orders_remove_exit_strategy;
use super::routes::requests::orders_update_entry_strategy;
use super::routes::requests::orders_update_exit_strategy;
use super::routes::requests::pairs_get_pair_candles;
use super::routes::requests::pairs_get_pair_detailed;
use super::routes::requests::pairs_get_pair_metrics;
use super::routes::requests::pairs_get_swaps;
use super::routes::requests::tokens_get_dev_tokens;
use super::routes::requests::tokens_get_token_simple;
use super::routes::requests::tokens_get_top_holders;
use super::routes::requests::tokens_get_top_traders;
use super::routes::requests::wallet_get_holding_history;
use super::routes::requests::wallet_get_holdings;
use super::routes::requests::wallet_get_native_balances;
use super::routes::requests::wallet_get_summary;
use super::routes::requests::wallet_get_wallet_swaps;
use super::routes::subscriptions::alerts_on_memescope;
use super::routes::subscriptions::alerts_on_order_updates;
use super::routes::subscriptions::alerts_on_pair_swaps;
use super::routes::subscriptions::alerts_on_pair_updates;
use super::routes::subscriptions::alerts_on_ping;
use super::routes::subscriptions::alerts_on_portfolio_updates;
use super::routes::subscriptions::alerts_on_token_updates;
use super::routes::subscriptions::alerts_on_wallet_swaps;
use crate::client::IrisClient;
use crate::client::Route;
use crate::messages::IrisClientError;
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
/// Trait for route execution
pub trait RouteValidator: Send + Sync {
    /// Returns the procedure name for this route
    fn procedure(&self) -> &'static str;
    /// Parse data and execute - single method, no double deserialization
    fn execute<'a>(
        &'a self,
        client: &'a IrisClient,
        data: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Value, IrisClientError>> + Send + 'a>>;
}
impl<
    I: serde::de::DeserializeOwned + serde::Serialize + Send + Sync,
    O: serde::de::DeserializeOwned + serde::Serialize + Clone + Send + Sync,
> RouteValidator for Route<I, O>
{
    fn procedure(&self) -> &'static str {
        self.procedure
    }
    fn execute<'a>(
        &'a self,
        client: &'a IrisClient,
        data: Value,
    ) -> Pin<Box<dyn Future<Output = Result<Value, IrisClientError>> + Send + 'a>> {
        Box::pin(async move {
            let input: I = serde_json::from_value(data).map_err(|e| IrisClientError::Deserialization(e.to_string()))?;
            use crate::client::RouteExecutor;
            let result: O = client.execute(self, &input).await?;
            serde_json::to_value(result).map_err(|e| IrisClientError::Serialization(e.to_string()))
        })
    }
}
/// Find a route by its procedure name
pub fn find_route(procedure: &str) -> Option<&'static (dyn RouteValidator + Sync)> {
    match procedure {
        "agent.changeEncryptedWallets" => Some(&agent_change_encrypted_wallets::ROUTE),
        "agent.createEncryptedWallet" => Some(&agent_create_encrypted_wallet::ROUTE),
        "agent.deleteEncryptedWallet" => Some(&agent_delete_encrypted_wallet::ROUTE),
        "agent.getTransportKey" => Some(&agent_get_transport_key::ROUTE),
        "agent.listEncryptedWallets" => Some(&agent_list_encrypted_wallets::ROUTE),
        "agent.proofGame" => Some(&agent_proof_game::ROUTE),
        "agent.rotateUserEncryptionKey" => Some(&agent_rotate_user_encryption_key::ROUTE),
        "alerts.onMemescope" => Some(&alerts_on_memescope::ROUTE),
        "alerts.onOrderUpdates" => Some(&alerts_on_order_updates::ROUTE),
        "alerts.onPairSwaps" => Some(&alerts_on_pair_swaps::ROUTE),
        "alerts.onPairUpdates" => Some(&alerts_on_pair_updates::ROUTE),
        "alerts.onPing" => Some(&alerts_on_ping::ROUTE),
        "alerts.onPortfolioUpdates" => Some(&alerts_on_portfolio_updates::ROUTE),
        "alerts.onTokenUpdates" => Some(&alerts_on_token_updates::ROUTE),
        "alerts.onWalletSwaps" => Some(&alerts_on_wallet_swaps::ROUTE),
        "intelligence.screenTokens" => Some(&intelligence_screen_tokens::ROUTE),
        "intelligence.searchSwaps" => Some(&intelligence_search_swaps::ROUTE),
        "intelligence.searchTokens" => Some(&intelligence_search_tokens::ROUTE),
        "orders.applyEntryStrategy" => Some(&orders_apply_entry_strategy::ROUTE),
        "orders.applyExitStrategy" => Some(&orders_apply_exit_strategy::ROUTE),
        "orders.cancel" => Some(&orders_cancel::ROUTE),
        "orders.cancelAll" => Some(&orders_cancel_all::ROUTE),
        "orders.createEntryStrategy" => Some(&orders_create_entry_strategy::ROUTE),
        "orders.createExitStrategy" => Some(&orders_create_exit_strategy::ROUTE),
        "orders.get" => Some(&orders_get::ROUTE),
        "orders.list" => Some(&orders_list::ROUTE),
        "orders.listEntryStrategies" => Some(&orders_list_entry_strategies::ROUTE),
        "orders.listExitStrategies" => Some(&orders_list_exit_strategies::ROUTE),
        "orders.placeLimitOrder" => Some(&orders_place_limit_order::ROUTE),
        "orders.placeSpotOrder" => Some(&orders_place_spot_order::ROUTE),
        "orders.removeEntryStrategy" => Some(&orders_remove_entry_strategy::ROUTE),
        "orders.removeExitStrategy" => Some(&orders_remove_exit_strategy::ROUTE),
        "orders.updateEntryStrategy" => Some(&orders_update_entry_strategy::ROUTE),
        "orders.updateExitStrategy" => Some(&orders_update_exit_strategy::ROUTE),
        "pairs.getPairCandles" => Some(&pairs_get_pair_candles::ROUTE),
        "pairs.getPairDetailed" => Some(&pairs_get_pair_detailed::ROUTE),
        "pairs.getPairMetrics" => Some(&pairs_get_pair_metrics::ROUTE),
        "pairs.getSwaps" => Some(&pairs_get_swaps::ROUTE),
        "tokens.getDevTokens" => Some(&tokens_get_dev_tokens::ROUTE),
        "tokens.getTokenSimple" => Some(&tokens_get_token_simple::ROUTE),
        "tokens.getTopHolders" => Some(&tokens_get_top_holders::ROUTE),
        "tokens.getTopTraders" => Some(&tokens_get_top_traders::ROUTE),
        "wallet.getHoldingHistory" => Some(&wallet_get_holding_history::ROUTE),
        "wallet.getHoldings" => Some(&wallet_get_holdings::ROUTE),
        "wallet.getNativeBalances" => Some(&wallet_get_native_balances::ROUTE),
        "wallet.getSummary" => Some(&wallet_get_summary::ROUTE),
        "wallet.getWalletSwaps" => Some(&wallet_get_wallet_swaps::ROUTE),
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_route_known() {
        let route = find_route("agent.listEncryptedWallets");
        assert!(route.is_some());
        assert_eq!(route.unwrap().procedure(), "agent.listEncryptedWallets");
    }
    #[test]
    fn test_find_route_unknown() {
        let route = find_route("unknown.nonexistent");
        assert!(route.is_none());
    }
}
