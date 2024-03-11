use std::sync::Arc;
use alloy::{
    rpc::types::eth::transaction::request::{TransactionInput, TransactionRequest},
    core::primitives::Address,
};

pub async fn make_eth_call_request<C: SolCall, T: TracingProvider> (
    call: C,
    provider: &Arc<T>,
    to: Address,
) -> eyre::Result<C::Return> {
    let encoded = call.encode();
    let req = TransactionRequest {
        to: Some(to),
        input: TransactionInput::new(encoded.into()),
        ..Default::default()
    };
    println!("{:#?}", req);
    Ok(())
}

#[derive(Debug, Clone)]
pub struct LocalProvider {
    provider: Arc<Provider<Http>Request::Client>>,
}

impl LocalProvider {
    pub fn new(url: String) -> Self {
        let http = Http::new(url.parse().unwrap());
        Self { provider: Arc::new(Provider::new(http)) }
    }
}

#[async_trait::async_trait]
impl TracingProvider for LocalProvider {
    async fn eth_call(
        &self,
        request: TransactionRequest,
        block_number: Option<BlockId>,
        state_overrides: Option<StateOverrides>,
        block_overrides: Option<Box<BlockOverrides>>,
    ) -> eyre::Result<Bytes> {
        if state_overrides.is_some() || block_overrides.is_some() {
            panic!("local provider doesn't support block or state overrides");
        }
        self.provider
            .call(request, block_number)
            .await
            .map_err(Into::into)
    }
}