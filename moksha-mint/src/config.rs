use std::{env, net::SocketAddr, path::PathBuf, str::FromStr};

use clap::Parser;
use moksha_core::primitives::{CurrencyUnit, Nut14, Nut15, PaymentMethod, PaymentMethodConfig};
use serde::{Deserialize, Serialize};

use crate::lightning::{
    alby::AlbyLightningSettings, cln::ClnLightningSettings, lnbits::LnbitsLightningSettings,
    lnd::LndLightningSettings, strike::StrikeLightningSettings, LightningType,
};

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(long, env = "MINT_PRIVATE_KEY")]
    pub privatekey: String,
    #[clap(flatten)]
    pub info: MintInfoConfig,
    #[clap(flatten)]
    pub lightning_fee: LightningFeeConfig,
    #[clap(flatten)]
    pub server: ServerConfig,
    #[clap(flatten)]
    pub database: DatabaseConfig,

    #[clap(long, env = "MINT_LIGHTNING_BACKEND")]
    pub lightning_backend: LightningTypeVariant,

    #[clap(long, env = "MINT_BTC_ONCHAIN_BACKEND")]
    pub btconchain_backend: Option<BtcOnchainTypeVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightningTypeVariant {
    Lnbits,
    Alby,
    Strike,
    Lnd,
    Cln,
}

impl FromStr for LightningTypeVariant {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lnbits" => Ok(Self::Lnbits),
            "Alby" => Ok(Self::Alby),
            "Strike" => Ok(Self::Strike),
            "Lnd" => Ok(Self::Lnd),
            "Cln" => Ok(Self::Cln),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MintConfig {
    pub privatekey: String,
    pub info: MintInfoConfig,
    pub lightning_fee: LightningFeeConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub btconchain_backend: Option<BtcOnchainConfig>,
    pub lightning_backend: Option<LightningType>,
}

impl From<(Opts, LightningType, Option<BtcOnchainConfig>)> for MintConfig {
    fn from((opts, ln, btc): (Opts, LightningType, Option<BtcOnchainConfig>)) -> Self {
        Self {
            privatekey: opts.privatekey,
            info: opts.info,
            lightning_fee: opts.lightning_fee,
            server: opts.server,
            database: opts.database,
            btconchain_backend: btc,
            lightning_backend: Some(ln),
        }
    }
}

impl MintConfig {
    pub fn read_config_with_defaults() -> Self {
        let opts: Opts = Opts::parse();

        let lightning = match opts.lightning_backend {
            LightningTypeVariant::Lnd => LightningType::Lnd(LndLightningSettings::parse()),
            LightningTypeVariant::Lnbits => LightningType::Lnbits(LnbitsLightningSettings::parse()),
            LightningTypeVariant::Strike => LightningType::Strike(StrikeLightningSettings::parse()),
            LightningTypeVariant::Alby => LightningType::Alby(AlbyLightningSettings::parse()),
            LightningTypeVariant::Cln => LightningType::Cln(ClnLightningSettings::parse()),
        };

        let btc_onchain: Option<BtcOnchainConfig> = match opts.btconchain_backend {
            Some(BtcOnchainTypeVariant::Lnd) => {
                let cfg = BtcOnchainConfig::parse();
                Some(BtcOnchainConfig {
                    onchain_type: Some(BtcOnchainType::Lnd(LndLightningSettings::parse())),
                    ..cfg
                })
            }
            None => None,
        };

        (opts, lightning, btc_onchain).into()
    }
}

impl MintConfig {
    pub const fn new(
        private_key: String,
        info: MintInfoConfig,
        lightning_fee: LightningFeeConfig,
        server: ServerConfig,
        database: DatabaseConfig,
        btconchain_backend: Option<BtcOnchainConfig>,
        lightning_backend: Option<LightningType>,
    ) -> Self {
        Self {
            privatekey: private_key,
            info,
            lightning_fee,
            server,
            database,
            btconchain_backend,
            lightning_backend,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Parser)]
pub struct BtcOnchainConfig {
    #[clap(skip)]
    pub onchain_type: Option<BtcOnchainType>,

    #[clap(long, default = 1, env = "MINT_BTC_ONCHAIN_BACKEND_MIN_CONFIRMATIONS")]
    pub min_confirmations: u8,

    #[clap(long, env = "MINT_BTC_ONCHAIN_BACKEND_MIN_AMOUNT")]
    pub min_amount: u64,

    #[clap(long, env = "MINT_BTC_ONCHAIN_BACKEND_MAX_AMOUNT")]
    pub max_amount: u64,
}

impl Default for BtcOnchainConfig {
    fn default() -> Self {
        Self {
            onchain_type: None,
            min_confirmations: 1,
            min_amount: 1_000,
            max_amount: 1_000_000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BtcOnchainType {
    Lnd(LndLightningSettings),
}

#[derive(Debug, Clone)]
pub enum BtcOnchainTypeVariant {
    Lnd,
}

impl FromStr for BtcOnchainTypeVariant {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Lnd" => Ok(Self::Lnd),
            _ => Err("no match"),
        }
    }
}

impl From<BtcOnchainConfig> for Nut14 {
    fn from(settings: BtcOnchainConfig) -> Self {
        Self {
            supported: true,
            payment_methods: vec![PaymentMethodConfig {
                payment_method: PaymentMethod::BtcOnchain,
                unit: CurrencyUnit::Sat,
                min_amount: settings.min_amount,
                max_amount: settings.max_amount,
            }],
        }
    }
}

impl From<BtcOnchainConfig> for Nut15 {
    fn from(settings: BtcOnchainConfig) -> Self {
        Self {
            supported: true,
            payment_methods: vec![PaymentMethodConfig {
                payment_method: PaymentMethod::BtcOnchain,
                unit: CurrencyUnit::Sat,
                min_amount: settings.min_amount,
                max_amount: settings.max_amount,
            }],
        }
    }
}

#[derive(Debug, Clone, Default, Parser)]
pub struct DatabaseConfig {
    #[clap(long, env = "MINT_DB_URL")]
    pub db_url: String,
}

#[derive(Debug, Clone, Parser)]
pub struct ServerConfig {
    #[clap(long, env = "MINT_HOST_PORT")]
    pub host_port: SocketAddr,
    #[clap(long, env = "MINT_SERVE_WALLET_PATH")]
    pub serve_wallet_path: Option<PathBuf>,
    #[clap(long, env = "MINT_API_PREFIX")]
    pub api_prefix: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host_port: "[::]:3338".to_string().parse().expect("invalid host port"),
            serve_wallet_path: None,
            api_prefix: None,
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone, Default, Parser)]
pub struct MintInfoConfig {
    #[clap(long, env = "MINT_INFO_NAME")]
    pub name: Option<String>,

    #[clap(long, env = "MINT_INFO_VERSION")]
    #[serde(default = "default_version")]
    pub version: bool,

    #[clap(long, env = "MINT_INFO_DESCRIPTION")]
    pub description: Option<String>,

    #[clap(long, env = "MINT_INFO_DESCRIPTION_LONG")]
    pub description_long: Option<String>,

    #[clap(long, env = "MINT_INFO_CONTACT")]
    //pub contact: Option<Vec<Vec<String>>>,
    pub contact: Option<String>, // FIXME change to Vec<Vec<String>>

    #[clap(long, env = "MINT_INFO_MOTD")]
    pub motd: Option<String>,
    // FIXME add missing fields for v1/info endpoint nut4/nut5 payment_methods, nut4 disabled flag
}

const fn default_version() -> bool {
    true
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct BuildParams {
    pub commit_hash: Option<String>,
    pub build_time: Option<String>,
    pub cargo_pkg_version: Option<String>,
}

impl BuildParams {
    pub fn from_env() -> Self {
        Self {
            commit_hash: env::var("COMMITHASH").ok(),
            build_time: env::var("BUILDTIME").ok(),
            cargo_pkg_version: Some(env!("CARGO_PKG_VERSION").to_owned()),
        }
    }

    pub fn full_version(&self) -> String {
        format!(
            "{}-{}",
            self.cargo_pkg_version
                .as_ref()
                .unwrap_or(&"unknown".to_string()),
            self.commit_hash.as_ref().unwrap_or(&"unknown".to_string())
        )
    }
}

#[derive(Debug, Clone, Parser)]
pub struct LightningFeeConfig {
    #[clap(long, env = "MINT_LIGHTNING_FEE_PERCENT")]
    pub fee_percent: f32,
    #[clap(long, env = "MINT_LIGHTNING_FEE_RESERVE_MIN")]
    pub fee_reserve_min: u64,
    // TODO check if fee_percent is in range
}

impl LightningFeeConfig {
    pub const fn new(fee_percent: f32, fee_reserve_min: u64) -> Self {
        Self {
            fee_percent,
            fee_reserve_min,
        }
    }

    // pub fn from_env() -> Self {
    //     let fee_config_default = Self::default();

    //     Self {
    //         fee_percent: env_or_default("LIGHTNING_FEE_PERCENT", fee_config_default.fee_percent),
    //         fee_reserve_min: env_or_default(
    //             "LIGHTNING_RESERVE_FEE_MIN",
    //             fee_config_default.fee_reserve_min,
    //         ),
    //     }
    // }
}

impl From<(f32, u64)> for LightningFeeConfig {
    fn from(tuple: (f32, u64)) -> Self {
        Self {
            fee_percent: tuple.0,
            fee_reserve_min: tuple.1,
        }
    }
}

// fn env_or_default<T: std::str::FromStr>(key: &str, default: T) -> T {
//     env::var(key)
//         .ok()
//         .and_then(|v| v.parse().ok())
//         .unwrap_or(default)
// }

impl Default for LightningFeeConfig {
    fn default() -> Self {
        Self {
            fee_percent: 1.0,
            fee_reserve_min: 4000,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_config() {
        dotenvy::dotenv().ok();

        // env::set_var("MINT_LIGHTNING_BACKEND_TYPE", "Lnd");
        // env::set_var("MINT_LIGHTNING_BACKEND_MACAROON_PATH","/Users/steffen/.polar/networks/1/volumes/lnd/alice/data/chain/bitcoin/regtest/admin.macaroon");
        // env::set_var(
        //     "MINT_LIGHTNING_BACKEND_TLS_CERT_PATH",
        //     "/Users/steffen/.polar/networks/1/volumes/lnd/alice/tls.cert",
        // );
        // env::set_var(
        //     "MINT_LIGHTNING_BACKEND_GRPC_HOST",
        //     "https://localhost:10001",
        // );

        let config = MintConfig::read_config_with_defaults();
        println!("{:?}", config);
    }
}
