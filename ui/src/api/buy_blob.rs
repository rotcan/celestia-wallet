use cel_wallet::menu::user;
use crate::app::{ADDRESS_PREFIX,CHAIN_ID};
use crate::state::BuyBlobDetail;

pub struct BuyBlobPromise{
    tx_promise: Option<poll_promise::Promise<Result<Option<String>,cel_wallet::error::QueryError>>>,
    gas_promise: Option<poll_promise::Promise<Result<cel_wallet::CosmosGas,cel_wallet::error::QueryError>>>,
    tx_result: Option<String>,
    gas_result: Option<cel_wallet::CosmosGas>,
    error: Option<String>,
}

impl BuyBlobPromise{
    pub fn new()->Self{
        BuyBlobPromise{
            tx_promise: None,
            gas_promise: None,
            tx_result: None,
            gas_result: None,
            error: None,
        }
    }

    fn clear(&mut self){
        self.tx_promise=None;
        self.gas_promise=None;
        self.gas_result=None;
        self.tx_result=None;
    }
    pub fn init_tx(&mut self,user: &user::User,current_account_address: &String,
        buy_blob_detail: BuyBlobDetail,
        gas: Option<cel_wallet::CosmosGas>){
            
        if let Some(account) = user.search_account_by_name(current_account_address) {
            self.clear();

            let mut signer=account.get_signer(&user,CHAIN_ID.to_string(),
                ADDRESS_PREFIX,cel_wallet::tx::FEE_DENOM.to_string());
            
            let namespace = buy_blob_detail.namespace.clone();
            let data=buy_blob_detail.data.clone();
    
            self.tx_promise = Some(poll_promise::Promise::spawn_async(async move {
                cel_wallet::tx::buy_blob(&mut signer, namespace.as_str(),data,gas).await
            }));
        }
    }

    pub fn init_gas(&mut self,user: &user::User,current_account_address: &String,
        buy_blob_detail: BuyBlobDetail,){
        if let Some(account) = user.search_account_by_name(current_account_address) {
            self.clear();

            let mut signer=account.get_signer(&user,CHAIN_ID.to_string(),
                ADDRESS_PREFIX,cel_wallet::tx::FEE_DENOM.to_string());
            
            let namespace = buy_blob_detail.namespace.clone();
            let data=buy_blob_detail.data.clone();
    
            self.gas_promise = Some(poll_promise::Promise::spawn_async(async move {
                cel_wallet::tx::buy_blob_gas_estimate(&mut signer, namespace.as_str(),data).await
            }));
        }
    }

    
    pub fn check_tx_result(&mut self, ) 
    {
        match &self.tx_promise{
            Some(promise)=>{
                if let Some(result) = promise.ready() {
                    match result{
                        Ok(result)=>{
                            self.tx_result=result.clone();
                        },
                        Err(_)=>{
                            self.error=Some("Failed to get response".to_string());
                        }
                    }   
                    self.tx_promise=None;
                }
            },
            None=>{}
        }
    }

    pub fn check_gas_result(&mut self, ) 
    {
        match &self.gas_promise{
            Some(promise)=>{
                if let Some(result) = promise.ready() {
                    match result{
                        Ok(result)=>{
                            self.gas_result=Some(*result);
                        },
                        Err(_)=>{
                            self.error=Some("Failed to get response".to_string());
                        }
                    }   
                    self.gas_promise=None;
                }
            },
            None=>{}
        }
    }

    pub fn consume_tx_result(&mut self)->Option<String>{
        if self.tx_result.is_some(){
            let result=self.tx_result.clone();
            self.tx_result=None;
            return result;
        };
        return None;
    }

    pub fn consume_gas_result(&mut self)->Option<cel_wallet::CosmosGas>{
        if self.gas_result.is_some(){
            let result=self.gas_result.clone();
            self.gas_result=None;
            return result;
        };
        return None;
    }

    pub fn consume_failure(&mut self)->Option<String>{
        if self.error.is_some(){
            let error=self.error.clone();
            self.error=None;
            return error;
        };
        return None;
    }
}
