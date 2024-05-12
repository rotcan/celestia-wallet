use crate::state::Coin;
use cosmos_sdk_proto::cosmos::bank::v1beta1::{query_client::QueryClient as BankClient,QueryAllBalancesRequest,QueryBalanceRequest};
use cosmos_sdk_proto::cosmos::auth::v1beta1::{query_client::QueryClient as AuthClient,BaseAccount,QueryAccountRequest,};
use tonic::transport::channel::Endpoint;
use cosmos_sdk_proto::cosmos::vesting::v1beta1::ContinuousVestingAccount;
use crate::error::QueryError;
use prost::Message;
use cosmrs::{Any};



pub enum AccountType{
    BaseAccount(BaseAccount),
    ContinuousVestingAccount(ContinuousVestingAccount),
}

pub fn convert_account_type(account: &Any)->Result<AccountType,QueryError>{
    match account.type_url.as_str() {
        "/cosmos.auth.v1beta1.BaseAccount" => Ok(AccountType::BaseAccount(BaseAccount::decode(
            account.value.as_slice(),
        )?)),
        "/cosmos.vesting.v1beta1.ContinuousVestingAccount" => {
            Ok(AccountType::ContinuousVestingAccount(
                ContinuousVestingAccount::decode(account.value.as_slice())?,
            ))
        }
        _ => Err(QueryError::UnknownAccountType),
    }
}

pub struct AccountQuery{}

impl AccountQuery{

    pub async fn get_account(address_id: String, grpc_url: &str,)->Option<Any>{
        let mut query_client=AuthClient::connect(Endpoint::from_shared(grpc_url.to_string()).unwrap()).await.unwrap();
        let account_request=QueryAccountRequest{
            address: address_id,
        };
        let result=query_client.account(account_request).await.unwrap();
        result.get_ref().account.clone()
    }
    
    pub async fn get_all_balances(account_id: String, grpc_url: &str,
    )->Vec<Coin>{
        let mut query_client=BankClient::connect(Endpoint::from_shared(grpc_url.to_string()).unwrap()).await.unwrap();
        let balance_request=QueryAllBalancesRequest{
            address: account_id,
            pagination: None,
        };
        let result=query_client.all_balances(balance_request).await.unwrap();
        result.get_ref().balances.clone()
    }

    pub async fn get_coin_balance(account_id: String,denom: String, grpc_url: &str,
    )->Option<Coin>{
        let mut query_client=BankClient::connect(Endpoint::from_shared(grpc_url.to_string()).unwrap()).await.unwrap();
        let balance_request=QueryBalanceRequest{
            address: account_id,
            denom,
        };
        let result=query_client.balance(balance_request).await.unwrap();
        result.get_ref().balance.clone()
    }
}