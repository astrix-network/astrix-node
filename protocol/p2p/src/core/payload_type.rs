use crate::pb::astrixd_message::Payload as AstrixdMessagePayload;

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum AstrixdMessagePayloadType {
    Addresses = 0,
    Block,
    Transaction,
    BlockLocator,
    RequestAddresses,
    RequestRelayBlocks,
    RequestTransactions,
    IbdBlock,
    InvRelayBlock,
    InvTransactions,
    Ping,
    Pong,
    Verack,
    Version,
    TransactionNotFound,
    Reject,
    PruningPointUtxoSetChunk,
    RequestIbdBlocks,
    UnexpectedPruningPoint,
    IbdBlockLocator,
    IbdBlockLocatorHighestHash,
    RequestNextPruningPointUtxoSetChunk,
    DonePruningPointUtxoSetChunks,
    IbdBlockLocatorHighestHashNotFound,
    BlockWithTrustedData,
    DoneBlocksWithTrustedData,
    RequestPruningPointAndItsAnticone,
    BlockHeaders,
    RequestNextHeaders,
    DoneHeaders,
    RequestPruningPointUtxoSet,
    RequestHeaders,
    RequestBlockLocator,
    PruningPoints,
    RequestPruningPointProof,
    PruningPointProof,
    Ready,
    BlockWithTrustedDataV4,
    TrustedData,
    RequestIbdChainBlockLocator,
    IbdChainBlockLocator,
    RequestAntipast,
    RequestNextPruningPointAndItsAnticoneBlocks,
}

impl From<&AstrixdMessagePayload> for AstrixdMessagePayloadType {
    fn from(payload: &AstrixdMessagePayload) -> Self {
        match payload {
            AstrixdMessagePayload::Addresses(_) => AstrixdMessagePayloadType::Addresses,
            AstrixdMessagePayload::Block(_) => AstrixdMessagePayloadType::Block,
            AstrixdMessagePayload::Transaction(_) => AstrixdMessagePayloadType::Transaction,
            AstrixdMessagePayload::BlockLocator(_) => AstrixdMessagePayloadType::BlockLocator,
            AstrixdMessagePayload::RequestAddresses(_) => AstrixdMessagePayloadType::RequestAddresses,
            AstrixdMessagePayload::RequestRelayBlocks(_) => AstrixdMessagePayloadType::RequestRelayBlocks,
            AstrixdMessagePayload::RequestTransactions(_) => AstrixdMessagePayloadType::RequestTransactions,
            AstrixdMessagePayload::IbdBlock(_) => AstrixdMessagePayloadType::IbdBlock,
            AstrixdMessagePayload::InvRelayBlock(_) => AstrixdMessagePayloadType::InvRelayBlock,
            AstrixdMessagePayload::InvTransactions(_) => AstrixdMessagePayloadType::InvTransactions,
            AstrixdMessagePayload::Ping(_) => AstrixdMessagePayloadType::Ping,
            AstrixdMessagePayload::Pong(_) => AstrixdMessagePayloadType::Pong,
            AstrixdMessagePayload::Verack(_) => AstrixdMessagePayloadType::Verack,
            AstrixdMessagePayload::Version(_) => AstrixdMessagePayloadType::Version,
            AstrixdMessagePayload::TransactionNotFound(_) => AstrixdMessagePayloadType::TransactionNotFound,
            AstrixdMessagePayload::Reject(_) => AstrixdMessagePayloadType::Reject,
            AstrixdMessagePayload::PruningPointUtxoSetChunk(_) => AstrixdMessagePayloadType::PruningPointUtxoSetChunk,
            AstrixdMessagePayload::RequestIbdBlocks(_) => AstrixdMessagePayloadType::RequestIbdBlocks,
            AstrixdMessagePayload::UnexpectedPruningPoint(_) => AstrixdMessagePayloadType::UnexpectedPruningPoint,
            AstrixdMessagePayload::IbdBlockLocator(_) => AstrixdMessagePayloadType::IbdBlockLocator,
            AstrixdMessagePayload::IbdBlockLocatorHighestHash(_) => AstrixdMessagePayloadType::IbdBlockLocatorHighestHash,
            AstrixdMessagePayload::RequestNextPruningPointUtxoSetChunk(_) => {
                AstrixdMessagePayloadType::RequestNextPruningPointUtxoSetChunk
            }
            AstrixdMessagePayload::DonePruningPointUtxoSetChunks(_) => AstrixdMessagePayloadType::DonePruningPointUtxoSetChunks,
            AstrixdMessagePayload::IbdBlockLocatorHighestHashNotFound(_) => {
                AstrixdMessagePayloadType::IbdBlockLocatorHighestHashNotFound
            }
            AstrixdMessagePayload::BlockWithTrustedData(_) => AstrixdMessagePayloadType::BlockWithTrustedData,
            AstrixdMessagePayload::DoneBlocksWithTrustedData(_) => AstrixdMessagePayloadType::DoneBlocksWithTrustedData,
            AstrixdMessagePayload::RequestPruningPointAndItsAnticone(_) => AstrixdMessagePayloadType::RequestPruningPointAndItsAnticone,
            AstrixdMessagePayload::BlockHeaders(_) => AstrixdMessagePayloadType::BlockHeaders,
            AstrixdMessagePayload::RequestNextHeaders(_) => AstrixdMessagePayloadType::RequestNextHeaders,
            AstrixdMessagePayload::DoneHeaders(_) => AstrixdMessagePayloadType::DoneHeaders,
            AstrixdMessagePayload::RequestPruningPointUtxoSet(_) => AstrixdMessagePayloadType::RequestPruningPointUtxoSet,
            AstrixdMessagePayload::RequestHeaders(_) => AstrixdMessagePayloadType::RequestHeaders,
            AstrixdMessagePayload::RequestBlockLocator(_) => AstrixdMessagePayloadType::RequestBlockLocator,
            AstrixdMessagePayload::PruningPoints(_) => AstrixdMessagePayloadType::PruningPoints,
            AstrixdMessagePayload::RequestPruningPointProof(_) => AstrixdMessagePayloadType::RequestPruningPointProof,
            AstrixdMessagePayload::PruningPointProof(_) => AstrixdMessagePayloadType::PruningPointProof,
            AstrixdMessagePayload::Ready(_) => AstrixdMessagePayloadType::Ready,
            AstrixdMessagePayload::BlockWithTrustedDataV4(_) => AstrixdMessagePayloadType::BlockWithTrustedDataV4,
            AstrixdMessagePayload::TrustedData(_) => AstrixdMessagePayloadType::TrustedData,
            AstrixdMessagePayload::RequestIbdChainBlockLocator(_) => AstrixdMessagePayloadType::RequestIbdChainBlockLocator,
            AstrixdMessagePayload::IbdChainBlockLocator(_) => AstrixdMessagePayloadType::IbdChainBlockLocator,
            AstrixdMessagePayload::RequestAntipast(_) => AstrixdMessagePayloadType::RequestAntipast,
            AstrixdMessagePayload::RequestNextPruningPointAndItsAnticoneBlocks(_) => {
                AstrixdMessagePayloadType::RequestNextPruningPointAndItsAnticoneBlocks
            }
        }
    }
}
