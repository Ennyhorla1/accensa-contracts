#![no_std]
use soroban_sdk::{contract, contracterror, contractimpl, contracttype, env, Address, Bytes, Env, Vec};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    BatchNotFound = 3,
    InvalidAmount = 4,
    Unauthorized = 5,
    BatchTooLarge = 6,
    BatchBelowPruneThreshold = 7,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Batch(u64),
    BatchCount,
    PrunedUpTo,
}

#[contract]
pub struct ReceiptAnchor;

#[contractimpl]
impl ReceiptAnchor {
    pub fn initialize(env: Env, merchant: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &merchant);
        env.storage().instance().set(&DataKey::BatchCount, &0u64);
        env.storage().instance().set(&DataKey::PrunedUpTo, &1u64);
        Ok(())
    }

    /// Extends the TTL of a batch to prevent archival.
    /// This function is permissionless: anyone may call it to extend the record's lifetime.
    pub fn extend_batch_ttl(env: Env, batch_id: u64) -> Result<(), Error> {
        let pruned_up_to: u64 = env.storage().instance().get(&DataKey::PrunedUpTo).unwrap_or(1);
        if batch_id < pruned_up_to {
            return Err(Error::BatchBelowPruneThreshold);
        }

        if !env.storage().persistent().has(&DataKey::Batch(batch_id)) {
            return Err(Error::BatchNotFound);
        }
        env.storage().persistent().extend_ttl(&DataKey::Batch(batch_id), 4096, 4096);
        Ok(())
    }

    pub fn get_batch(env: Env, batch_id: u64) -> Result<Vec<soroban_sdk::Map<soroban_sdk::Symbol, soroban_sdk::RawVal>>, Error> {
         // Implementation as before
         todo!()
    }
    
    // ... other methods
}