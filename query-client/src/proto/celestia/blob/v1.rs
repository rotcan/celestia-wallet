/// MsgPayForBlobs pays for the inclusion of a blob in the block.
#[derive(Clone, PartialEq, ::prost::Message, )]
pub struct MsgPayForBlobs {
    #[prost(string, tag="1")]
    pub signer: ::prost::alloc::string::String,
    /// namespaces is a list of namespaces that the blobs are associated with. A
    /// namespace is a byte slice of length 29 where the first byte is the
    /// namespaceVersion and the subsequent 28 bytes are the namespaceId.
    #[prost(bytes="vec", repeated, tag="2")]
    pub namespaces: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, repeated, tag="3")]
    pub blob_sizes: ::prost::alloc::vec::Vec<u32>,
    /// share_commitments is a list of share commitments (one per blob).
    #[prost(bytes="vec", repeated, tag="4")]
    pub share_commitments: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// share_versions are the versions of the share format that the blobs
    /// associated with this message should use when included in a block. The
    /// share_versions specified must match the share_versions used to generate the
    /// share_commitment in this message.
    #[prost(uint32, repeated, tag="8")]
    pub share_versions: ::prost::alloc::vec::Vec<u32>,
}


macro_rules! impl_name {
    ($type:ty, $package:expr, $name:expr) => {
        impl prost::Name for $type {
            const NAME: &'static str = $name;
            const PACKAGE: &'static str = $package;
        }
    };
}

impl_name!(
    //cosmos::upgrade::v1beta1::SoftwareUpgradeProposal,
    MsgPayForBlobs,
    "celestia.blob.v1",
    "MsgPayForBlobs"
);

/// MsgPayForBlobsResponse describes the response returned after the submission
/// of a PayForBlobs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPayForBlobsResponse {
}
/// Generated client implementations.
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Msg defines the blob Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        // #[must_use]
        // pub fn send_gzip(mut self) -> Self {
        //     self.inner = self.inner.send_gzip();
        //     self
        // }
        // /// Enable decompressing responses with `gzip`.
        // #[must_use]
        // pub fn accept_gzip(mut self) -> Self {
        //     self.inner = self.inner.accept_gzip();
        //     self
        // }
        /// PayForBlobs allows the user to pay for the inclusion of one or more blobs
        pub async fn pay_for_blobs(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPayForBlobs>,
        ) -> Result<tonic::Response<super::MsgPayForBlobsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/celestia.blob.v1.Msg/PayForBlobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}

