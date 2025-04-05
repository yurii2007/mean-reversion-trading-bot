#  Mean Reversion Strategy Configuration

This file documents the customizable parameters available in the `strategy.toml` configuration used by the [Mean Reversion Crypto Trading Bot][1].

---

## Overview

The strategy is based on the [**mean reversion**][2] concept:

> _Prices tend to revert back to a historical average over time._

This configuration defines:

- The trading pair
- Timeframes for analysis
- Exchange & API options
- Risk management rules
- How the mean and deviation are calculated
- Conditions for entering/exiting positions

---

##  Core Settings

### `symbol`

**Type:** `string`  
**Example:** `"BTCUSDT"`

> Symbol to be used on the exchange (Binance format).

---

### `pair`

**Type:** `string`  
**Example:** `"BTC/USDT"`

> Exchnage api friendly format of the trading pair.

---

### `trading_symbol`

**Type:** `string`  
**Example:** `"USDT"`

> The quote currency. Determines the capital you’re trading **with** (used in capital calculation).

---

## Timeframes

```toml
[timeframe]
interval = "2h"
tick = "30m"
```

### `interval`

**Type:** `enum`<br />
[**Possible values:**][3]: `1m`, `3m`, `5m`, `15m`, `30m`, `1h`, `2h`, `4h`, `8h`, `12h`, `1d`, `3d`, `1w`, `1M`<br />
**Example:** `2h`

> Total lookback window used to calculate the average price.

### `tick`

**Type:** `enum`<br />
[**Possible values:**][3] `1m`, `3m`, `5m`, `15m`, `30m`, `1h`, `2h`, `4h`, `8h`, `12h`, `1d`, `3d`, `1w`, `1M`<br />
**Example:** `15m`

> Timeframe for a single candle/bar (used for fetching historical data and live price).

## Exchange

```toml
[exchange]
api = "binance"
```

### `api`

**Type:** `enum`
**Possible values:** `binance` 
**Example:** `binance`

> Exchange api bot will use, currently, only binance is supported.

## Risk Management

```toml
[risk_management]
capital_per_trade = 0.1
max_positions = 5
max_drawdown = 3.5
stop_loss = 0.5
profit_level = 0.2
```

| Parameter           | Type                | Description                                       | Example                                |
|---------------------|---------------------|---------------------------------------------------|----------------------------------------|
| `capital_per_trade` | `float` (0.0 - 1.0) | Fraction of your total capital used per trade.    | `0.1` = 10% of available capital       |
| `max_positions`     | `integer`           | Maximum number of concurrent open positions       | `5` = Never hold more than 5 positions |
| `max_drawdown`      | `float`             | Percentage drawdown to trigger trading suspension | `3.5` = Stop new trades if down 3.5%   |
| `stop_loss`         | `float`             | Percentage loss at which to exit positions        | `0.05` = Exit if position loses 5%    |
| `profit_level`      | `float`             | Percentage gain at which to take profit           | `0.2` = Exit when position gains 20%  |


## Mean Calculation

```toml
[timeframe.period_measurement]
measure_bars = 20
mean_calculation_method = "SimpleMA"
```

### `measure_bars`

**Type:** `usize`
**Example:** 20

> Number of bars to calculate the mean from.

### `mean_calculation_method`

**Type:** `enum`<br />
**Possible values:** `SimpleMA`, `EMA`, `VWAP`<br /> 
**Example:** `SimpleMA`

> Currently only SimpleMA method is supported.

## Deviation Measurement

```toml
[measurement_deviation]
enter_deviation = 0.15
```

### `enter_deviation`

**Type:** `f64`<br />
**Example:** 0.15

> Minimum % deviation from the average price to trigger a buy.
> This defines the bot's "edge" — how far price must diverge from the mean before entering a trade.

## Notes

- All percentage values are in decimal form. For example, 0.5 = 50%
- Do not specify any negative values in config, they would be automatically converted during trading cycles.
- Config is live-loaded on startup; changes require a restart of the bot.

[1]: ./Readme.md
[2]: https://en.wikipedia.org/wiki/Mean_reversion_(finance)
[3]: https://docs.rs/binance_spot_connector_rust/1.3.0/binance_spot_connector_rust/market/klines/enum.KlineInterval.html#variants
