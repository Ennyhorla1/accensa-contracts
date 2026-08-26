#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_extend_batch_ttl_as_non_admin_succeeds() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ReceiptAnchor);
        let client = ReceiptAnchorClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        
        // Anchor batch
        client.anchor_batch(...); 
        
        let non_admin = Address::generate(&env);
        env.mock_all_auths();
        // This should not panic or require admin auth
        client.extend_batch_ttl(&1u64);
    }

    #[test]
    fn test_extend_batch_ttl_fails_if_pruned() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ReceiptAnchor);
        let client = ReceiptAnchorClient::new(&env, &contract_id);
        let admin = Address::generate(&env);
        client.initialize(&admin);
        
        // anchor... and prune... 
        client.prune_batches(&10u32);
        
        let res = client.try_extend_batch_ttl(&1u64);
        assert_eq!(res, Err(Ok(Error::BatchBelowPruneThreshold)));
    }
}