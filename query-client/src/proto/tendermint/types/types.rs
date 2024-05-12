#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSet {
    #[prost(message, repeated, tag="1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    #[prost(message, optional, tag="2")]
    pub proposer: ::core::option::Option<Validator>,
    #[prost(int64, tag="3")]
    pub total_voting_power: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(bytes="vec", tag="1")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub pub_key: ::core::option::Option<super::crypto::PublicKey>,
    #[prost(int64, tag="3")]
    pub voting_power: i64,
    #[prost(int64, tag="4")]
    pub proposer_priority: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleValidator {
    #[prost(message, optional, tag="1")]
    pub pub_key: ::core::option::Option<super::crypto::PublicKey>,
    #[prost(int64, tag="2")]
    pub voting_power: i64,
}
/// PartsetHeader
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartSetHeader {
    #[prost(uint32, tag="1")]
    pub total: u32,
    #[prost(bytes="vec", tag="2")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Part {
    #[prost(uint32, tag="1")]
    pub index: u32,
    #[prost(bytes="vec", tag="2")]
    pub bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub proof: ::core::option::Option<super::crypto::Proof>,
}
/// BlockID
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockId {
    #[prost(bytes="vec", tag="1")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="2")]
    pub part_set_header: ::core::option::Option<PartSetHeader>,
}
// --------------------------------

/// Header defines the structure of a block header.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    /// basic block info
    #[prost(message, optional, tag="1")]
    pub version: ::core::option::Option<super::version::Consensus>,
    #[prost(string, tag="2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag="3")]
    pub height: i64,
    #[prost(message, optional, tag="4")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// prev block info
    #[prost(message, optional, tag="5")]
    pub last_block_id: ::core::option::Option<BlockId>,
    /// hashes of block data
    ///
    /// commit from validators from the last block
    #[prost(bytes="vec", tag="6")]
    pub last_commit_hash: ::prost::alloc::vec::Vec<u8>,
    /// transactions
    #[prost(bytes="vec", tag="7")]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    /// hashes from the app output from the prev block
    ///
    /// validators for the current block
    #[prost(bytes="vec", tag="8")]
    pub validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// validators for the next block
    #[prost(bytes="vec", tag="9")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus params for current block
    #[prost(bytes="vec", tag="10")]
    pub consensus_hash: ::prost::alloc::vec::Vec<u8>,
    /// state after txs from the previous block
    #[prost(bytes="vec", tag="11")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    /// root hash of all results from the txs from the previous block
    #[prost(bytes="vec", tag="12")]
    pub last_results_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus info
    ///
    /// evidence included in the block
    #[prost(bytes="vec", tag="13")]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
    /// original proposer of the block
    #[prost(bytes="vec", tag="14")]
    pub proposer_address: ::prost::alloc::vec::Vec<u8>,
}
/// Data contains all the information needed for a consensus full node to
/// reconstruct an extended data square.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    /// Txs that will be applied to state in block.Height + 1 because deferred execution.
    /// This means that the block.AppHash of this block does not include these txs.
    /// NOTE: not all txs here are valid. We're just agreeing on the order first.
    #[prost(bytes="vec", repeated, tag="1")]
    pub txs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// SquareSize is the number of rows or columns in the original data square.
    #[prost(uint64, tag="5")]
    pub square_size: u64,
    /// Hash is the root of a binary Merkle tree where the leaves of the tree are
    /// the row and column roots of an extended data square. Hash is often referred
    /// to as the "data root".
    #[prost(bytes="vec", tag="6")]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// Blob (named after binary large object) is a chunk of data submitted by a user
/// to be published to the Celestia blockchain. The data of a Blob is published
/// to a namespace and is encoded into shares based on the format specified by
/// share_version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    #[prost(bytes="vec", tag="1")]
    pub namespace_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="3")]
    pub share_version: u32,
    #[prost(uint32, tag="4")]
    pub namespace_version: u32,
}
/// Vote represents a prevote, precommit, or commit vote from validators for
/// consensus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vote {
    #[prost(enumeration="SignedMsgType", tag="1")]
    pub r#type: i32,
    #[prost(int64, tag="2")]
    pub height: i64,
    #[prost(int32, tag="3")]
    pub round: i32,
    /// zero if vote is nil.
    #[prost(message, optional, tag="4")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag="5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="6")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(int32, tag="7")]
    pub validator_index: i32,
    #[prost(bytes="vec", tag="8")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Commit contains the evidence that a block was committed by a set of validators.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commit {
    #[prost(int64, tag="1")]
    pub height: i64,
    #[prost(int32, tag="2")]
    pub round: i32,
    #[prost(message, optional, tag="3")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, repeated, tag="4")]
    pub signatures: ::prost::alloc::vec::Vec<CommitSig>,
}
/// CommitSig is a part of the Vote included in a Commit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitSig {
    #[prost(enumeration="BlockIdFlag", tag="1")]
    pub block_id_flag: i32,
    #[prost(bytes="vec", tag="2")]
    pub validator_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="4")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    #[prost(enumeration="SignedMsgType", tag="1")]
    pub r#type: i32,
    #[prost(int64, tag="2")]
    pub height: i64,
    #[prost(int32, tag="3")]
    pub round: i32,
    #[prost(int32, tag="4")]
    pub pol_round: i32,
    #[prost(message, optional, tag="5")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(message, optional, tag="6")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(bytes="vec", tag="7")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedHeader {
    #[prost(message, optional, tag="1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag="2")]
    pub commit: ::core::option::Option<Commit>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LightBlock {
    #[prost(message, optional, tag="1")]
    pub signed_header: ::core::option::Option<SignedHeader>,
    #[prost(message, optional, tag="2")]
    pub validator_set: ::core::option::Option<ValidatorSet>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockMeta {
    #[prost(message, optional, tag="1")]
    pub block_id: ::core::option::Option<BlockId>,
    #[prost(int64, tag="2")]
    pub block_size: i64,
    #[prost(message, optional, tag="3")]
    pub header: ::core::option::Option<Header>,
    #[prost(int64, tag="4")]
    pub num_txs: i64,
}
/// TxProof represents a Merkle proof of the presence of a transaction in the
/// Merkle tree.
///
/// Note: TxProof is not used in celestia-core because of modifications to the
/// data root. In a normal Cosmos chain, the data root is the root of a Merkle
/// tree of transactions in the block. However, in Celestia the data root is the
/// root of the row and column roots in the extended data square. See
/// <https://github.com/celestiaorg/celestia-app/blob/852a229f11f0f269021b36f7621609f432bb858b/pkg/da/data_availability_header.go>
/// for more details. Therefore, TxProof isn't sufficient to prove the existence
/// of a transaction in a Celestia block and ShareProof was defined instead. See
/// ShareProof for more details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TxProof {
    #[prost(bytes="vec", tag="1")]
    pub root_hash: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="3")]
    pub proof: ::core::option::Option<super::crypto::Proof>,
}
/// IndexWrapper adds index metadata to a transaction. This is used to track
/// transactions that pay for blobs, and where the blobs start in the square.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexWrapper {
    #[prost(bytes="vec", tag="1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, repeated, tag="2")]
    pub share_indexes: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, tag="3")]
    pub type_id: ::prost::alloc::string::String,
}
/// BlobTx wraps an encoded sdk.Tx with a second field to contain blobs of data.
/// The raw bytes of the blobs are not signed over, instead we verify each blob
/// using the relevant MsgPayForBlobs that is signed over in the encoded sdk.Tx.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobTx {
    #[prost(bytes="vec", tag="1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="2")]
    pub blobs: ::prost::alloc::vec::Vec<Blob>,
    #[prost(string, tag="3")]
    pub type_id: ::prost::alloc::string::String,
}
/// ShareProof is an NMT proof that a set of shares exist in a set of rows and a
/// Merkle proof that those rows exist in a Merkle tree with a given data root.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareProof {
    #[prost(bytes="vec", repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="2")]
    pub share_proofs: ::prost::alloc::vec::Vec<NmtProof>,
    #[prost(bytes="vec", tag="3")]
    pub namespace_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag="4")]
    pub row_proof: ::core::option::Option<RowProof>,
    #[prost(uint32, tag="5")]
    pub namespace_version: u32,
}
/// RowProof is a Merkle proof that a set of rows exist in a Merkle tree with a
/// given data root.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowProof {
    #[prost(bytes="vec", repeated, tag="1")]
    pub row_roots: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag="2")]
    pub proofs: ::prost::alloc::vec::Vec<super::crypto::Proof>,
    #[prost(bytes="vec", tag="3")]
    pub root: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="4")]
    pub start_row: u32,
    #[prost(uint32, tag="5")]
    pub end_row: u32,
}
/// NMTProof is a proof of a namespace.ID in an NMT.
/// In case this proof proves the absence of a namespace.ID
/// in a tree it also contains the leaf hashes of the range
/// where that namespace would be.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NmtProof {
    /// Start index of this proof.
    #[prost(int32, tag="1")]
    pub start: i32,
    /// End index of this proof.
    #[prost(int32, tag="2")]
    pub end: i32,
    /// Nodes that together with the corresponding leaf values can be used to
    /// recompute the root and verify this proof. Nodes should consist of the max
    /// and min namespaces along with the actual hash, resulting in each being 48
    /// bytes each
    #[prost(bytes="vec", repeated, tag="3")]
    pub nodes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// leafHash are nil if the namespace is present in the NMT. In case the
    /// namespace to be proved is in the min/max range of the tree but absent, this
    /// will contain the leaf hash necessary to verify the proof of absence. Leaf
    /// hashes should consist of the namespace along with the actual hash,
    /// resulting 40 bytes total.
    #[prost(bytes="vec", tag="4")]
    pub leaf_hash: ::prost::alloc::vec::Vec<u8>,
}
/// BlockIdFlag indicates which BlcokID the signature is for
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BlockIdFlag {
    Unknown = 0,
    Absent = 1,
    Commit = 2,
    Nil = 3,
}
/// SignedMsgType is a type of signed message in the consensus.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SignedMsgType {
    Unknown = 0,
    /// Votes
    Prevote = 1,
    Precommit = 2,
    /// Proposals
    Proposal = 32,
}
