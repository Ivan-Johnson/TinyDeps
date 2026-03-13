pub mod mock_notification;
mod notification_trait;

#[cfg(feature = "notification_notify_send")]
pub mod notify_send;

pub use mock_notification::MockNotification;
pub use notification_trait::Notification;
#[cfg(feature = "notification_notify_send")]
pub use notify_send::NotifySend;
