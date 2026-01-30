# 🕐 tznow

> A beautiful command-line tool to display current time across multiple timezones

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

`tznow` is a simple yet powerful CLI tool built in Rust that displays the current time across multiple timezones. Whether you're coordinating with global teams or just need to know what time it is in different parts of the world, `tznow` has you covered.

## ✨ Features

- 🌍 **Multiple Timezones** - Display time for any timezone you need
- 🔄 **Watch Mode** - Auto-refresh times at custom intervals (supports sub-second precision)
- 🏷️ **Smart Aliases** - Use common abbreviations like IST, PST, EST instead of full timezone names
- 📊 **Clean Display** - Beautiful, easy-to-read formatted output
- ⚡ **Fast & Lightweight** - Written in Rust, starts instantly with minimal resource usage
- 🎯 **Always Shows Local & UTC** - Convenient reference points included automatically

## 🚀 Installation

### Homebrew (macOS)

```bash
brew tap itsmykairos/tap
brew install tznow
```

### From Source

If you have Rust installed:

```bash
cargo install --git https://github.com/itsmykairos/tznow
```

### Download Binary

Download the latest release for your platform from the [releases page](https://github.com/itsmykairos/tznow/releases).

## 📖 Usage

### Basic Usage

Display current time in specific timezones:

```bash
tznow America/New_York Asia/Tokyo Europe/London
```

Output:
```
🕐 Time Zones
────────────────────────
Local          2025-01-29 15:30:45
UTC            2025-01-29 23:30:45
America/New_York 2025-01-29 18:30:45
Asia/Tokyo     2025-01-30 08:30:45
Europe/London  2025-01-29 23:30:45
```

### Using Timezone Aliases

Save typing with common timezone abbreviations:

```bash
tznow IST PST EST
```

### Watch Mode

Auto-refresh times every second:

```bash
tznow --watch America/Los_Angeles Asia/Kolkata
```

Custom refresh interval (in seconds):

```bash
# Refresh every 5 seconds
tznow --watch 5 PST EST CET

# Sub-second refresh (every 100ms)
tznow --watch 0.1 UTC
```

Press `Ctrl+C` to exit watch mode.

## 🏷️ Supported Timezone Aliases

Common timezone abbreviations for your convenience:

| Alias | Full Timezone Name |
|-------|-------------------|
| IST | Asia/Kolkata (Indian Standard Time) |
| PST | America/Los_Angeles (Pacific Standard Time) |
| EST | America/New_York (Eastern Standard Time) |
| EAT | Africa/Addis_Ababa (East Africa Time) |
| CET | Africa/Algiers (Central European Time) |
| WAT | Africa/Bangui (West Africa Time) |
| CAT | Africa/Blantyre (Central Africa Time) |
| SAST | Africa/Johannesburg (South Africa Standard Time) |
| UTC | UTC (Coordinated Universal Time) |

You can also use any valid [IANA timezone name](https://en.wikipedia.org/wiki/List_of_tz_database_time_zones).

## 🎨 Examples

**Quick check across major business hubs:**
```bash
tznow PST EST UTC Asia/Tokyo Asia/Singapore
```

**Monitor time during a global meeting:**
```bash
tznow --watch 1 America/New_York Europe/London Asia/Kolkata
```

**Check if it's business hours in different regions:**
```bash
tznow America/Los_Angeles Europe/Paris Asia/Shanghai Australia/Sydney
```

## 🛠️ Building from Source

Requirements:
- Rust 1.70 or higher

```bash
git clone https://github.com/itsmykairos/tznow.git
cd tznow
cargo build --release
```

The binary will be available at `target/release/tznow`.

## 📦 Dependencies

- `chrono` - Date and time handling
- `chrono-tz` - Timezone database
- `clap` - Command-line argument parsing
- `crossterm` - Terminal UI for watch mode
- `ctrlc` - Graceful shutdown handling

## 🤝 Contributing

Contributions are welcome! Feel free to:

- Report bugs
- Suggest new features
- Submit pull requests
- Improve documentation

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with ❤️ using Rust
- Timezone data from the [IANA Time Zone Database](https://www.iana.org/time-zones)

---

**Made with 🦀 by [Kairos]**

*Star ⭐ this repo if you find it useful!*
