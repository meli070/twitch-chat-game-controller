use std::{
    collections::BTreeMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use log::{debug, warn};
use rdev::{EventType, Key};
use twitch_irc::message::{PrivmsgMessage, ServerMessage};
use yaml_rust::Yaml;

use crate::{
    exit_on_error::ExitOnError,
    keyboard::{self, ParseKey},
};

/// The actual input information which is used
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Input {
    Keyboard(Key),
}

/// Creates an [Input] as an Result of a command specified in the config file
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Action {
    input: Vec<Input>,
    time_ms: Duration,
}

/// Handles chat messages and creates an [Action] if a command is recognized
pub struct ChatHandler {
    command_to_action: BTreeMap<String, Action>,
    execute_action: Arc<AtomicBool>,
}

impl ChatHandler {
    /// Create [ChatHandler] and register actions based on `config`
    pub fn create(config: &Yaml) -> Self {
        let actions = config["actions"]
            .as_hash()
            .exit_on_error("Problem reading actions from config!");
        let mut command_to_action = BTreeMap::new();
        for (name, info) in actions.iter() {
            let name = name
                .as_str()
                .exit_on_error(format!("Error parsing action name: {name:?}").as_str());
            let time = if let Some(time) = info["time"].as_i64() {
                if time > 0 {
                    time as u64
                } else {
                    warn!("Time for {name} is negative! Using default 100ms.");
                    100
                }
            } else {
                warn!("Could not read time for action {name} using default 100ms!");
                100
            };
            let time = Duration::from_millis(time);
            let keys = info["key"]
                .as_str()
                .exit_on_error(format!("Key in action {name} not found in config!").as_str())
                .split("+");
            let keys: Vec<Input> = keys
                .map(|key| {
                    Key::parse(key)
                        .exit_on_error(format!("Could not parse key for action {name}!").as_str())
                })
                .map(Input::Keyboard)
                .collect();
            command_to_action.insert(
                name.to_string(),
                Action {
                    input: keys,
                    time_ms: time,
                },
            );
        }
        Self {
            command_to_action,
            execute_action: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn handle(&mut self, message: &ServerMessage) {
        if keyboard::PAUSE.load(Ordering::Relaxed) {
            return;
        }
        if let ServerMessage::Privmsg(PrivmsgMessage {
            message_text,
            sender,
            ..
        }) = message
        {
            println!("Twitch chat {}: {}", sender.name, message_text);
            let text = message_text.trim();
            if let Some(action) = self.command_to_action.get(text) {
                if self.execute_action.swap(true, Ordering::SeqCst) {
                    debug!("Already executing action ignore {action:?}");
                    return;
                }
                debug!("Executing {action:?}...");
                for input in &action.input {
                    ChatHandler::handle_input(input, action.time_ms);
                }
                let time = action.time_ms;
                let execute_action = self.execute_action.clone();
                tokio::spawn(async move {
                    tokio::time::sleep(time).await;
                    execute_action.store(false, Ordering::Release);
                });
            }
        }
    }

    fn handle_input(input: &Input, time: Duration) {
        match input {
            Input::Keyboard(key) => {
                rdev::simulate(&EventType::KeyPress(*key))
                    .exit_on_error("Problem simulating key press!");
                let key = *key;
                tokio::spawn(async move {
                    tokio::time::sleep(time).await;
                    rdev::simulate(&EventType::KeyRelease(key))
                        .exit_on_error("Problem simulating key release!");
                });
            }
        }
    }
}
