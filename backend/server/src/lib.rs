// serve images and handle api requests
use axum::extract::FromRef;
use uchat_query::{AsyncConnection, AsyncConnectionPool, QueryError};
use color_eyre::{Result, Help, eyre::Context};
use color_eyre::eyre::WrapErr;

pub mod logging;
pub mod router;
pub mod error;
pub mod extractor; 
pub mod handler;

#[derive(FromRef, Clone)]
pub struct AppState{
    // seen in query
    pub db_pool: AsyncConnectionPool,
    pub signing_keys: uchat_crypto::sign::Keys,
    pub rng: rand::rngs::StdRng,
}

impl AppState {
    pub async fn connect(&self) -> Result<AsyncConnection, QueryError> {
        self.db_pool.get().await
    }
}

pub mod cli {
    use color_eyre::{eyre::Context, Section};
    use rand::{CryptoRng, RngCore};
    use uchat_crypto::sign::{encode_private_key, EncodedPrivateKey, Keys};

    // Use color_eyre error because it accepts any result
    pub fn gen_keys<R>(rng: &mut R) -> color_eyre::Result<(EncodedPrivateKey, Keys)>
    where 
        R: CryptoRng + RngCore,
    {
        let (private_key, keys) = Keys::generate(rng)?;
        let private_key = encode_private_key(private_key)?;
        Ok((private_key, keys))
    }

    pub fn load_keys() -> color_eyre::Result<Keys> {
        let private_key = std::env::var("API_PRIVATE_KEY")
            .wrap_err("failed to locate private key")
            .suggestion("set API_PRIVATE_KEY env var")?;

        Ok(Keys::from_encoded(private_key)?)
    }
}

