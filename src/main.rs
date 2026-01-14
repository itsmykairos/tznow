use chrono::prelude::*;
use chrono_tz::Tz;
use clap::Parser;
use dirs::config_dir;
use serde::Deserialize;
use std::{fs, thread, time::Duration};

#[derive(Parser)]
struct Cli {
    zones: Vec<String>,
    #[arg(long)]
    format24: bool,
    #[arg(long)]
    watch: Option<f64>, // fractional seconds
}

#[derive(Deserialize)]
struct Config {
    zones: Option<Vec<String>>,
    format24: Option<bool>,
}

fn resolve_alias(zone: &str) -> String {
    match zone.to_uppercase().as_str() {
        "IST" => "Asia/Kolkata".to_string(),
        "PST" => "America/Los_Angeles".to_string(),
        "EST" => "America/New_York".to_string(),
        "UTC" => "UTC".to_string(),
        other => other.to_string(),
    }
}


fn load_config() -> Option<Config> {
    let mut path = config_dir()?;
    path.push("tznow/config.toml");
    let content = fs::read_to_string(path).ok()?;
    toml::from_str(&content).ok()
}

fn print_times(zones: &[String], format24: bool) {
    print!("\x1B[2J\x1B[H"); // clear terminal
    let now_utc = Utc::now();

    for z in zones {
        let zone_name = resolve_alias(z);
        if zone_name == "LOCAL" {
            // local system time
            let local = Local::now();
            let fmt = if format24 { "%H:%M:%S" } else { "%I:%M:%S %p" };
            println!("{:<20} {}", "Local", local.format(fmt));
        } else if let Ok(tz) = zone_name.parse::<Tz>() {
            let time = now_utc.with_timezone(&tz);
            let fmt = if format24 { "%H:%M:%S" } else { "%I:%M:%S %p" };
            println!("{:<20} {}", zone_name, time.format(fmt));
        } else {
            println!("❌ Invalid timezone: {}", zone_name);
        }
    }
}


fn main() {
    let cli = Cli::parse();
    let config = load_config();

    let zones = if !cli.zones.is_empty() {
        cli.zones.clone()
    } else if let Some(cfg) = &config {
        cfg.zones.clone().unwrap_or_else(|| vec!["Local".to_string(), "UTC".to_string()])
    } else {
        vec!["Local".to_string(), "UTC".to_string()]
    };

    let format24 = cli.format24 || config.as_ref().and_then(|c| c.format24).unwrap_or(false);

    if let Some(interval) = cli.watch {
        let sleep_duration = Duration::from_secs_f64(interval);
        loop {
            print_times(&zones, format24);
            thread::sleep(sleep_duration);
        }
    } else {
        print_times(&zones, format24);
    }
}
