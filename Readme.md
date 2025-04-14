# Mean Reversion Crypto Trading Bot

A fully customizable CLI bot written in Rust that trades cryptocurrency using a [**mean reversion**][1] strategy. Currently supports [**Binance**][2], with plans to expand to other exchanges in the future.

[**Disclaimer**](#disclaimer)

## Table of Contents

- [Features](#features)
- [How It Works](#how-it-works)
- [Setup](#setup)
- [Running the Bot](#running-the-bot)
- [Logs](#logs)
- [Status](#status)
- [Roadmap](#roadmap)
- [Disclaimer](#disclaimer)

---

## Features

- [Mean reversion-based][1] strategy
- Fully configurable via [`strategy.toml`][5]
- Simulated and real trading modes
- Logging to stdout and files (`logs/info.log`, `logs/error.log`)

---

## How It Works

This bot monitors market prices and attempts to profit from price movements that deviate from a [calculated mean][1]. For more on the strategy and how to customize it, see the [strategy documentation][3].

---

## Setup

### 1. Install Rust

If you haven’t already, [install Rust][4]

### 2. Clone the repo

```bash
git clone https://github.com/yurii2007/mean-reversion-trading-bot
cd mean-reversion-trading-bot
```

### 3. Configure Environment Variables

Copy the [template][5] and add your [Binance API keys][7]

### 4. Customize strategy

The trading strategy is configured via the [strategy.toml][6] file at the root of the project.

See the [strategy configuration guide][3] for all available options.

## Running the Bot

### Run directly:

```bash
cargo run --release
```

### Or build and run

```bash
cargo build --release
./target/release/mean-reversion-trading-bot
```

### Logs

Logs are written both to the console (stdout) and to files in the logs/ folder:

- info.log — `INFO` and above

- error.log — `WARN` and above

- stdout - All logs level

You can customize stdout log level via the `RUST_LOG` environment variable

## Status

Currently, the bot still under development and missing some features.

## Roadmap

Here are the planned milestones and improvements:

- [ ]  Improve order precision handling to prevent quantity errors

- [ ] Add test coverage across all modules (currently only strategy parsing is tested)

- [ ] Better strategy parameters validation

- [ ] Add validation before placing orders

- [ ] Persist market/trade data across sessions

- [ ] Make logs more structured and human-readable

- [ ] Enhance error recovery mechanisms

- [ ] Refactor for better module structure and separation of concerns

- [ ] Make error messages more user-friendly

- [ ] Telegram integration for trade alerts and basic commands (pause, stop, close positions)

## Disclaimer

This bot is intended for educational and research purposes only. Trading cryptocurrencies involves significant risk. Use at your own risk. Always test in simulation mode before trading real funds. The author(s) are not responsible for any financial losses incurred while using this software.




[1]: https://en.wikipedia.org/wiki/Mean_reversion_(finance)
[2]: https://www.binance.com/en/binance-api
[3]: ./strategy.md
[4]: https://www.rust-lang.org/tools/install
[5]: ./.env.template
[6]: ./strategy.toml
[7]: https://www.binance.com/en/support/faq/detail/360002502072
