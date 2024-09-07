use exit_on_error::ExitOnError;
use log::{debug, error, info, warn, LevelFilter, ParseLevelError};
use std::{
    fs::{self, read_to_string},
    path::Path,
    str::FromStr,
    thread,
};
use tokio::runtime::{self};
use twitch_irc::{
    login::StaticLoginCredentials,
    SecureTCPTransport, TwitchIRCClient,
};
use yaml_rust::{self, Yaml};

mod actions;
mod exit_on_error;
mod keyboard;

static TEMPLATE_CONFIG: &str = include_str!("template_config.yaml");
static CONFIG_PATH: &str = "config.yaml";

fn main() {
    simple_logging::log_to_file("twitch-controller.log", LevelFilter::Info).unwrap();
    info!("Twitch game controller started.");
    let config = load_or_create_config(CONFIG_PATH);
    set_log_level(&config).unwrap_or_else(|_| warn!("Problem setting log_level, ignoring..."));
    debug!("Creating global key listener callback...");
    let callback = keyboard::create_global_listener(&config);
    debug!("Registering global key listener callback...");
    thread::spawn(move || {
        // TODO: This is bullshit: will block one CPU core! Need to find a better library than rdev!
        rdev::listen(callback).exit_on_error("Could not register global event listener callback");
    });
    info!("Registered key listener.");
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

        let mut message_handler = actions::ChatHandler::create(config);

        // consuming incoming messages
        let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                message_handler.handle(&message);
            }
        });

        // join the channel
        let channel = config["channel"]
            .as_str()
            .exit_on_error("Could not find channel to connect in config file!");
        client
            .join(channel.to_owned())
            .exit_on_error("Can not connect to Twitch channel!");
        info!("Joined the Twitch channel: {channel}");

        // keep the tokio executor alive.
        let exit_signal = keyboard::get_exit_cancellation_token();
        tokio::select! {
            _ = exit_signal.cancelled() => {
                info!("Received exit request, exiting...");
            }
            _ = join_handle => {
                warn!("Something went wrong listening to chat, exiting...");
            }
        }
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
