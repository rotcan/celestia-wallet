use cosmrs::tx::{Body, BodyBuilder,Fee, SignDoc, SignerInfo};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{BroadcastTxRequest,GetTxRequest,
      BroadcastMode,service_client::ServiceClient};
use cosmos_sdk_proto::cosmos::tx::v1beta1::{SimulateRequest,SimulateResponse};
use cosmos_sdk_proto::cosmos::base::abci::v1beta1::TxResponse;
use crate::accounts::{AccountQuery,AccountType,convert_account_type};
use crate::error::QueryError;
use cosmrs::{tx::Msg, AccountId,Any,Coin, crypto::{ PublicKey, secp256k1::SigningKey },bank::MsgSend} ;
use tonic::transport::channel::Endpoint;
//use tendermint_rpc::endpoint::broadcast::{tx_async, tx_commit, tx_sync};
use std::thread::sleep;
use std::time::Duration;
use cosmrs::tendermint::chain::id::Id;
use std::str::FromStr;
use std::ops::{DivAssign, MulAssign};
use std::fmt;
use prost::Message;
use serde::{Serialize,Deserialize};
use chrono::{DateTime,Utc};

pub struct CosmosTx{
    tx: BodyBuilder,
}

impl CosmosTx{
    pub fn build()->Self{
        CosmosTx{
            tx: BodyBuilder::new()
        }
    }

    pub fn memo(mut self, memo: &str) -> Self {
        self.tx.memo(memo.to_string());
        self
    }

    pub fn add_msg(mut self, msg: Any) -> Self {
        self.tx.msg(msg);
        self
    }


    pub fn finish(&self) -> Body {
        self.tx.finish()
    }
}


// pub struct CosmosTxResponse{
//     pub txhash: Option<String>,
//     pub error: Option<String>,
//     pub gas: Option<cosmrs::Gas>,
// }

// impl CosmosTxResponse{
//     pub fn new(hash: Option<String>, error: Option<String>, gas: Option<cosmrs::Gas>)->Self{
//         CosmosTxResponse{
//             txhash: hash,
//             error,
//             gas
//         }
//     }
// }


pub struct CosmosSigner{
    pub account_id: AccountId,
    pub private_key: SigningKey,
    pub public_key: PublicKey,
    pub chain_id: String,
    pub account_number: Option<u64>,
    pub sequence_id: Option<u64>,
    pub fee_denom: String,
    pub grpc_url: String,
    pub gas_price: u128,
}

impl fmt::Debug for CosmosSigner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "account_id={:?} \r\n,pubkey={:?} \r\n,chain_id={:?} \r\n,account_number={:?} \r\n,sequence_id={:?} \r\n,fee_denom={:?} \r\n,grpc={:?} \r\n,gas={:?}\r\n", 
        self.account_id,self.public_key,self.chain_id,self.account_number,self.sequence_id,self.fee_denom,self.grpc_url,self.gas_price)
    }
}

impl CosmosSigner{

    pub fn new(chain_id: String,address_prefix: &str,fee_denom: String,
        grpc_url: &str,
         private_key_bytes: Vec<u8>)->CosmosSigner{
        let private_key=SigningKey::from_slice(&private_key_bytes[..]).unwrap();
        let account_id=private_key.public_key().account_id(address_prefix).unwrap();
        let public_key=private_key.public_key();
        Self{
            chain_id,
            account_number: None,
            sequence_id: None,
            public_key,
            private_key,
            account_id,
            fee_denom,
            grpc_url: grpc_url.to_string(),
            gas_price: 25000,
        }
    }


    pub async fn sign_and_broadcast(
        &mut self,
        // signer: &CosmosSigner, chain_id: &str, 
        // gas_price: u64, denom: String,
        tx: CosmosTx,
        gas: Option<cosmrs::Gas>,
        mode: BroadcastMode,
        // grpc_url: &str
    ) -> Result<Option<TxResponse>, QueryError> {
        let payload = self.sign(tx,gas).await.unwrap();
        let mut service_client=ServiceClient::connect(Endpoint::from_shared(self.grpc_url.clone()).unwrap()).await.unwrap();
        let result= service_client.broadcast_tx(BroadcastTxRequest{
            tx_bytes: payload, mode: mode.into()
        }).await.unwrap();
        Ok(result.get_ref().tx_response.clone())
    }



    pub async fn sign_blob_and_broadcast(
        &mut self,
        // signer: &CosmosSigner, chain_id: &str, 
        // gas_price: u64, denom: String,
        tx: CosmosTx,
        blobs: Vec<celestia_types::Blob>,
        gas: Option<cosmrs::Gas>,
        mode: BroadcastMode,
        // grpc_url: &str
    ) -> Result<Option<TxResponse>, QueryError> {
        let payload = self.sign(tx,gas).await.unwrap();
        //create blob tx
        let blob_tx=crate::proto::tendermint::types::types::BlobTx{
            tx: payload,
            blobs: blobs.iter().map(|m| m.into()).collect::<Vec<crate::proto::tendermint::types::types::Blob>>(),
            type_id: "BLOB".parse().unwrap()
        };
        let blob_tx_vec=blob_tx.encode_to_vec();
        let mut service_client=ServiceClient::connect(Endpoint::from_shared(self.grpc_url.clone()).unwrap()).await.unwrap();
        let result= service_client.broadcast_tx(BroadcastTxRequest{
            tx_bytes: blob_tx_vec, mode: mode.into()
        }).await.unwrap();
        Ok(result.get_ref().tx_response.clone())
    }

    async fn update_signer(&mut self)->Result<(),QueryError>{
        let grpc_url=self.grpc_url.as_str();
        //sequence number
        let account=AccountQuery::get_account(self.account_id.to_string(),grpc_url).await;
        match account{
            Some(account)=>{
                match convert_account_type(&account)? {
                    AccountType::BaseAccount(account) => {
                        self.account_number=Some(account.account_number);
                        self.sequence_id=Some(account.sequence);
                        
                    },
                    AccountType::ContinuousVestingAccount(account) => {
                        let account = account
                        .base_vesting_account
                        .ok_or(QueryError::NoVestingBaseAccount)?
                        .base_account
                        .ok_or(QueryError::NoVestingBaseAccount)?;
                        self.account_number=Some(account.account_number);
                        self.sequence_id=Some(account.sequence);
                    },
                    //_=>{ return Err(QueryError::UnknownAccountType);},
                }
            },
            None=>{ return Err(QueryError::UnknownAccountType);},
        };

        Ok(())
    }

    pub async fn calculate_gas(&mut self,tx_body: &Body,)->Result<cosmrs::Gas,QueryError>{

        self.update_signer().await?;
        //let tx_body=cosmos_tx.finish();
        
        let auth_info = SignerInfo::single_direct(Some(self.public_key), self.sequence_id.unwrap()).auth_info(
            Fee::from_amount_and_gas(
                cosmrs::Coin {
                    amount: self.gas_price,
                    denom: self.fee_denom.parse()?,
                },
                100u64,
            ),
        );
        
        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &Id::from_str(self.chain_id.as_str())?,
            self.account_number.unwrap(),
        )?;

        let tx_raw = sign_doc.sign(&self.private_key)?;
        //simulate
        let tx = self.simulate(tx_raw.to_bytes()?).await?;
        
        if tx.gas_info.is_none() {
            return Err(QueryError::SimulationFailed);
        }

        let mut gas_info = tx.gas_info.unwrap_or_default().gas_used;
        gas_info.mul_assign(100u64 + u64::from(10u64));
        gas_info.div_assign(100);

        Ok(gas_info)
    }


    pub async fn sign(&mut self,
        // chain_id: &str, 
        cosmos_tx: CosmosTx,
        gas: Option<cosmrs::Gas>,
        //gas_price: u64, denom: String, 
    //    grpc_url: &str
    )->Result<Vec<u8>,QueryError>{
        
        self.update_signer().await?;

        let tx_body=cosmos_tx.finish();

        let gas_info = match gas {
            Some(gas)=>gas,
            None=> self.calculate_gas(&tx_body).await?,
        };

        let auth_info = SignerInfo::single_direct(Some(self.public_key), self.sequence_id.unwrap()).auth_info(
            Fee::from_amount_and_gas(
                cosmrs::Coin {
                    amount: self.gas_price,
                    denom: self.fee_denom.parse()?,
                },
                gas_info,
            ),
        );

        let sign_doc = SignDoc::new(
            &tx_body,
            &auth_info,
            &Id::from_str(self.chain_id.as_str())?,
            self.account_number.unwrap(),
        )?;

        Ok(sign_doc.sign(&self.private_key)?.to_bytes()?)
    }

    pub async fn simulate(&self, payload: Vec<u8>) -> Result<SimulateResponse, QueryError> {
        let mut service_client=ServiceClient::connect(Endpoint::from_shared(self.grpc_url.clone()).unwrap()).await.unwrap();
        #[allow(deprecated)]
        let request = SimulateRequest {
            tx: None,
            tx_bytes: payload,
        };

        let response=service_client.simulate(request).await.unwrap();
        Ok(response.get_ref().clone())
    }


    pub async fn poll_for_tx(&self, tx: TxResponse) -> Result<TxResponse, QueryError> {
        // let hash = match tx {
        //     CosmosTxResponse::Sync(tx) => tx.hash,
        //     CosmosTxResponse::Async(tx) => tx.hash,
        //     CosmosTxResponse::Commit(tx) => tx.hash,
        // };
        let hash=tx.txhash;
        for _ in 0..60 {
            let tx = self.get_tx(hash.as_str()).await;
            
            if tx.is_ok() {
                return Ok(tx.unwrap().unwrap());
            }
            sleep(Duration::from_secs(3));
        }

        Err(QueryError::TxTimeout)
    }

    async fn get_tx( &self,hash: &str) -> Result<Option<TxResponse>, QueryError> {

        let mut service_client=ServiceClient::connect(Endpoint::from_shared(self.grpc_url.clone()).unwrap()).await.unwrap();
        let result = service_client.get_tx(GetTxRequest{
            hash: hash.to_string(),
        }).await;
        result.map(|m| m.get_ref().tx_response.clone()).map_err(|_| QueryError::TxnNotFound)
    }

}

pub async fn send_gas_estimate(from: &mut CosmosSigner, to: &str, coin: Vec<Coin>,
    memo: Option<&str>)->Result<cosmrs::Gas,QueryError>{
        let msg: Any=MsgSend {
            from_address:from.account_id.clone(),
            to_address:  AccountId::from_str(to).unwrap(),
            amount: coin,
        }
        .to_any()?;
    //println!("msg={:?}",msg);
        let mut payload = CosmosTx::build()
            .add_msg(msg);

        if let Some(memo) = memo {
            payload = payload.memo(memo);
        }

        from.calculate_gas(&payload.finish()).await

    }

pub async fn send(from: &mut CosmosSigner, to: &str, coin: Vec<Coin>,
    memo: Option<&str>,gas: Option<cosmrs::Gas>)->Result<Option<String>,QueryError>{
        
        let msg: Any=MsgSend {
            from_address:from.account_id.clone(),
            to_address:  AccountId::from_str(to).unwrap(),
            amount: coin,
        }
        .to_any()?;
    //println!("msg={:?}",msg);
        let mut payload = CosmosTx::build()
            .add_msg(msg);

        if let Some(memo) = memo {
            payload = payload.memo(memo);
        }

        let tx=from.sign_and_broadcast(payload,gas,BroadcastMode::Sync).await?;
        let response=if let Some(ref tx) = tx {
            Some(from.poll_for_tx(tx.clone()).await?)
        }else{
            None
        };
        //println!("_response= {:?} ",response);

        response.map_or_else(
            || Err(QueryError::TxError("Missing".to_string())),
            |m| {
            if m.code>1 {
                return Err(QueryError::TxError(m.raw_log));
            }
            Ok(Some(m.txhash))}
        )
            
}

pub async fn estimate_gas_blob_tx(from: &mut CosmosSigner, msg:  crate::blob::MsgPayForBlobs,
    _blobs: Vec<celestia_types::Blob>,
    memo: Option<&str>,)->Result<cosmrs::Gas,QueryError>{
        let msg: Any=msg.to_any()?;
        let mut payload = CosmosTx::build()
            .add_msg(msg,);
        if let Some(memo) = memo {
            payload = payload.memo(memo);
        }

        from.calculate_gas(&payload.finish()).await
    }


pub async fn buy_blob_tx(from: &mut CosmosSigner, msg:  crate::blob::MsgPayForBlobs,
    blobs: Vec<celestia_types::Blob>,
    memo: Option<&str>,gas: Option<cosmrs::Gas>)->Result<Option<String>,QueryError>{
        
        let msg: Any=msg.to_any()?;
        let mut payload = CosmosTx::build()
            .add_msg(msg,);
        

        if let Some(memo) = memo {
            payload = payload.memo(memo);
        }

        let tx=from.sign_blob_and_broadcast(payload,blobs,gas,BroadcastMode::Sync).await.unwrap();
        println!("blob tx={:?}",tx);
        let _response=if let Some(ref tx) = tx {
            Some(from.poll_for_tx(tx.clone()).await.unwrap())
        }else{
            None
        };

        Ok(tx.map(|m| m.txhash))
    
}
 
#[derive(Serialize,Deserialize,Clone,Debug)]
pub struct TxnListResponse{
    pub id: u64,
    pub height: u64,
    pub position: u64,
    pub gas_wanted: u64,
    pub gas_used: u64,
    pub timeout_height: u64,
    pub events_count: u64,
    pub messages_count: u64,
    pub hash: String,
    pub fee: String,
    pub time: DateTime<Utc>,
    pub signers: Vec<String>,
    pub message_types: Vec<String>,
    pub status: String
}

pub struct TxList{
    indexer_url : String,
    page_size: u32,
    sort_order: String
}   

impl TxList{
    pub fn new(url: String)->Self{
        TxList{
            indexer_url: url,
            page_size: 10,
            sort_order: "desc".to_owned(),
        }
    }

    //https://api-arabica-11.celenium.io/v1/address/celestia1ded9jxnd6yv7g52sf72qq6z8qqynzjre50me5e/txs?limit=10&sort=desc
    pub async fn fetch_txns(&self,address: &str)->Result<Vec<TxnListResponse>,QueryError>{
        let url=format!("{}address/{}/txs?limit={}&sort={}",self.indexer_url,address,self.page_size,self.sort_order);
        let response=reqwest::get(&url).await?;
        let txns: Vec<TxnListResponse>=response.json::<Vec<TxnListResponse>>().await?;
        Ok(txns)
    }
}