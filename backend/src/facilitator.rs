use x402_types::networks::USDC;
use x402_chain_solana::{V1SolanaExact, KnownNetworkSolana};
use x402_facilitator::chain::solana::SolanaProvider;
use x402_facilitator::chain::FromEnvByNetworkBuild;
use x402_facilitator::facilitator::Facilitator;
use x402_facilitator::network::Network;
use x402_facilitator::types::{SettleRequest, VerifyRequest};
use x402_types::chain::FromConfig;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let provider = SolanaProvider::from_env(Network::SolanaDevnet)
        .await?
        .expect("Solana devnet provider not configured — check env vars");

    let usdc = USDC::solana();

    let pay_to = "9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM";
    let max_amount_required = usdc.amount(1_000_000u64);


    let verify_request: VerifyRequest = todo!("build from incoming HTTP request");
    let settle_request: SettleRequest = todo!("build from incoming HTTP request");

    let verify_response = provider.verify(&verify_request).await?;

    let settle_response = provider.settle(&settle_request).await?;

    Ok(())

}