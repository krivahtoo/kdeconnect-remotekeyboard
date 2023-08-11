use crate::client::Connection;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;
use std::time::Duration;

pub mod remotekeyboard;

pub struct Device<'a> {
    connection: &'a Connection,
    proxy: dbus::blocking::Proxy<'a, &'a dbus::blocking::Connection>,
    id: String,
}

impl<'a> Device<'a> {
    pub fn new(connection: &'a Connection, id: String) -> Self {
        Self {
            connection,
            proxy: connection.0.with_proxy(
                "org.kde.kdeconnect",
                format!("/modules/kdeconnect/devices/{}", id),
                Duration::from_millis(5000),
            ),
            id,
        }
    }

    #[allow(dead_code)]
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_name(&self) -> Result<String, dbus::Error> {
        self.proxy.get("org.kde.kdeconnect.device", "name")
    }

    #[allow(dead_code)]
    pub fn has_plugin(&self, name: &str) -> Result<bool, dbus::Error> {
        let (has_plugin,): (bool,) =
            self.proxy
                .method_call("org.kde.kdeconnect.device", "hasPlugin", (name,))?;
        Ok(has_plugin)
    }

    pub fn get_keyboard(&self) -> remotekeyboard::RemoteKeyboard {
        remotekeyboard::RemoteKeyboard::new(self.connection, &self.id)
    }
}
