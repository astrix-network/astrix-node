use crate::Notification;

pub type ChannelConnection = astrix_notify::connection::ChannelConnection<Notification>;
pub use astrix_notify::connection::ChannelType;
