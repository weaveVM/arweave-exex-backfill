use crate::utils::constants::IRYS_UPLOADER_URL;
use crate::utils::env_var::get_env_var;
use bundlr_sdk::{
    currency::solana::{Solana, SolanaBuilder},
    tags::Tag,
    Bundlr, BundlrBuilder,
};
use eyre::eyre;
use reqwest::Url;

#[derive(Clone, Debug)]
pub struct UploaderProvider {
    private_key: Option<String>,
}

async fn init_bundlr(private_key: Option<String>) -> eyre::Result<Bundlr<Solana>> {
    let irys_wallet_pk: String = get_env_var("irys_pk").unwrap_or_else(|e| private_key.unwrap());
    let url = Url::parse(IRYS_UPLOADER_URL).unwrap();

    let currency = SolanaBuilder::new()
        .wallet(&irys_wallet_pk)
        .build()
        .map_err(|e| {
            eyre::eyre!(
                "failed to initialize irys provider, failed to create bundlr wallet instance: {}",
                e
            )
        })?;

    let bundlr = BundlrBuilder::new()
        .url(url)
        .currency(currency)
        .fetch_pub_info()
        .await
        .map_err(|e| eyre::eyre!("failed to fetch bundlr public info: {}", e))?
        .build()
        .map_err(|e| eyre::eyre!("bundlr successfully initialized: {}", e))?;

    Ok(bundlr)
}

impl UploaderProvider {
    pub fn new(private_key: Option<String>) -> UploaderProvider {
        UploaderProvider { private_key }
    }

    pub async fn upload_data(&self, data: Vec<u8>, param_tags: Vec<Tag>) -> eyre::Result<String> {
        let mut tags = vec![
            Tag::new("Protocol", "WeaveVM-ExEx"),
            Tag::new("ExEx-Type", "Arweave-Data-Uploader"),
        ];

        tags.extend(param_tags);

        let bundlr = init_bundlr(self.private_key.clone())
            .await
            .map_err(|e| eyre!("failed to initialize bundlr: {}", e))?;

        let mut tx = bundlr
            .create_transaction(data, tags)
            .map_err(|e| eyre!("failed to create transaction: {}", e))?;

        bundlr
            .sign_transaction(&mut tx)
            .await
            .map_err(|e| eyre!("failed to sign transaction: {}", e))?;

        let result = bundlr
            .send_transaction(tx)
            .await
            .map_err(|e| eyre!("failed to send transaction: {}", e))?;

        let id = result["id"]
            .as_str()
            .ok_or_else(|| eyre!("missing 'id' field in response"))?
            .to_string();

        eyre::Ok(id)
    }
}

#[derive(Clone, Debug)]
pub struct ArweaveRequest {
    tags: Vec<Tag>,
    data: Vec<u8>,
    private_key: Option<String>,
}

impl ArweaveRequest {
    pub fn new() -> Self {
        ArweaveRequest {
            tags: vec![],
            data: vec![],
            private_key: None,
        }
    }

    pub fn set_tag(&mut self, name: &str, value: &str) -> &mut ArweaveRequest {
        self.tags.push(Tag::new(name, value));
        self
    }

    pub fn set_data(&mut self, data: Vec<u8>) -> &mut ArweaveRequest {
        self.data = data;
        self
    }

    pub fn set_private_key(&mut self, data: String) -> &mut ArweaveRequest {
        self.private_key = Some(data);
        self
    }

    pub async fn send(&self) -> eyre::Result<String> {
        let provider = UploaderProvider::new(self.private_key.clone());
        self.send_with_provider(&provider).await
    }

    pub async fn send_with_provider(&self, provider: &UploaderProvider) -> eyre::Result<String> {
        provider
            .upload_data(self.data.clone(), self.tags.clone())
            .await
    }
}
