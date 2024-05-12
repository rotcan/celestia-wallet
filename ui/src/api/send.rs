use crate::state::SendDetail;
use cel_wallet::menu::user;
use crate::app::{ADDRESS_PREFIX,CHAIN_ID};

pub struct SendPromise{
    tx_promise: Option<poll_promise::Promise<Result<Option<String>,cel_wallet::error::QueryError>>>,
    gas_promise: Option<poll_promise::Promise<Result<cel_wallet::CosmosGas,cel_wallet::error::QueryError>>>,
    tx_result: Option<String>,
    gas_result: Option<cel_wallet::CosmosGas>,
    error: Option<String>,
}

impl SendPromise{
    pub fn new()->Self{
        SendPromise{
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
        receiver_detail: SendDetail,
        gas: Option<cel_wallet::CosmosGas>){
            
        if let Some(account) = user.search_account_by_name(current_account_address) {
            self.clear();

            let mut signer=account.get_signer(&user,CHAIN_ID.to_string(),
                ADDRESS_PREFIX,receiver_detail.denom.to_string());
            
            let to = receiver_detail.to.clone();
            let denom=receiver_detail.denom.clone();
            //u128::from_str_radix(&receiver_detail.amount,10).unwrap();
            let amount: u128 = crate::helper::convert_amount(receiver_detail.amount.as_str(),receiver_detail.exponent).unwrap_or(0);
            if amount>0 {
                println!("amount={}",amount);
                let coin=cel_wallet::CosmosCoin {
                    amount,
                    denom: denom.parse().unwrap(),
                };
                println!("coin to transfer = {:?}",coin);
                self.tx_promise = Some(poll_promise::Promise::spawn_async(async move {
                    cel_wallet::tx::send(&mut signer, to.as_str(),vec![coin],None,gas).await
                }));
            }
        }
    }

    pub fn init_gas(&mut self,user: &user::User,current_account_address: &String,
        receiver_detail: SendDetail,){
        if let Some(account) = user.search_account_by_name(current_account_address) {
            self.clear();

            let mut signer=account.get_signer(&user,CHAIN_ID.to_string(),
                ADDRESS_PREFIX,receiver_detail.denom.to_string());
            
            let to = receiver_detail.to.clone();
            let denom=receiver_detail.denom.clone();
            let amount: u128 = crate::helper::convert_amount(receiver_detail.amount.as_str(),receiver_detail.exponent).unwrap_or(0);
            if amount>0 {
                let coin=cel_wallet::CosmosCoin {
                    amount,
                    denom: denom.parse().unwrap(),
                };
                
                self.gas_promise = Some(poll_promise::Promise::spawn_async(async move {
                    cel_wallet::tx::send_gas_estimate(&mut signer, to.as_str(),vec![coin],None).await
                }));
            }
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
