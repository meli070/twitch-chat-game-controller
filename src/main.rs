use std::{
    fs::{self, read_to_string},
    path::Path,
    str::FromStr,
};

use exit_on_error::ExitOnError;
use log::{error, info, warn, LevelFilter, ParseLevelError};
use tokio::runtime::{self};
use twitch_irc::{
    login::StaticLoginCredentials,
    message::{PrivmsgMessage, ServerMessage},
    SecureTCPTransport, TwitchIRCClient,
};
use yaml_rust::{self, Yaml};


mod exit_on_error;
mod keyboard;

static TEMPLATE_CONFIG: &str = include_str!("template_config.yaml");
static CONFIG_PATH: &str = "config.yaml";

fn main() {
    simple_logging::log_to_file("twitch-controller.log", LevelFilter::Info).unwrap();
    info!("Twitch game controller started.");
    let config = load_or_create_config(CONFIG_PATH);
    set_log_level(&config).unwrap_or_else(|_| warn!("Problem setting log_level, ignoring..."));
    rdev::listen(keyboard::global_listener)
        .exit_on_error("Could not register global event listener callback");
    connect_and_poll_twitch(&config);
    info!("Program exit.");
}

fn connect_and_poll_twitch(config: &Yaml) {
    let rt = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .exit_on_error("Could not create tokio runtime!");
    rt.block_on(async {
        // default configuration is to join chat as anonymous.
        let client_config = twitch_irc::ClientConfig::default();
        let (mut incoming_messages, client) =
            TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(client_config);

        // first thing you should do: start consuming incoming messages,
        // otherwise they will back up.
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
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
            }
        });

        // join a channel
        // This function only returns an error if the passed channel login name is malformed,
        // so in this simple case where the channel name is hardcoded we can ignore the potential
        // error with `unwrap`.
        client
            .join(
                config["channel"]
                    .as_str()
                    .exit_on_error("Could not find channel to connect in config file!")
                    .to_owned(),
            )
            .exit_on_error("Can not connect to Twitch channel!");

        // keep the tokio executor alive.
        // If you return instead of waiting the background task will exit.
        join_handle.await.unwrap();
    });
}

/// Load or create config yaml file
fn load_or_create_config<P: AsRef<Path>>(path: P) -> Yaml {
    let Ok(config) = read_to_string(&path) else {
        error!("Config file does not exist creating template config...");
        fs::write(path, TEMPLATE_CONFIG).exit_on_error("Could not write template config!");
        std::process::exit(1);
    };
    yaml_rust::YamlLoader::load_from_str(&config).exit_on_error("Loading config YAML failed!")[0]
        .clone()
}

fn set_log_level(config: &Yaml) -> Result<(), ParseLevelError> {
    log::set_max_level(LevelFilter::from_str(
        config["log_level"].as_str().unwrap_or_else(|| {
            warn!("log_level not found using 'info'");
            "info"
        }),
    )?);
    Ok(())
}
