use crate::client::{
    Connection, ConversationMessage, ConversationMessageCreated, ConversationMessageUpdated,
};
use std::time::Duration;

pub struct SMS<'a> {
    conversation_proxy: dbus::blocking::Proxy<'a, &'a dbus::blocking::Connection>,
    _sms_proxy: dbus::blocking::Proxy<'a, &'a dbus::blocking::Connection>,
}

impl<'a> SMS<'a> {
    pub fn new(connection: &'a Connection, device_id: &str) -> Self {
        Self {
            conversation_proxy: connection.0.with_proxy(
                "org.kde.kdeconnect",
                format!("/modules/kdeconnect/devices/{}", device_id),
                Duration::from_millis(5000),
            ),
            _sms_proxy: connection.0.with_proxy(
                "org.kde.kdeconnect",
                format!("/modules/kdeconnect/devices/{}/sms", device_id),
                Duration::from_millis(5000),
            ),
        }
    }

    pub fn on_conversation_created<F>(&self, mut f: F) -> Result<dbus::channel::Token, dbus::Error>
    where
        F: FnMut(ConversationMessage) -> () + Send + 'static,
    {
        self.conversation_proxy.match_signal(
            move |data: ConversationMessageCreated,
                  _: &dbus::blocking::Connection,
                  _: &dbus::Message| {
                f(data.0);
                true
            },
        )
    }

    pub fn on_conversation_updated<F>(&self, mut f: F) -> Result<dbus::channel::Token, dbus::Error>
    where
        F: FnMut(ConversationMessage) -> () + Send + 'static,
    {
        self.conversation_proxy.match_signal(
            move |data: ConversationMessageUpdated,
                  _: &dbus::blocking::Connection,
                  _: &dbus::Message| {
                f(data.0);
                true
            },
        )
    }
}
