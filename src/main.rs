use chrono::{DateTime, Local, Utc};
use chrono_tz::Tz;
use clap::Parser;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::borrow::Cow;
use std::{
    io::{Write, stdout},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

// use time_tz::timezones::db::america::New_York;

#[derive(Parser)]
#[command(name = "tznow")]
struct Args {
    /// Watch interval in seconds (supports sub-second, e.g. 0.1)
    #[arg(long,
        num_args = 0..=1,
        default_missing_value = "1"
        )]
    watch: Option<f64>,

    /// Timezones (aliases allowed)
    zones: Vec<String>,
}

fn resolve_alias(zone: &str) -> Cow<'_, str> {
    match zone.to_uppercase().as_str() {
        "IST" => Cow::Borrowed("Asia/Kolkata"),
        "PST" => Cow::Borrowed("America/Los_Angeles"),
        "EST" => Cow::Borrowed("America/New_York"),
        "EAT" => Cow::Borrowed("Africa/Addis_Ababa"),
        "CET" => Cow::Borrowed("Africa/Algiers"),
        "WAT" => Cow::Borrowed("Africa/Bangui"),
        "CAT" => Cow::Borrowed("Africa/Blantyre"),
        "SAST" => Cow::Borrowed("Africa/Johannesburg"),
        "" => Cow::Borrowed(""),
        "UTC" => Cow::Borrowed("UTC"),
        _ => Cow::Borrowed(zone),
    }
}

fn print_times(zones: &[String]) {
    // let mut out = stdout();
    // execute!(out, MoveTo(0, 0), Clear(ClearType::All)).unwrap();

    println!("🕒 Time Zones");
    println!("────────────────────────");

    // Local time
    let local: DateTime<Local> = Local::now();
    println!("Local          {}", local.format("%Y-%m-%d %H:%M:%S"));

    // UTC
    let utc: DateTime<Utc> = Utc::now();
    println!("UTC            {}", utc.format("%Y-%m-%d %H:%M:%S"));

    for zone in zones {
        let resolved = resolve_alias(zone);
        match resolved.parse::<Tz>() {
            Ok(tz) => {
                let now = Utc::now().with_timezone(&tz);
                println!("{:<14} {}", zone, now.format("%Y-%m-%d %H:%M:%S"));
            }
            Err(_) => {
                println!("{:<14} ❌ Invalid timezone", zone);
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let mut stdout = stdout();

    let interval = args.watch.unwrap_or(0.0);

    if interval <= 0.0 {
        print_times(&args.zones);
    } else {
        // ✅ Take control of terminal
        execute!(stdout, EnterAlternateScreen, Hide).unwrap();
        let sleep = Duration::from_secs_f64(interval);

        while running.load(Ordering::SeqCst) {
            // move the position on top
            execute!(stdout, MoveTo(0, 0), Clear(ClearType::All)).unwrap();
            print_times(&args.zones);
            thread::sleep(sleep);
            stdout.flush().unwrap();
        }
        // ✅ Restore terminal cleanly
        execute!(stdout, Show, LeaveAlternateScreen).unwrap();
    }
}
