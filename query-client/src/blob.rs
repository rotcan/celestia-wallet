use celestia_types::{blob::Commitment,Blob, nmt::Namespace};
use cosmrs::{tx::Msg,ErrorReport,Result,AccountId};
// use prost::Name;
use crate::error::QueryError;

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct MsgPayForBlobs {
    /// Sender's address.
    pub signer: AccountId,

    /// Recipient's address.
    pub namespaces: Vec<Namespace>,

    /// Amount to send
    pub blob_sizes: Vec<u32>,

    pub share_commitments: Vec<Commitment>,

    pub share_versions: Vec<u8>,
}
 
impl Msg for MsgPayForBlobs {
    type Proto = crate::proto::celestia::blob::v1::MsgPayForBlobs;    
}

impl TryFrom<crate::proto::celestia::blob::v1::MsgPayForBlobs> for MsgPayForBlobs {
    type Error = ErrorReport;

    fn try_from(proto: crate::proto::celestia::blob::v1::MsgPayForBlobs) -> Result<MsgPayForBlobs> {
        MsgPayForBlobs::try_from(&proto)
    }
}

impl TryFrom<&crate::proto::celestia::blob::v1::MsgPayForBlobs> for MsgPayForBlobs {
    type Error = ErrorReport;

    fn try_from(proto: &crate::proto::celestia::blob::v1::MsgPayForBlobs) -> Result<MsgPayForBlobs> {
        Ok(MsgPayForBlobs {
            signer: proto.signer.parse()?,
            namespaces: proto.namespaces.iter().map(|m| Namespace::from_raw(m).unwrap()).collect::<Vec<Namespace>>(),
            blob_sizes: proto.blob_sizes.iter().map(|m| *m).collect::<Vec<u32>>(),
            share_commitments: proto.share_commitments.iter().map(|m| Commitment(m.clone().as_slice().try_into().unwrap())).collect::<Vec<Commitment>>(),
            share_versions: proto.share_versions.iter().flat_map(|val| val.to_be_bytes()).collect(),
                // .amount
                // .iter()
                // .map(TryFrom::try_from)
                // .collect::<Result<_, _>>()?,
        })
    }
}

impl From<MsgPayForBlobs> for crate::proto::celestia::blob::v1::MsgPayForBlobs {
    fn from(coin: MsgPayForBlobs) -> crate::proto::celestia::blob::v1::MsgPayForBlobs {
        crate::proto::celestia::blob::v1::MsgPayForBlobs::from(&coin)
    }
}

impl From<&MsgPayForBlobs> for crate::proto::celestia::blob::v1::MsgPayForBlobs {
    fn from(msg: &MsgPayForBlobs) -> crate::proto::celestia::blob::v1::MsgPayForBlobs {
        crate::proto::celestia::blob::v1::MsgPayForBlobs {
            signer: msg.signer.to_string(),
            namespaces: msg.namespaces.iter().map(|m| m.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>(),
            blob_sizes: msg.blob_sizes.iter().map(|m| *m).collect::<Vec<u32>>(),
            share_commitments: msg.share_commitments.iter().map(|m| m.0.to_vec()).collect::<Vec<Vec<u8>>>() ,
            share_versions: msg.share_versions.iter().map(|m| *m as u32).collect::<Vec<u32>>(),
            // msg.amount.iter().map(Into::into).collect(),
        }
    }
}

impl From<&Blob> for crate::proto::tendermint::types::types::Blob{
    fn from(blob: &Blob)->crate::proto::tendermint::types::types::Blob{
        crate::proto::tendermint::types::types::Blob{
            namespace_id: blob.namespace.id().to_vec(),
            data: blob.data.clone(),
            share_version: blob.share_version as u32,
            namespace_version: blob.namespace.version() as u32,
        }
    }
}

fn get_blob(namespace: &str, data: Vec<u8>)->Blob{

    let namespace_bytes=namespace.as_bytes();
    //println!("namespace_bytes={:?}",namespace_bytes);
    let my_namespace = Namespace::new_v0(&namespace_bytes).expect("Invalid namespace");
    let blob = Blob::new(my_namespace, data)
        .expect("Failed to create a blob");
    blob
}


pub async fn buy_blob(signer: &mut crate::tx::CosmosSigner,namespace: &str, 
    data: Vec<u8>, gas: Option<cosmrs::Gas>)->Result<Option<String>,QueryError>{
    // let mut query_client=MsgClient::connect(Endpoint::from_shared(grpc_url.to_string()).unwrap()).await.unwrap();
    // println!("query_client={:?}",query_client);
    let blob=get_blob(namespace,data);
   
    
    let request=MsgPayForBlobs{
        signer: signer.account_id.clone(), //AccountId::from_str(&signer).unwrap(),
        namespaces: vec![blob.namespace],
        //no of blobs
        blob_sizes: vec![blob.data.len().try_into().unwrap()],
        share_commitments: vec![blob.commitment],
        share_versions: vec![blob.share_version],
    };
    //let msg_any: Any=request.to_any().unwrap();
    //println!("msg={:?}",msg_any);
    
    Ok(crate::tx::buy_blob_tx(signer,request,vec![blob],None,gas).await?)
    // let result=query_client.pay_for_blobs(request).await.unwrap();
    // println!("result={:?}",result);
}

pub async fn buy_blob_gas_estimate(signer: &mut crate::tx::CosmosSigner,namespace: &str, 
    data: Vec<u8>)->Result<cosmrs::Gas,QueryError>{
        let blob=get_blob(namespace,data);
   
    
    let request=MsgPayForBlobs{
        signer: signer.account_id.clone(), //AccountId::from_str(&signer).unwrap(),
        namespaces: vec![blob.namespace],
        //no of blobs
        blob_sizes: vec![blob.data.len().try_into().unwrap()],
        share_commitments: vec![blob.commitment],
        share_versions: vec![blob.share_version],
    };
    
    Ok(crate::tx::estimate_gas_blob_tx(signer,request,vec![blob],None).await?)
}