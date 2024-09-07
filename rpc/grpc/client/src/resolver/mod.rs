use super::error::Result;
use core::fmt::Debug;
use astrix_grpc_core::{
    ops::AstrixdPayloadOps,
    protowire::{AstrixdRequest, AstrixdResponse},
};
use std::{sync::Arc, time::Duration};
use tokio::sync::oneshot;

pub(crate) mod id;
pub(crate) mod matcher;
pub(crate) mod queue;

pub(crate) trait Resolver: Send + Sync + Debug {
    fn register_request(&self, op: AstrixdPayloadOps, request: &AstrixdRequest) -> AstrixdResponseReceiver;
    fn handle_response(&self, response: AstrixdResponse);
    fn remove_expired_requests(&self, timeout: Duration);
}

pub(crate) type DynResolver = Arc<dyn Resolver>;

pub(crate) type AstrixdResponseSender = oneshot::Sender<Result<AstrixdResponse>>;
pub(crate) type AstrixdResponseReceiver = oneshot::Receiver<Result<AstrixdResponse>>;
