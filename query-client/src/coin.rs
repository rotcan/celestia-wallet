use cosmos_sdk_proto::cosmos::bank::v1beta1::{QueryDenomMetadataRequest,query_client::QueryClient as BankClient};
use crate::state::ProtoMetadata;
use tonic::transport::Endpoint;

pub struct CoinQuery{}

impl CoinQuery{
    pub async fn get_coin_info(coin_denom: String, grpc_url: &str,)->Option<ProtoMetadata>{
        let mut query_client=BankClient::connect(Endpoint::from_shared(grpc_url.to_string()).unwrap()).await.unwrap();
        let coin_request=QueryDenomMetadataRequest{
            denom: coin_denom
        };
        let result=query_client.denom_metadata(coin_request).await.unwrap();
        result.get_ref().metadata.clone()
    }
}