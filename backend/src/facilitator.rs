use x402_facilitator_local::{FacilitatorLocal, handlers};
use x402_types::chain::{ChainId, ChainProviderOps, ChainRegistry, FromConfig};
use x402_types::scheme::{
    SchemeBlueprints, SchemeHandlerSlug, SchemeRegistry,
    X402SchemeFacilitator, X402SchemeFacilitatorBuilder, X402SchemeFacilitatorError, X402SchemeId,
};
use x402_chain_solana::{V1SolanaExact, V2SolanaExact};
use x402_chain_solana::chain::{SolanaChainProvider, config::{SolanaChainConfig, SolanaChainConfigInner}};
use x402_types::config::Config;
use x402_types::facilitator::Facilitator;
use x402_types::proto;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::Router;
use tower_http::cors;
use axum::http::Method;

/// Custom facilitator that holds scheme handlers directly,
/// bypassing SchemeRegistry::build's incompatible trait bounds.
struct DirectFacilitator {
    handlers: HashMap<SchemeHandlerSlug, Box<dyn X402SchemeFacilitator>>,
}

impl Facilitator for DirectFacilitator {
    type Error = x402_facilitator_local::FacilitatorLocalError;

    async fn verify(
        &self,
        request: &proto::VerifyRequest,
    ) -> Result<proto::VerifyResponse, Self::Error> {
        let handler = request
            .scheme_handler_slug()
            .and_then(|slug| self.handlers.get(&slug).map(|h| h.as_ref()))
            .ok_or(x402_facilitator_local::FacilitatorLocalError::Verification(
                X402SchemeFacilitatorError::PaymentVerification(
                    proto::PaymentVerificationError::UnsupportedScheme,
                ),
            ))?;
        handler
            .verify(request)
            .await
            .map_err(x402_facilitator_local::FacilitatorLocalError::Verification)
    }

    async fn settle(
        &self,
        request: &proto::SettleRequest,
    ) -> Result<proto::SettleResponse, Self::Error> {
        let handler = request
            .scheme_handler_slug()
            .and_then(|slug| self.handlers.get(&slug).map(|h| h.as_ref()))
            .ok_or(x402_facilitator_local::FacilitatorLocalError::Settlement(
                X402SchemeFacilitatorError::PaymentVerification(
                    proto::PaymentVerificationError::UnsupportedScheme,
                ),
            ))?;
        handler
            .settle(request)
            .await
            .map_err(x402_facilitator_local::FacilitatorLocalError::Settlement)
    }

    async fn supported(&self) -> Result<proto::SupportedResponse, Self::Error> {
        let mut kinds = vec![];
        let mut extensions = std::collections::HashSet::new();
        let mut signers = HashMap::new();
        for handler in self.handlers.values() {
            if let Ok(mut supported) = handler.supported().await {
                kinds.append(&mut supported.kinds);
                for (chain_id, signer_addresses) in supported.signers {
                    signers.entry(chain_id).or_insert(signer_addresses);
                }
                extensions.extend(supported.extensions);
            }
        }
        Ok(proto::SupportedResponse {
            kinds,
            extensions: extensions.into_iter().collect(),
            signers,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize rustls crypto provider
    rustls::crypto::CryptoProvider::install_default(
        rustls::crypto::ring::default_provider()
    ).expect("Failed to initialize rustls crypto provider");

    // Load .env variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config: Config<HashMap<ChainId, SolanaChainConfigInner>> = Config::load()?;

    // Build scheme handlers directly for each chain
    let mut handlers: HashMap<SchemeHandlerSlug, Box<dyn X402SchemeFacilitator>> = HashMap::new();

    for (chain_id, chain_config_inner) in config.chains().iter() {
        let chain_reference = chain_id.clone().try_into()
            .map_err(|e: x402_chain_solana::chain::SolanaChainReferenceFormatError| {
                format!("Invalid chain reference for {}: {}", chain_id, e)
            })?;
        let solana_config = SolanaChainConfig {
            chain_reference,
            inner: chain_config_inner.clone(),
        };
        let provider = Arc::new(SolanaChainProvider::from_config(&solana_config).await?);

        // Build V1 handler
        let v1_handler = V1SolanaExact.build(provider.clone(), None)?;
        let v1_slug = SchemeHandlerSlug::new(
            chain_id.clone(),
            V1SolanaExact.x402_version(),
            V1SolanaExact.scheme().to_string(),
        );
        handlers.insert(v1_slug, v1_handler);

        // Build V2 handler
        let v2_handler = V2SolanaExact.build(provider, None)?;
        let v2_slug = SchemeHandlerSlug::new(
            chain_id.clone(),
            V2SolanaExact.x402_version(),
            V2SolanaExact.scheme().to_string(),
        );
        handlers.insert(v2_slug, v2_handler);
    }

    // Create facilitator with directly-built handlers
    let facilitator = DirectFacilitator { handlers };
    let state = Arc::new(facilitator);

    // Create HTTP routes with CORS
    let app = Router::new()
        .merge(handlers::routes().with_state(state))
        .layer(
            cors::CorsLayer::new()
                .allow_origin(cors::Any)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers(cors::Any),
        );

    // Run server
    let addr = SocketAddr::new(config.host(), config.port());
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}