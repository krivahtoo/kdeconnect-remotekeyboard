use dbus::arg::{RefArg, Variant};

#[derive(Debug)]
pub enum MessageType {
    Inbox,
    Sent,
    Draft,
    Outbox,
    Failed,
    Queued,
    Unknown,
}

#[derive(Debug)]
pub struct ConversationMessage {
    pub sender: String,
    pub body: String,
    pub date: i64,
    pub msg_type: MessageType,
    pub is_read: bool,
    pub thread_id: i64,
    pub unique_id: i32,
    pub sim_id: i64,
}

impl dbus::arg::AppendAll for ConversationMessage {
    fn append(&self, _i: &mut dbus::arg::IterAppend) {
        //dbus::arg::RefArg::append(&self.sender, i);
    }
}

impl dbus::arg::ReadAll for ConversationMessage {
    fn read(i: &mut dbus::arg::Iter) -> Result<Self, dbus::arg::TypeMismatchError> {
        let mut result = ConversationMessage {
            sender: String::new(),
            body: String::new(),
            date: 0,
            msg_type: MessageType::Unknown,
            is_read: false,
            thread_id: 0,
            unique_id: 0,
            sim_id: 0,
        };

        let msg_variant: Variant<Box<dyn RefArg>> = i.read()?;

        if let Some(mut msg_structs) = msg_variant.as_iter() {
            if let Some(msg_struct) = msg_structs.next() {
                if let Some(mut struct_elements) = msg_struct.as_iter() {
                    let _event = struct_elements.next();

                    if let Some(body) = struct_elements.next() {
                        if let Some(body) = body.as_str() {
                            result.body = body.to_string();
                        }
                    }

                    struct_elements.next().map(|array_of_array_of_addresses| {
                        if let Some(mut array_of_addresses_iter) =
                            array_of_array_of_addresses.as_iter()
                        {
                            if let Some(first_array_of_addresses) = array_of_addresses_iter.next() {
                                if let Some(mut addresses) = first_array_of_addresses.as_iter() {
                                    if let Some(first_address) = addresses.next() {
                                        if let Some(first_address) = first_address.as_str() {
                                            result.sender = first_address.to_string();
                                        }
                                    }
                                }
                            }
                        }
                    });

                    struct_elements.next().map(|date| {
                        if let Some(date) = date.as_i64() {
                            result.date = date;
                        }
                    });

                    struct_elements.next().map(|message_type| {
                        if let Some(message_type) = message_type.as_i64() {
                            result.msg_type = match message_type {
                                1 => MessageType::Inbox,
                                2 => MessageType::Sent,
                                3 => MessageType::Draft,
                                4 => MessageType::Outbox,
                                5 => MessageType::Failed,
                                6 => MessageType::Queued,
                                _ => MessageType::Unknown,
                            };
                        }
                    });

                    struct_elements.next().map(|read| {
                        if let Some(read) = read.as_i64() {
                            result.is_read = read != 0;
                        }
                    });

                    struct_elements.next().map(|thread_id| {
                        if let Some(thread_id) = thread_id.as_i64() {
                            result.thread_id = thread_id;
                        }
                    });

                    struct_elements.next().map(|unique_id| {
                        if let Some(unique_id) = unique_id.as_i64() {
                            result.unique_id = unique_id as i32;
                        }
                    });

                    struct_elements.next().map(|sim_id| {
                        if let Some(sim_id) = sim_id.as_i64() {
                            result.sim_id = sim_id;
                        }
                    });
                }
            }
        }

        Ok(result)
    }
}

pub struct ConversationMessageCreated(pub ConversationMessage);

impl dbus::message::SignalArgs for ConversationMessageCreated {
    const NAME: &'static str = "conversationCreated";
    const INTERFACE: &'static str = "org.kde.kdeconnect.device.conversations";
}

impl dbus::arg::AppendAll for ConversationMessageCreated {
    fn append(&self, i: &mut dbus::arg::IterAppend) {
        self.0.append(i);
    }
}

impl dbus::arg::ReadAll for ConversationMessageCreated {
    fn read(i: &mut dbus::arg::Iter) -> Result<Self, dbus::arg::TypeMismatchError> {
        Ok(ConversationMessageCreated(ConversationMessage::read(i)?))
    }
}

pub struct ConversationMessageUpdated(pub ConversationMessage);

impl dbus::message::SignalArgs for ConversationMessageUpdated {
    const NAME: &'static str = "conversationUpdated";
    const INTERFACE: &'static str = "org.kde.kdeconnect.device.conversations";
}

impl dbus::arg::AppendAll for ConversationMessageUpdated {
    fn append(&self, i: &mut dbus::arg::IterAppend) {
        self.0.append(i);
    }
}

impl dbus::arg::ReadAll for ConversationMessageUpdated {
    fn read(i: &mut dbus::arg::Iter) -> Result<Self, dbus::arg::TypeMismatchError> {
        Ok(ConversationMessageUpdated(ConversationMessage::read(i)?))
    }
}
