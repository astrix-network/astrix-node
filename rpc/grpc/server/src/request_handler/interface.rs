use super::method::{DropFn, Method, MethodTrait, RoutingPolicy};
use crate::{
    connection::Connection,
    connection_handler::ServerContext,
    error::{GrpcServerError, GrpcServerResult},
};
use astrix_grpc_core::{
    ops::AstrixdPayloadOps,
    protowire::{AstrixdRequest, AstrixdResponse},
};
use std::fmt::Debug;
use std::{collections::HashMap, sync::Arc};

pub type AstrixdMethod = Method<ServerContext, Connection, AstrixdRequest, AstrixdResponse>;
pub type DynAstrixdMethod = Arc<dyn MethodTrait<ServerContext, Connection, AstrixdRequest, AstrixdResponse>>;
pub type AstrixdDropFn = DropFn<AstrixdRequest, AstrixdResponse>;
pub type AstrixdRoutingPolicy = RoutingPolicy<AstrixdRequest, AstrixdResponse>;

/// An interface providing methods implementations and a fallback "not implemented" method
/// actually returning a message with a "not implemented" error.
///
/// The interface can provide a method clone for every [`AstrixdPayloadOps`] variant for later
/// processing of related requests.
///
/// It is also possible to directly let the interface itself process a request by invoking
/// the `call()` method.
pub struct Interface {
    server_ctx: ServerContext,
    methods: HashMap<AstrixdPayloadOps, DynAstrixdMethod>,
    method_not_implemented: DynAstrixdMethod,
}

impl Interface {
    pub fn new(server_ctx: ServerContext) -> Self {
        let method_not_implemented = Arc::new(Method::new(|_, _, astrixd_request: AstrixdRequest| {
            Box::pin(async move {
                match astrixd_request.payload {
                    Some(ref request) => Ok(AstrixdResponse {
                        id: astrixd_request.id,
                        payload: Some(AstrixdPayloadOps::from(request).to_error_response(GrpcServerError::MethodNotImplemented.into())),
                    }),
                    None => Err(GrpcServerError::InvalidRequestPayload),
                }
            })
        }));
        Self { server_ctx, methods: Default::default(), method_not_implemented }
    }

    pub fn method(&mut self, op: AstrixdPayloadOps, method: AstrixdMethod) {
        let method: DynAstrixdMethod = Arc::new(method);
        if self.methods.insert(op, method).is_some() {
            panic!("RPC method {op:?} is declared multiple times")
        }
    }

    pub fn replace_method(&mut self, op: AstrixdPayloadOps, method: AstrixdMethod) {
        let method: DynAstrixdMethod = Arc::new(method);
        let _ = self.methods.insert(op, method);
    }

    pub fn set_method_properties(
        &mut self,
        op: AstrixdPayloadOps,
        tasks: usize,
        queue_size: usize,
        routing_policy: AstrixdRoutingPolicy,
    ) {
        self.methods.entry(op).and_modify(|x| {
            let method: Method<ServerContext, Connection, AstrixdRequest, AstrixdResponse> =
                Method::with_properties(x.method_fn(), tasks, queue_size, routing_policy);
            let method: Arc<dyn MethodTrait<ServerContext, Connection, AstrixdRequest, AstrixdResponse>> = Arc::new(method);
            *x = method;
        });
    }

    pub async fn call(
        &self,
        op: &AstrixdPayloadOps,
        connection: Connection,
        request: AstrixdRequest,
    ) -> GrpcServerResult<AstrixdResponse> {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).call(self.server_ctx.clone(), connection, request).await
    }

    pub fn get_method(&self, op: &AstrixdPayloadOps) -> DynAstrixdMethod {
        self.methods.get(op).unwrap_or(&self.method_not_implemented).clone()
    }
}

impl Debug for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interface").finish()
    }
}
