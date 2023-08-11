use std::time::Duration;
use dbus::blocking::stdintf::org_freedesktop_dbus::Properties;

use crate::client::Connection;

#[derive(Debug, Clone)]
pub enum SpecialKey {
    Backspace = 1,
    Tab = 2,
    Left = 4,
    Up = 5,
    Right = 6,
    Down = 7,
    // PageUp = 8,
    // PageDown = 9,
    // Home = 10,
    // End = 11,
    Enter = 12,
    Delete = 13,
    Escape = 14,
    // SysReq = 15,
    // ScrollLock = 16,
}

#[derive(Debug, Clone)]
pub enum Keys {
    Char(char),
    Special(SpecialKey),
}

pub struct RemoteKeyboard<'a> {
    pub proxy: dbus::blocking::Proxy<'a, &'a dbus::blocking::Connection>,
    pub id: String,
}

impl<'a> RemoteKeyboard<'a> {
    pub fn new(conn: &'a Connection, id: &str) -> Self {
        Self {
            proxy: conn.0.with_proxy(
                "org.kde.kdeconnect",
                format!("/modules/kdeconnect/devices/{}/remotekeyboard", id),
                Duration::from_millis(5000),
            ),
            id: id.into(),
        }
    }

    pub fn get_state(&self) -> Result<bool, dbus::Error> {
        self.proxy.get("org.kde.kdeconnect.device.remotekeyboard", "remoteState")
    }

    pub fn send_key(&self, key: Keys) -> Result<(), dbus::Error> {
        let (text, special) = match key {
            Keys::Char(ch) => (format!("{}", ch), -1),
            Keys::Special(k) => ("".into(), k as i32),
        };
        self.proxy.method_call(
            "org.kde.kdeconnect.device.remotekeyboard",
            "sendKeyPress",
            (text, special, false, false, false, true),
        )?;
        Ok(())
    }
}

