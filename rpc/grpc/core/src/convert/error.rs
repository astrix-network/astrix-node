use crate::from;
use crate::protowire;

// ----------------------------------------------------------------------------
// rpc_core to protowire
// ----------------------------------------------------------------------------

from!(item: astrix_rpc_core::RpcError, protowire::RpcError, { Self { message: item.to_string() } });
from!(item: &astrix_rpc_core::RpcError, protowire::RpcError, { Self { message: item.to_string() } });

// ----------------------------------------------------------------------------
// protowire to rpc_core
// ----------------------------------------------------------------------------

from!(item: &protowire::RpcError, astrix_rpc_core::RpcError, { astrix_rpc_core::RpcError::from(item.message.to_string()) });