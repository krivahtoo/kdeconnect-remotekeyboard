use std::time::Duration;

pub struct Connection(pub dbus::blocking::Connection);

impl Connection {
    pub fn new() -> Result<Self, dbus::Error> {
        Ok(Self(dbus::blocking::Connection::new_session()?))
    }

    #[allow(dead_code)]
    pub fn process(&self, timeout: Duration) -> Result<(), dbus::Error> {
        self.0.process(timeout)?;
        Ok(())
    }
}
