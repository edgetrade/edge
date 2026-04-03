use crate::domains::client::generated::routes::requests::orders_place_spot_order::PlaceSpotOrderResponseItem;

/// Result type for command operations.
pub type CommandResult<T> = Result<T, super::error::CommandError>;

/// Success: Key already exists (idempotent state)
pub fn key_exists() {
    println!("Key already exists. You're good to go!");
}

/// Success: Key creation completed
pub fn key_created() {
    println!("✓ Key created and stored in OS keyring");
}

/// Success/Info: Storage location
pub fn storage_location(dir: &str) {
    println!("Storage location: {}", dir);
}

pub fn successful_order(response: Vec<PlaceSpotOrderResponseItem>) {
    for item in response {
        println!("Order placed successfully for wallet: {}", item.wallet);
        for tx in item.transactions {
            if let Some(success) = tx.subtype_0 {
                println!("  Transaction successful: {}", success.hash);
            }
            if let Some(failure) = tx.subtype_1 {
                println!("  Transaction failed: {}", failure.error);
            }
        }
    }
}

/// Success/Info: Wallet list header
pub fn wallets_list_header() {
    println!("Wallets:");
}

/// Success/Info: Wallet item
pub fn wallet_item(chain: &str, address: &str, name: &str) {
    match name.len() {
        0 => wallet_item_no_name(chain, address),
        _ => wallet_item_has_name(chain, address, name),
    }
}

pub fn wallet_item_no_name(chain: &str, address: &str) {
    println!("  {}: {}", chain, address);
}

pub fn wallet_item_has_name(chain: &str, address: &str, name: &str) {
    println!("  {}: {} ({})", chain, address, name);
}

/// Success/Info: Wallet import progress
pub fn wallet_importing() {
    println!("Importing your wallet...");
}

/// Success/Info: Wallet created
pub fn wallet_created(chain: &str, address: &str) {
    println!("✓ Wallet created successfully for {}: {}", chain, address);
}

/// Success/Info: Wallet imported
pub fn wallet_imported(chain: &str, address: &str) {
    println!("✓ Wallet imported successfully for {}: {}", chain, address);
}

/// Success/Info: Wallet deleted
pub fn wallet_deleted(address: &str) {
    println!("✓ Wallet deleted successfully: {}", address);
}

/// Success: Key config created (filestore)
pub fn key_config_created() {
    println!("✓ Key configuration created successfully");
}

/// Success/Info: Password updated
pub fn password_updated() {
    println!("Password updated successfully");
}

/// Success/Info: Session unlocked
pub fn session_unlocked() {
    println!("Session unlocked successfully");
}

/// Success/Info: Session locked
pub fn session_locked() {
    println!("Session locked successfully");
}

/// Success/Info: Key updated
pub fn key_updated() {
    println!("✓ Key updated successfully");
}

/// Success/Info: Key deleted
pub fn key_deleted() {
    println!("Key deleted from OS keyring");
}

/// Success/Info: Key config deleted
pub fn key_config_deleted() {
    println!("Key configuration deleted successfully");
}

/// Success: No key found (filestore fallback)
pub fn key_not_found() {
    println!("No key found in OS keyring");
}

/// Informational: User needs to confirm they want to create a key
pub fn no_key_found_create() {
    println!("No key found. Let's create one first...");
}

/// Success/Info: Key config not found
pub fn key_config_not_found() {
    println!("No key configuration found");
}

/// Success/Info: Session already unlocked
pub fn session_already_unlocked() {
    println!("Session is already unlocked");
}

/// Success/Info: Session unlocked (filestore)
pub fn session_unlocked_filestore() {
    println!("Session unlocked successfully");
}

/// Success/Info: Session locking
pub fn session_unlocking() {
    println!("Session locked. Unlocking now...");
}

/// Success: Keyring unlock help
pub fn keyring_unlock_help() {
    println!("Lock/Unlock your session from your operating system's keyring.");
}

/// Success: Keyring lock help
pub fn keyring_lock_help() {
    println!("Lock/Unlock your session from your operating system's keyring.");
}

/// Success: Session already locked (filestore)
pub fn session_already_locked() {
    println!("Session is already locked");
}

/// Success: Session remains unlocked (filestore update)
pub fn session_remains_unlocked() {
    println!("Session remains unlocked");
}

/// Success: Creating key in OS keyring
pub fn creating_key_os_keyring() {
    println!("Creating a new key in the OS keyring...");
}

/// Success: Key creation cancelled
pub fn key_creation_cancelled() {
    println!("Key creation cancelled.");
}

/// Success: Use key unlock verify
pub fn use_key_unlock_verify() {
    println!("  Use 'edge key unlock' to verify access");
}

/// Success/Info: Session reloaded
pub fn manifest_reloaded() {
    println!("Manifest reloaded");
}

/// Success/Info: Manifest cached
pub fn manifest_cached() {
    println!("Manifest loaded from cache");
}

/// Success/Info: Manifest refreshed
pub fn manifest_refreshed() {
    println!("Manifest refreshed");
}

/// Success: Query response
pub fn query_response(path: &str, output: &str) {
    println!("Query response {}\toutput={}", path, output);
}

/// Success: Query request
pub fn query_request(path: &str, input: &str) {
    println!("Query request: {}\tinput={}", path, input);
}

/// Success: Connection URL and key (for two-argument verbose logging)
pub fn connecting_to_url(url: &str) {
    println!("[edge] connecting to {}", url);
}

/// Success/Info: Skill output
pub fn skill_output(name: &str, description: &str) {
    println!("{}: {}", name, description);
}

/// Success: Ping success
pub fn ping_success(status: &str) {
    println!("Ping succeeded with status: {}", status);
}

/// Success: Subscription stop
pub fn subscription_stop(id: u32) {
    println!("Subscription stopped: {}", id);
}

/// Success: Subscribe registered
pub fn subscribe_registered(path: &str, id: u32) {
    println!("Subscribe registered: {} (id={})", path, id);
}

/// Success: Subscribe reconnect
pub fn subscribe_reconnect(path: &str, id: u32) {
    println!("Subscribe reconnect: {} (id={})", path, id);
}

/// Success: Subscribe request
pub fn subscribe_request(path: &str, id: u32, input: &str) {
    println!("Subscribe request: {} (id={}) input={}", path, id, input);
}

/// Success: Mutation request
pub fn mutation_request(path: &str, input: &str) {
    println!("Mutation request: {} input={}", path, input);
}

/// Success: Mutation response
pub fn mutation_response(path: &str) {
    println!("Mutation response: {}", path);
}

/// Success: JSON output
pub fn json_output(json: &str) {
    println!("{}", json);
}
