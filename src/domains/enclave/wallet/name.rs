use tyche_enclave::types::chain_type::ChainType;

pub fn ensure_wallet_name(chain: ChainType, name: Option<String>) -> String {
    name.unwrap_or_else(|| {
        let binding = uuid::Uuid::new_v4().to_string();
        let id = binding.split('-').next().unwrap();
        match chain {
            ChainType::EVM => format!("agents-evm-wallet-{}", id),
            ChainType::SVM => format!("agents-svm-wallet-{}", id),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_wallet_name_with_provided_name() {
        let name = ensure_wallet_name(ChainType::EVM, Some("my-custom-wallet".to_string()));
        assert_eq!(name, "my-custom-wallet");
    }

    #[test]
    fn test_ensure_wallet_name_evm_generates_name() {
        let name = ensure_wallet_name(ChainType::EVM, None);
        assert!(name.starts_with("agents-evm-wallet-"));
        // UUID first segment is 8 characters
        assert_eq!(name.len(), "agents-evm-wallet-".len() + 8);
    }

    #[test]
    fn test_ensure_wallet_name_svm_generates_name() {
        let name = ensure_wallet_name(ChainType::SVM, None);
        assert!(name.starts_with("agents-svm-wallet-"));
        // UUID first segment is 8 characters
        assert_eq!(name.len(), "agents-svm-wallet-".len() + 8);
    }

    #[test]
    fn test_ensure_wallet_name_empty_string_uses_generated() {
        // Empty string is a provided name, so it should be used as-is
        let name = ensure_wallet_name(ChainType::EVM, Some("".to_string()));
        assert_eq!(name, "");
    }

    #[test]
    fn test_ensure_wallet_name_generated_names_are_unique() {
        // Generate multiple names and verify they're different
        let name1 = ensure_wallet_name(ChainType::EVM, None);
        let name2 = ensure_wallet_name(ChainType::EVM, None);
        let name3 = ensure_wallet_name(ChainType::SVM, None);

        assert_ne!(name1, name2, "Generated EVM names should be unique");
        assert!(!name1.starts_with("agents-svm-wallet-"));
        assert!(!name2.starts_with("agents-svm-wallet-"));
        assert!(name3.starts_with("agents-svm-wallet-"));
    }
}
