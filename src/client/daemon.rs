use crate::client::device::Device;
use crate::client::Connection;
use std::time::Duration;

pub struct Daemon<'a> {
    connection: &'a Connection,
    proxy: dbus::blocking::Proxy<'a, &'a dbus::blocking::Connection>,
}

impl<'a> Daemon<'a> {
    pub fn new(connection: &'a Connection) -> Self {
        Self {
            connection,
            proxy: connection.0.with_proxy(
                "org.kde.kdeconnect",
                "/modules/kdeconnect",
                Duration::from_millis(5000),
            ),
        }
    }

    pub fn get_devices(
        &self,
        only_reachable: bool,
        only_paired: bool,
    ) -> Result<Vec<Device>, dbus::Error> {
        let (ids,): (Vec<String>,) = self.proxy.method_call(
            "org.kde.kdeconnect.daemon",
            "devices",
            (only_reachable, only_paired),
        )?;

        Ok(ids
            .into_iter()
            .map(|id| Device::new(self.connection, id))
            .collect())
    }
}
