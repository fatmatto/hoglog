extern crate logwatcher;
use clap::{App, Arg};
use logwatcher::{LogWatcher, LogWatcherAction};
use serde::Deserialize;
use std::fs;
use std::path::Path;
mod server;
mod storage;

#[derive(Deserialize)]
struct ReceiverConfig {
    ip: String,
    port: String,
}
#[derive(Deserialize)]
struct ForwarderConfig {
    receiver_url: String,
    targets: Vec<String>,
}

fn main() {
    let matches = App::new("Hog")
        .version("1.0")
        .author("Mattia Alfieri <mattialfieri@gmail.com>")
        .about("Aggregates JSON logs")
        .subcommand(
            App::new("agent").about("Sends logs to the receiver").arg(
                Arg::new("config")
                    .about("sets the config file to use")
                    .short('c')
                    .long("config")
                    .takes_value(true),
            ),
        )
        .subcommand(
            App::new("receiver").about("receives logs").arg(
                Arg::new("config")
                    .about("sets the config file to use")
                    .short('c')
                    .long("config")
                    .takes_value(true),
            ),
        )
        .get_matches();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level app
    if let Some(ref matches) = matches.subcommand_matches("agent") {
        println!("Running in agent mode");
        if let Some(ref config_path) = matches.value_of("config") {
            // It's safe to call unwrap() because of the required options we set above
            println!("Doing work with {}", config_path);

            let contents =
                fs::read_to_string(config_path).expect("Something went wrong reading the file");

            println!("With text:\n{}", contents);
            // Ora parsiamo il contenuto
            let configuration: ForwarderConfig = toml::from_str(&contents).unwrap();

            println!("Receiver is at {}", configuration.receiver_url);

            for target in &configuration.targets {
                println!("Tailing logfile {}", target);

                if Path::new(target).exists() {
                    let mut log_watcher = LogWatcher::register(target.to_string()).unwrap();

                    let ship_log = |line: String| -> Result<(), reqwest::Error> {
                        println!("Invio il log a {}", configuration.receiver_url);
                        let client = reqwest::blocking::Client::new();
                        let _res = client
                            //.post("https://e4e34fea4865181259c3edde3aa91631.m.pipedream.net")
                            .post(&configuration.receiver_url)
                            .body(line)
                            .send()?;
                        //return Ok(&res.status().as_str().to_string())
                        return Ok(());
                    };

                    log_watcher.watch(&mut move |line: String| {
                        println!("Line {}", line);
                        let _result = ship_log(line);
                        println!("Log sent");
                        LogWatcherAction::None
                    });
                } else {
                    println!(
                        "Ignoring target {}: File not found or not readable.",
                        target
                    )
                }
            }
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("receiver") {
        println!("Running in receiver mode");
        if let Some(ref config_path) = matches.value_of("config") {
            // It's safe to call unwrap() because of the required options we set above
            println!("Doing work with {}", config_path);

            let contents =
                fs::read_to_string(config_path).expect("Something went wrong reading the file");

            println!("With text:\n{}", contents);
            // Ora parsiamo il contenuto
            let configuration: ReceiverConfig = toml::from_str(&contents).unwrap();

            server::listen(configuration.ip, configuration.port);
        }
    }
}
