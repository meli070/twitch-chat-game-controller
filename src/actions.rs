use std::{
    collections::{BTreeMap, HashSet},
    sync::{atomic::Ordering, Arc, Mutex},
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
    input: Input,
    time_ms: Duration,
}

/// Handles chat messages and creates an [Action] if a command is recognized
pub struct ChatHandler {
    command_to_action: BTreeMap<String, Action>,
    executing_inputs: Arc<Mutex<HashSet<Input>>>,
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
            let key = Key::parse(
                info["key"]
                    .as_str()
                    .exit_on_error(format!("Key in action {name} not found in config!").as_str()),
            )
            .exit_on_error(format!("Could not parse key for action {name}!").as_str());
            command_to_action.insert(
                name.to_string(),
                Action {
                    input: Input::Keyboard(key),
                    time_ms: time,
                },
            );
        }
        Self {
            command_to_action,
            executing_inputs: Arc::default(),
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
            }) = message {
            println!("Twitch chat {}: {}", sender.name, message_text);
            let text = message_text.trim();
            if let Some(action) = self.command_to_action.get(text) {
                let mut executing_inputs = self
                    .executing_inputs
                    .lock()
                    .exit_on_error("Locking mutex failed!");
                if executing_inputs.contains(&action.input) {
                    debug!("Already executing {action:?}");
                    return;
                }
                debug!("Executing {action:?}...");
                executing_inputs.insert(action.input.clone());
                match action.input {
                    Input::Keyboard(key) => {
                        rdev::simulate(&EventType::KeyPress(key)).exit_on_error("Problem simulating key press!");;
                        let time = action.time_ms;
                        let executing_inputs = self.executing_inputs.clone();
                        tokio::spawn(async move {
                            tokio::time::sleep(time).await;
                            rdev::simulate(&EventType::KeyRelease(key))
                                .exit_on_error("Problem simulating key release!");
                            tokio::time::sleep(Duration::from_millis(20)).await;
                            executing_inputs
                                .lock()
                                .exit_on_error(
                                    "Could not lock mutex to remove executing input!",
                                )
                                .remove(&Input::Keyboard(key));
                        });
                    }
                }
            }
        }
    }
}
