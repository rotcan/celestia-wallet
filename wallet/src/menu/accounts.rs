use crate::menu::settings::{GlobalConfig,BECH32_ACCOUNT_ADDRESS_PREFIX};
use crate::menu::user::{User,UserConfig};
use serde::{Serialize,Deserialize};
use rust_keyring::secp256k1_key;
use rust_keyring::state::{FileCredential,EXTENSION};
use query_client::{accounts::AccountQuery,coin::CoinQuery};
//use query_client::state::Coin;
use query_client::tx::{CosmosSigner,TxList,TxnListResponse};
use crate::utils::CoinMetadata;
use crate::error::WalletError;

pub type Coin = query_client::state::Coin;
//Get List of accounts

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Account{
    pubkey: Vec<u8>,
    private_key: Vec<u8>,
    pub account_id: String,
    pub value: String,
    pub title: String,
    dir: String,
    key: String,
}



impl Account{

    pub fn get_value(&self)->String{
        self.value.clone()
    }
    
    pub fn get_title(&self)->String{
        self.title.clone()
    }

    pub fn set_title(&self)->String{
        self.title.clone()
    }

    pub fn value_match(&self,value: &str)->bool{
        if &self.value.as_str().replace(EXTENSION,"") == value {
            return true;
        };
        false
    }

    pub fn load( dir: &str, key: &str, password: &str)->Result<Account,WalletError>{
        let entry=FileCredential::get_entry_from_file(dir,key,Some(password.to_string()),true).unwrap();
        let credential: &FileCredential = entry
        .get_credential()
        .downcast_ref()
        .expect("Not a file credential");
        Self::create_account_from_credential(&credential, key.to_owned())
    }

    // pub fn new_from_private_key(config: &GlobalConfig,name: &str, private_key: &str, 
    //     password: &str)->Result<Account,Box<dyn Error>>{
    // }

    fn next_private_key(serial_no: u64,config: &GlobalConfig, seed: Vec<u8>)->Vec<u8>{
        //let password=user.get_password()?;
        let derivation_path=format!("{0}{1}",config.derivation_path,serial_no);
        secp256k1_key::derive_private_key(seed,
            &derivation_path)
    }
    //create new
    pub fn new_hd_account(user_config: &UserConfig, config: &GlobalConfig, password: &str, seed: Vec<u8>,
    name: Option<String>)
    ->Result<Account,WalletError>{
        
        let serial_no= user_config.user_settings.key_serial_no;
        let name=name.unwrap_or(format!("account-{0}",serial_no));
        // //let password=user.get_password()?;
        // let derivation_path=format!("{0}{1}",config.derivation_path,serial_no);
        let private_key=Self::next_private_key(serial_no,config,seed);
        Self::new_entry(config,&name,password,private_key)
    }

    pub fn get_new_hd_account_address(user_config: &UserConfig, config: &GlobalConfig, seed: Vec<u8>)
    ->Result<(String,String),WalletError>{
        
        let serial_no= user_config.user_settings.key_serial_no;
        //let password=user.get_password()?;
        let name=format!("account-{0}",serial_no);
        let private_key=Self::next_private_key(serial_no,config,seed);
        let pubkey=secp256k1_key::get_public_key(private_key);
        Ok((name,secp256k1_key::get_account_id(BECH32_ACCOUNT_ADDRESS_PREFIX,&pubkey)))
    }

    fn new_entry(config: &GlobalConfig, name: &str, password: &str, private_key: Vec<u8>)->Result<Account,WalletError>{
        let entry=secp256k1_key::create_key(&config.accounts_path,name,password,private_key,true).unwrap();
        let credential: &FileCredential = entry
        .get_credential()
        .downcast_ref()
        .expect("Not a file credential");
        Self::create_account_from_credential(credential,name.to_owned())
    }
 

    pub fn new_account_by_private_key(config: &GlobalConfig, password: &str, 
        private_key: &str, name: &str)->Result<Account,WalletError>{
        let private_key_bytes=hex::decode(private_key)?;
        Self::new_entry(config,name,password,private_key_bytes.try_into().unwrap())
    }

    fn create_account_from_credential(credential: &FileCredential,key: String)->Result<Account,WalletError>{
        let metadata=credential.decode_data()?;
        let private_key=metadata.get_item();
        let pubkey=secp256k1_key::get_public_key(private_key.clone());
        let account_id=secp256k1_key::get_account_id(BECH32_ACCOUNT_ADDRESS_PREFIX,&pubkey);
        Ok(Account{
            pubkey: pubkey.to_bytes(),
            private_key,
            account_id,
            value: key.clone(),
            dir: credential.get_dir(),
            key: credential.get_key(),
            title: key.replace(EXTENSION,""),
        })
    }

    pub fn remove(&self,password: &str)->Result<(),WalletError>{
        let entry=FileCredential::get_entry_from_file(&self.dir,&self.key,Some(password.to_owned()),true).unwrap();
        entry
        .delete_password()
        .expect("Couldn't delete after get_credential");
        Ok(())
    }

    pub async fn get_all_balances(&self, user: &User)->Result<Option<Vec<CoinMetadata>>,WalletError>{
        let coins=AccountQuery::get_all_balances(self.account_id.clone(),user.get_grpc_url().as_str()).await;
        Ok(self.get_coin_denoms(user,Some(coins)).await )
    }

    pub async fn get_txns(&self, user_config: &UserConfig)->Result<Vec<TxnListResponse>,WalletError>{
        let indexer_url=user_config.user_settings.get_current_network().get_indexer_url();
        let tx_list=TxList::new(indexer_url);
        Ok(tx_list.fetch_txns(&self.account_id).await?)
        //let indexer_url=.get_current_network().get_indexer_url();
        
    }

    pub async fn get_balance(&self, user: &User, denom: String,)->Option<Coin>{
        AccountQuery::get_coin_balance(self.account_id.clone(),denom,user.get_grpc_url().as_str()).await 
    }

    async fn get_coin_denoms(&self,user: &User, coins: Option<Vec<Coin>>)->Option<Vec<CoinMetadata>>{
        if coins.is_some() {
            let mut res: Vec<CoinMetadata>=Vec::new();
            for coin in coins.unwrap().iter(){
                let metadata=CoinQuery::get_coin_info(coin.denom.clone(),user.get_grpc_url().as_str()).await;
                let coin_metadata=metadata.map(|m| {
                    let mut cm= CoinMetadata::from(&m);
                    cm.update_balance(u128::from_str_radix(&coin.amount,10).unwrap());
                    cm
                });
                if coin_metadata.is_some() {
                    res.push(coin_metadata.unwrap())
                }
            }
            return Some(res);
        };
        None
    }

    //create sign
    pub fn get_signer(&self,user: &User, chain_id: String,address_prefix: &str,fee_denom: String,
        )->CosmosSigner{
            CosmosSigner::new(chain_id,address_prefix,fee_denom,user.get_grpc_url().as_str(),
            self.private_key.clone())
    }

    pub fn get_account_id(&self)->String{
        self.account_id.clone()
    }

    
}

// impl From<Account> for CosmosAccount{
//     fn from(a: &Account)->Self{
//         CosmosAccount{

//         }
//     }
// }