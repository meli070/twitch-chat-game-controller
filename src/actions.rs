use std::{
    sync::{self, mpsc::Sender},
    time::Duration,
};

use twitch_irc::message::{PrivmsgMessage, ServerMessage};
use yaml_rust::Yaml;

/// The actual input information which is used
struct Input {}

/// Creates an [Input] as an Result of a command specified in the config file
struct Action {
    key: Input,
    time_ms: Duration,
}

/// Handles chat messages and creates an [Action] if a command is recognized
pub struct ChatHandler {}

/// Chat message
pub struct ChatMessage {}

impl ChatHandler {
    /// Create [ChatHandler] and register actions based on `config`
    pub fn create(config: &Yaml) -> Self {
        todo!()
    }

    pub fn handle(&mut self, message: &ServerMessage) {
        match message {
            ServerMessage::Privmsg(PrivmsgMessage {
                message_text,
                sender,
                ..
            }) => {
                println!("{}: {}", sender.name, message_text);
            }
            _ => (),
        }
        todo!()
    }
}

impl Drop for ChatHandler {
    fn drop(&mut self) {
        todo!()
    }
}
