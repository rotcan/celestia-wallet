pub mod tx;
pub mod state;
pub mod accounts;
pub mod error;
pub mod coin;
pub mod blob;
pub mod proto;


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

//Get balances
//Get transactions
//Do transaction
//Then look at blobs
#[cfg(test)]
mod tests {
    use super::*;
    use cometbft_rpc::client::{HttpClient,Client};
    use cosmos_sdk_proto::cosmos::bank::v1beta1::{query_client::QueryClient,QueryAllBalancesRequest};
    use tonic::transport::channel::Endpoint;

    //#[tokio::test]
    async fn test_rpc() {
        let client = HttpClient::new("https://rpc.celestia-arabica-11.com")
        .unwrap();

        let abci_info = client.abci_info()
            .await
            .unwrap();

        println!("Got ABCI info: {:?}", abci_info);
    }

    #[tokio::test]
    async fn test_grpc(){
        let mut query_client=QueryClient::connect(Endpoint::from_static("https://validator-1.celestia-arabica-11.com:9090")).await.unwrap();
        let balance_request=QueryAllBalancesRequest{
            address: "celestia1ded9jxnd6yv7g52sf72qq6z8qqynzjre50me5e".to_string(),
            pagination: None,
        };
        let result=query_client.all_balances(balance_request).await.unwrap();
        println!("result={:?}",result);
    }
}
