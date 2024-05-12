use serde::{Serialize,Deserialize};
use dirs::home_dir;
use std::path::PathBuf;


pub const APP_NAME: &str="Celestia-app";
pub const KEY_NAME: &str="my-celes-key";
pub const BECH32_ACCOUNT_ADDRESS_PREFIX: &str="celestia";
pub const BECH32_ACCOUNT_PUBKEY_PREFIX: &str="celestiapub";
pub const DERIVATION_PATH: &str="m/44'/118'/0'/0/";
pub const DEFAULT_PASSWORD: &str="";
pub const WALLET_BASE_DIR_PATH: &str=".celestia-wallet";
pub const ACCOUNTS_DIR_PATH: &str="account";
pub const SETTINGS_FILE_PATH: &str="settings.toml";
pub const SEED_KEY: &str="seed";

#[derive(Debug,Clone,Serialize,Deserialize,)]
pub struct GlobalConfig{
    pub derivation_path: String,
    pub account_id: String,
    pub base_path: String,
    pub accounts_path: String,
    pub settings_path: String,
}



impl GlobalConfig{
 


    fn get_base_path()->PathBuf{
        let dir=home_dir();
        match dir{
            Some(dir)=>PathBuf::from(dir).join(WALLET_BASE_DIR_PATH),
            None=> {
                #[cfg(any(target_os = "android"))]
                return PathBuf::from("data/data/rotcan.cel_wallet_mobile/files".to_string());
                return PathBuf::from("./".to_string());
            },
        }
    }

    fn get_accounts_path()->String{
        GlobalConfig::get_base_path().join(ACCOUNTS_DIR_PATH).as_path().to_str().unwrap().to_owned()
    }

 
    fn get_settings_path()->String{
        GlobalConfig::get_base_path().join(SETTINGS_FILE_PATH).as_path().to_str().unwrap().to_owned()
    }
    
    pub fn default()->GlobalConfig{
        GlobalConfig{
            derivation_path: DERIVATION_PATH.to_owned(),
            account_id: BECH32_ACCOUNT_ADDRESS_PREFIX.to_owned(),
            base_path: GlobalConfig::get_base_path().as_path().to_str().unwrap().to_owned(),
            accounts_path: GlobalConfig::get_accounts_path(),
            settings_path: GlobalConfig::get_settings_path(),
        }
    }

}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Network{
    name: String,
    grpc_url: String,
    rpc_url: Option<String>,
    indexer_url: String,
}

impl Network{
    pub fn default()->Self{
        Self::new(
            "arabica".to_owned(),
            "https://validator-1.celestia-arabica-11.com:9090".to_owned(),
            Some("https://rpc.celestia-arabica-11.com".to_owned()),
            "https://api-arabica-11.celenium.io/v1/".to_owned()
        )
    }

    pub fn new(name: String, grpc_url: String, rpc_url: Option<String>,indexer_url: String)->Self{
        Network{
            name,
            grpc_url,
            rpc_url,
            indexer_url
        }
    }
    
    pub fn get_name(&self)->String{
        self.name.clone()
    }

    pub fn get_grpc_url(&self)->String{
        self.grpc_url.clone()
    }

    pub fn get_indexer_url(&self)->String{
        self.indexer_url.clone()
    }
}


