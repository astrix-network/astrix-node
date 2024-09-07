use crate::protowire::{astrixd_request, AstrixdRequest, AstrixdResponse};

impl From<astrixd_request::Payload> for AstrixdRequest {
    fn from(item: astrixd_request::Payload) -> Self {
        AstrixdRequest { id: 0, payload: Some(item) }
    }
}

impl AsRef<AstrixdRequest> for AstrixdRequest {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<AstrixdResponse> for AstrixdResponse {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub mod astrixd_request_convert {
    use crate::protowire::*;
    use astrix_rpc_core::{RpcError, RpcResult};

    impl_into_astrixd_request!(Shutdown);
    impl_into_astrixd_request!(SubmitBlock);
    impl_into_astrixd_request!(GetBlockTemplate);
    impl_into_astrixd_request!(GetBlock);
    impl_into_astrixd_request!(GetInfo);

    impl_into_astrixd_request!(GetCurrentNetwork);
    impl_into_astrixd_request!(GetPeerAddresses);
    impl_into_astrixd_request!(GetSink);
    impl_into_astrixd_request!(GetMempoolEntry);
    impl_into_astrixd_request!(GetMempoolEntries);
    impl_into_astrixd_request!(GetConnectedPeerInfo);
    impl_into_astrixd_request!(AddPeer);
    impl_into_astrixd_request!(SubmitTransaction);
    impl_into_astrixd_request!(GetSubnetwork);
    impl_into_astrixd_request!(GetVirtualChainFromBlock);
    impl_into_astrixd_request!(GetBlocks);
    impl_into_astrixd_request!(GetBlockCount);
    impl_into_astrixd_request!(GetBlockDagInfo);
    impl_into_astrixd_request!(ResolveFinalityConflict);
    impl_into_astrixd_request!(GetHeaders);
    impl_into_astrixd_request!(GetUtxosByAddresses);
    impl_into_astrixd_request!(GetBalanceByAddress);
    impl_into_astrixd_request!(GetBalancesByAddresses);
    impl_into_astrixd_request!(GetSinkBlueScore);
    impl_into_astrixd_request!(Ban);
    impl_into_astrixd_request!(Unban);
    impl_into_astrixd_request!(EstimateNetworkHashesPerSecond);
    impl_into_astrixd_request!(GetMempoolEntriesByAddresses);
    impl_into_astrixd_request!(GetCoinSupply);
    impl_into_astrixd_request!(Ping);
    impl_into_astrixd_request!(GetMetrics);
    impl_into_astrixd_request!(GetServerInfo);
    impl_into_astrixd_request!(GetSyncStatus);
    impl_into_astrixd_request!(GetDaaScoreTimestampEstimate);

    impl_into_astrixd_request!(NotifyBlockAdded);
    impl_into_astrixd_request!(NotifyNewBlockTemplate);
    impl_into_astrixd_request!(NotifyUtxosChanged);
    impl_into_astrixd_request!(NotifyPruningPointUtxoSetOverride);
    impl_into_astrixd_request!(NotifyFinalityConflict);
    impl_into_astrixd_request!(NotifyVirtualDaaScoreChanged);
    impl_into_astrixd_request!(NotifyVirtualChainChanged);
    impl_into_astrixd_request!(NotifySinkBlueScoreChanged);

    macro_rules! impl_into_astrixd_request {
        ($name:tt) => {
            paste::paste! {
                impl_into_astrixd_request_ex!(astrix_rpc_core::[<$name Request>],[<$name RequestMessage>],[<$name Request>]);
            }
        };
    }

    use impl_into_astrixd_request;

    macro_rules! impl_into_astrixd_request_ex {
        // ($($core_struct:ident)::+, $($protowire_struct:ident)::+, $($variant:ident)::+) => {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<&$core_struct> for astrixd_request::Payload {
                fn from(item: &$core_struct) -> Self {
                    Self::$variant(item.into())
                }
            }

            impl From<&$core_struct> for AstrixdRequest {
                fn from(item: &$core_struct) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<$core_struct> for astrixd_request::Payload {
                fn from(item: $core_struct) -> Self {
                    Self::$variant((&item).into())
                }
            }

            impl From<$core_struct> for AstrixdRequest {
                fn from(item: $core_struct) -> Self {
                    Self { id: 0, payload: Some((&item).into()) }
                }
            }

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&astrixd_request::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &astrixd_request::Payload) -> RpcResult<Self> {
                    if let astrixd_request::Payload::$variant(request) = item {
                        request.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&AstrixdRequest> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &AstrixdRequest) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("AstrixRequest".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }

            impl From<$protowire_struct> for AstrixdRequest {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(astrixd_request::Payload::$variant(item)) }
                }
            }

            impl From<$protowire_struct> for astrixd_request::Payload {
                fn from(item: $protowire_struct) -> Self {
                    astrixd_request::Payload::$variant(item)
                }
            }
        };
    }
    use impl_into_astrixd_request_ex;
}

pub mod astrixd_response_convert {
    use crate::protowire::*;
    use astrix_rpc_core::{RpcError, RpcResult};

    impl_into_astrixd_response!(Shutdown);
    impl_into_astrixd_response!(SubmitBlock);
    impl_into_astrixd_response!(GetBlockTemplate);
    impl_into_astrixd_response!(GetBlock);
    impl_into_astrixd_response!(GetInfo);
    impl_into_astrixd_response!(GetCurrentNetwork);

    impl_into_astrixd_response!(GetPeerAddresses);
    impl_into_astrixd_response!(GetSink);
    impl_into_astrixd_response!(GetMempoolEntry);
    impl_into_astrixd_response!(GetMempoolEntries);
    impl_into_astrixd_response!(GetConnectedPeerInfo);
    impl_into_astrixd_response!(AddPeer);
    impl_into_astrixd_response!(SubmitTransaction);
    impl_into_astrixd_response!(GetSubnetwork);
    impl_into_astrixd_response!(GetVirtualChainFromBlock);
    impl_into_astrixd_response!(GetBlocks);
    impl_into_astrixd_response!(GetBlockCount);
    impl_into_astrixd_response!(GetBlockDagInfo);
    impl_into_astrixd_response!(ResolveFinalityConflict);
    impl_into_astrixd_response!(GetHeaders);
    impl_into_astrixd_response!(GetUtxosByAddresses);
    impl_into_astrixd_response!(GetBalanceByAddress);
    impl_into_astrixd_response!(GetBalancesByAddresses);
    impl_into_astrixd_response!(GetSinkBlueScore);
    impl_into_astrixd_response!(Ban);
    impl_into_astrixd_response!(Unban);
    impl_into_astrixd_response!(EstimateNetworkHashesPerSecond);
    impl_into_astrixd_response!(GetMempoolEntriesByAddresses);
    impl_into_astrixd_response!(GetCoinSupply);
    impl_into_astrixd_response!(Ping);
    impl_into_astrixd_response!(GetMetrics);
    impl_into_astrixd_response!(GetServerInfo);
    impl_into_astrixd_response!(GetSyncStatus);
    impl_into_astrixd_response!(GetDaaScoreTimestampEstimate);

    impl_into_astrixd_notify_response!(NotifyBlockAdded);
    impl_into_astrixd_notify_response!(NotifyNewBlockTemplate);
    impl_into_astrixd_notify_response!(NotifyUtxosChanged);
    impl_into_astrixd_notify_response!(NotifyPruningPointUtxoSetOverride);
    impl_into_astrixd_notify_response!(NotifyFinalityConflict);
    impl_into_astrixd_notify_response!(NotifyVirtualDaaScoreChanged);
    impl_into_astrixd_notify_response!(NotifyVirtualChainChanged);
    impl_into_astrixd_notify_response!(NotifySinkBlueScoreChanged);

    impl_into_astrixd_notify_response!(NotifyUtxosChanged, StopNotifyingUtxosChanged);
    impl_into_astrixd_notify_response!(NotifyPruningPointUtxoSetOverride, StopNotifyingPruningPointUtxoSetOverride);

    macro_rules! impl_into_astrixd_response {
        ($name:tt) => {
            paste::paste! {
                impl_into_astrixd_response_ex!(astrix_rpc_core::[<$name Response>],[<$name ResponseMessage>],[<$name Response>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            paste::paste! {
                impl_into_astrixd_response_base!(astrix_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>],[<$protowire_name Response>]);
            }
        };
    }
    use impl_into_astrixd_response;

    macro_rules! impl_into_astrixd_response_base {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<$core_struct>> for $protowire_struct {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    item.as_ref().map_err(|x| (*x).clone()).into()
                }
            }

            impl From<RpcError> for $protowire_struct {
                fn from(item: RpcError) -> Self {
                    let x: RpcResult<&$core_struct> = Err(item);
                    x.into()
                }
            }

            impl From<$protowire_struct> for astrixd_response::Payload {
                fn from(item: $protowire_struct) -> Self {
                    astrixd_response::Payload::$variant(item)
                }
            }

            impl From<$protowire_struct> for AstrixdResponse {
                fn from(item: $protowire_struct) -> Self {
                    Self { id: 0, payload: Some(astrixd_response::Payload::$variant(item)) }
                }
            }
        };
    }
    use impl_into_astrixd_response_base;

    macro_rules! impl_into_astrixd_response_ex {
        ($core_struct:path, $protowire_struct:ident, $variant:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl From<RpcResult<&$core_struct>> for astrixd_response::Payload {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    astrixd_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<&$core_struct>> for AstrixdResponse {
                fn from(item: RpcResult<&$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl From<RpcResult<$core_struct>> for astrixd_response::Payload {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    astrixd_response::Payload::$variant(item.into())
                }
            }

            impl From<RpcResult<$core_struct>> for AstrixdResponse {
                fn from(item: RpcResult<$core_struct>) -> Self {
                    Self { id: 0, payload: Some(item.into()) }
                }
            }

            impl_into_astrixd_response_base!($core_struct, $protowire_struct, $variant);

            // ----------------------------------------------------------------------------
            // protowire to rpc_core
            // ----------------------------------------------------------------------------

            impl TryFrom<&astrixd_response::Payload> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &astrixd_response::Payload) -> RpcResult<Self> {
                    if let astrixd_response::Payload::$variant(response) = item {
                        response.try_into()
                    } else {
                        Err(RpcError::MissingRpcFieldError("Payload".to_string(), stringify!($variant).to_string()))
                    }
                }
            }

            impl TryFrom<&AstrixdResponse> for $core_struct {
                type Error = RpcError;
                fn try_from(item: &AstrixdResponse) -> RpcResult<Self> {
                    item.payload
                        .as_ref()
                        .ok_or(RpcError::MissingRpcFieldError("AstrixResponse".to_string(), "Payload".to_string()))?
                        .try_into()
                }
            }
        };
    }
    use impl_into_astrixd_response_ex;

    macro_rules! impl_into_astrixd_notify_response {
        ($name:tt) => {
            impl_into_astrixd_response!($name);

            paste::paste! {
                impl_into_astrixd_notify_response_ex!(astrix_rpc_core::[<$name Response>],[<$name ResponseMessage>]);
            }
        };
        ($core_name:tt, $protowire_name:tt) => {
            impl_into_astrixd_response!($core_name, $protowire_name);

            paste::paste! {
                impl_into_astrixd_notify_response_ex!(astrix_rpc_core::[<$core_name Response>],[<$protowire_name ResponseMessage>]);
            }
        };
    }
    use impl_into_astrixd_notify_response;

    macro_rules! impl_into_astrixd_notify_response_ex {
        ($($core_struct:ident)::+, $protowire_struct:ident) => {
            // ----------------------------------------------------------------------------
            // rpc_core to protowire
            // ----------------------------------------------------------------------------

            impl<T> From<Result<(), T>> for $protowire_struct
            where
                T: Into<RpcError>,
            {
                fn from(item: Result<(), T>) -> Self {
                    item
                        .map(|_| $($core_struct)::+{})
                        .map_err(|err| err.into()).into()
                }
            }

        };
    }
    use impl_into_astrixd_notify_response_ex;
}
