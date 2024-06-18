use crate::indexer::{Handleable, HandlerParams};
use alloy::{sol, sol_types::SolEvent};
use async_trait::async_trait;

sol!(
    #[sol(rpc)]
    VaultsRegistry,
    "abis/stakewise/VaultsRegistry.json"
);

sol!(
    #[sol(rpc)]
    EthVault,
    "abis/stakewise/EthVault.json"
);

#[derive(Clone)]
pub struct StakewiseHandler;

impl StakewiseHandler {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

#[async_trait]
impl Handleable for StakewiseHandler {
    // TODO: we should update multiple events to trigger a handler
    fn get_event_signature(&self) -> String {
        VaultsRegistry::VaultAdded::SIGNATURE.to_string()
    }

    async fn handle(&self, params: HandlerParams) {
        // Do nothing
    }
}