use rust_decimal::{ dec, prelude::FromPrimitive, Decimal };

pub struct TradingMath;

pub trait Math {
    fn calculate_mean(values: &[f64]) -> Decimal {
        let sum = values
            .iter()
            .fold(dec!(0), |acc, val| { acc + Decimal::from_f64(*val).unwrap_or_default() });

        sum / Decimal::from_usize(values.len()).unwrap_or_default()
    }
    fn calculate_standard_deviation(values: &[Decimal]) -> Decimal;
    fn calculate_variance(values: &[Decimal]) -> Decimal;

    // Z-score calculation (how many std devs from mean)
    fn calculate_z_score(value: Decimal, mean: Decimal, std_dev: Decimal) -> Decimal;

    // Bollinger Bands
    fn calculate_bollinger_bands(
        prices: &[Decimal],
        period: usize,
        deviations: Decimal
    ) -> (Vec<Decimal>, Vec<Decimal>, Vec<Decimal>);

    // Moving averages
    fn calculate_simple_moving_average(values: &[Decimal], period: usize) -> Vec<Decimal>;
    fn calculate_exponential_moving_average(values: &[Decimal], period: usize) -> Vec<Decimal>;

    // RSI (Relative Strength Index)
    fn calculate_rsi(prices: &[Decimal], period: usize) -> Vec<Decimal>;

    // MACD (Moving Average Convergence Divergence)
    fn calculate_macd(
        prices: &[Decimal],
        fast_period: usize,
        slow_period: usize,
        signal_period: usize
    ) -> (Vec<Decimal>, Vec<Decimal>, Vec<Decimal>);

    // Mean Reversion specific metrics
    fn calculate_half_life(prices: &[Decimal]) -> Decimal;
    fn calculate_hurst_exponent(prices: &[Decimal]) -> Decimal;
    fn calculate_adf_statistic(prices: &[Decimal]) -> Decimal; // Augmented Dickey-Fuller test

    // Cointegration for pairs trading
    fn calculate_cointegration(series_a: &[Decimal], series_b: &[Decimal]) -> Decimal;
    fn calculate_hedge_ratio(series_a: &[Decimal], series_b: &[Decimal]) -> Decimal;
    fn calculate_spread(
        series_a: &[Decimal],
        series_b: &[Decimal],
        hedge_ratio: Decimal
    ) -> Vec<Decimal>;

    // Entry/exit signal calculations
    fn calculate_entry_signals(z_scores: &[Decimal], entry_threshold: Decimal) -> Vec<i8>;
    fn calculate_exit_signals(z_scores: &[Decimal], exit_threshold: Decimal) -> Vec<i8>;

    fn calculate_kelly_criterion(win_prob: Decimal, win_loss_ratio: Decimal) -> Decimal;
    fn calculate_optimal_position_size(
        price: Decimal,
        volatility: Decimal,
        account_size: Decimal,
        risk_factor: Decimal
    ) -> Decimal;

    // Performance metrics
    fn calculate_sharpe_ratio(returns: &[Decimal], risk_free_rate: Decimal) -> Decimal;
    fn calculate_sortino_ratio(returns: &[Decimal], risk_free_rate: Decimal) -> Decimal;
    fn calculate_max_drawdown(equity_curve: &[Decimal]) -> Decimal;

    // Stop-loss calculations
    fn calculate_atr(
        high: &[Decimal],
        low: &[Decimal],
        close: &[Decimal],
        period: usize
    ) -> Vec<Decimal>;
    fn calculate_chandelier_exit(
        high: &[Decimal],
        low: &[Decimal],
        close: &[Decimal],
        period: usize,
        multiplier: Decimal
    ) -> (Vec<Decimal>, Vec<Decimal>);

    // Additional utility functions for decimal calculations
    fn round_to_tick(value: Decimal, tick_size: Decimal) -> Decimal;
    fn truncate_to_precision(value: Decimal, precision: u32) -> Decimal;
}

impl Math for TradingMath {}

#[cfg(test)]
mod tests {
    use rust_decimal::{ dec, Decimal };

    use super::{ TradingMath, Math };

    #[test]
    fn integer_percentage_calculation() {
        let integer_vector: Vec<(f64, f64, Decimal)> = vec![
            (50.0, 100.0, dec!(50)),
            (25.0, 200.0, dec!(12.5)),
            (75.0, 300.0, dec!(25)),
            (120.0, 600.0, dec!(20)),
            (90.0, 450.0, dec!(20)),
            (64.0, 400.0, dec!(16)),
            (35.0, 70.0, dec!(50)),
            (18.0, 150.0, dec!(12)),
            (250.0, 1000.0, dec!(25)),
            (82.0, 410.0, dec!(20))
        ];

        integer_vector.into_iter().for_each(|(part, val, expected)| {
            let percentage = TradingMath::calculate_percentage(val, part);

            assert_eq!(percentage, expected);
        });
    }
}
