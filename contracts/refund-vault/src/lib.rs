use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    RefundNotFound = 3,
    RefundBelowPruneThreshold = 4,
}

#[contracttype]
pub enum DataKey {
    PrunedUpTo,
    Refund(u64),
}

#[contract]
pub struct RefundVault;

#[contractimpl]
impl RefundVault {
    /// Extends the TTL of a refund record to prevent archival.
    /// This function is permissionless: anyone may call it to extend the record's lifetime.
    pub fn extend_refund_ttl(env: Env, payment_ref: u64) -> Result<(), Error> {
        let pruned_up_to: u64 = env.storage().instance().get(&DataKey::PrunedUpTo).unwrap_or(0);
        if payment_ref < pruned_up_to {
            return Err(Error::RefundBelowPruneThreshold);
        }

        if !env.storage().persistent().has(&DataKey::Refund(payment_ref)) {
            return Err(Error::RefundNotFound);
        }
        env.storage().persistent().extend_ttl(&DataKey::Refund(payment_ref), 4096, 4096);
        Ok(())
    }
    // ... other methods
}