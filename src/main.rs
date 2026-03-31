mod cli;
mod discovery;

// Logging
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::time::Duration;

// Required deps
use cli::get_app_cli;
use discovery::discover_lights;
use reqwest::Client;
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    let matches = get_app_cli(&version).get_matches();

    let verbose = match matches.occurrences_of("verbose") {
        0 => LevelFilter::Off,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter_level(verbose)
        .parse_env("LOG_LEVEL")
        .init();

    // Determine target lights
    let discover = matches.is_present("discover");
    let elgato_ip = matches.value_of("elgato_ip");

    let targets: Vec<(String, u16)> = if discover {
        println!("Discovering Elgato Keylights on the network...");
        let lights = discover_lights(Duration::from_secs(5))?;
        if lights.is_empty() {
            eprintln!("Error: No Elgato Keylights found on the network.");
            std::process::exit(1);
        }
        for light in &lights {
            println!("  Found: {} ({}:{})", light.name, light.ip, light.port);
        }
        lights.into_iter().map(|l| (l.ip, l.port)).collect()
    } else if let Some(ip) = elgato_ip {
        vec![(ip.to_string(), 9123)]
    } else {
        eprintln!("Error: Either --discover or --elgato-ip must be specified.");
        std::process::exit(1);
    };

    let numberoflights = matches.value_of("number_of_lights").unwrap();

    let switch = match matches.value_of("switch").unwrap() {
        "off" => 0,
        "on" => 1,
        "status" => 2,
        _ => 0,
    };

    if switch == 0 {
        println!("Elgato Keylight is: off");
    } else if switch == 1 {
        println!("Elgato Keylight is: on");
    }

    let brightness = matches
        .value_of("brightness")
        .and_then(|s| s.parse::<u8>().ok())
        .unwrap();

    let temperature = matches
        .value_of("temperature")
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap();

    let body = json!({
        "numberOfLights":numberoflights,
        "lights":[
            {
                "on":switch,
                "brightness":brightness,
                "temperature":temperature
            }
        ]
    });

    let client = Client::new();

    for (ip, port) in &targets {
        let url = format!("http://{}:{}/elgato/lights", ip, port);
        log::info!("Sending request to: {}", url);

        if switch == 2 {
            // GET to read current settings without modifying them
            let response = client.get(&url).send().await?;
            log::info!("Response status: {}", response.status());

            let response_body = response.text().await?;
            log::info!("Response text: {}", response_body);

            let v: Value = serde_json::from_str(&response_body)?;
            let light = &v["lights"][0];

            let power = if light["on"] == 1 { "on" } else { "off" };
            let brightness = &light["brightness"];
            let temperature = &light["temperature"];

            println!("Elgato light at {}:", ip);
            println!("  Power:       {}", power);
            println!("  Brightness:  {}", brightness);
            println!("  Temperature: {}", temperature);
        } else {
            // PUT to change settings
            let response = client.put(&url).json(&body).send().await?;

            let response_success = response.status();
            log::info!("Response status: {}", response_success);

            let response_body = response.text().await?;
            log::info!("Response text: {}", response_body);

            let response_json: serde_json::Value = serde_json::from_str(&response_body)?;
            log::info!("Response json: {:?}", response_json);
        }
    }

    Ok(())
}
