//! Technical Analysis Dashboard
//!
//! This example demonstrates comprehensive technical analysis using various indicators
//! including moving averages, RSI, MACD, Bollinger Bands, and more.

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;
    let symbol = "AAPL"; // Apple Inc. as example

    println!("═══════════════════════════════════════════════════════════");
    println!("       📊 TECHNICAL ANALYSIS DASHBOARD - {} 📊", symbol);
    println!("═══════════════════════════════════════════════════════════\n");

    // 1. Moving Averages Analysis
    println!("📈 MOVING AVERAGES ANALYSIS");
    println!("─────────────────────────────────────────────────────────");

    // Get different moving averages
    let sma_20 = client
        .technical_indicators()
        .get_sma(symbol, 20, None, None)
        .await?;
    let sma_50 = client
        .technical_indicators()
        .get_sma(symbol, 50, None, None)
        .await?;
    let sma_200 = client
        .technical_indicators()
        .get_sma(symbol, 200, None, None)
        .await?;
    let ema_12 = client
        .technical_indicators()
        .get_ema(symbol, 12, None, None)
        .await?;
    let ema_26 = client
        .technical_indicators()
        .get_ema(symbol, 26, None, None)
        .await?;

    if let (Some(sma20), Some(sma50), Some(sma200), Some(ema12_val), Some(ema26_val)) = (
        sma_20.first().and_then(|s| s.sma),
        sma_50.first().and_then(|s| s.sma),
        sma_200.first().and_then(|s| s.sma),
        ema_12.first().and_then(|e| e.ema),
        ema_26.first().and_then(|e| e.ema),
    ) {
        println!("  Current Moving Averages:");
        println!("    SMA(20):  ${:.2}", sma20);
        println!("    SMA(50):  ${:.2}", sma50);
        println!("    SMA(200): ${:.2}", sma200);
        println!("    EMA(12):  ${:.2}", ema12_val);
        println!("    EMA(26):  ${:.2}", ema26_val);
        println!();

        // Trend analysis
        let short_term_trend = if sma20 > sma50 {
            "🟢 Bullish"
        } else {
            "🔴 Bearish"
        };
        let long_term_trend = if sma50 > sma200 {
            "🟢 Bullish"
        } else {
            "🔴 Bearish"
        };
        let golden_cross = sma50 > sma200 && sma20 > sma50;
        let death_cross = sma50 < sma200 && sma20 < sma50;

        println!("  Trend Analysis:");
        println!("    Short-term (20 vs 50):   {}", short_term_trend);
        println!("    Long-term (50 vs 200):   {}", long_term_trend);

        if golden_cross {
            println!("    ⭐ Golden Cross detected - Strong bullish signal!");
        } else if death_cross {
            println!("    ⚠️  Death Cross detected - Strong bearish signal!");
        } else {
            println!("    📊 No major crossover signals");
        }
    }
    println!();

    // 2. RSI Momentum Analysis
    println!("⚡ RSI MOMENTUM ANALYSIS");
    println!("─────────────────────────────────────────────────────────");
    let rsi_14 = client
        .technical_indicators()
        .get_rsi(symbol, 14, None, None)
        .await?;

    if !rsi_14.is_empty() {
        println!("  RSI(14) Last 5 Days:");
        for point in rsi_14.iter().take(5) {
            if let Some(rsi) = point.rsi {
                let signal = if rsi > 70.0 {
                    "🔴 Overbought"
                } else if rsi < 30.0 {
                    "🟢 Oversold"
                } else if rsi > 50.0 {
                    "🟡 Bullish"
                } else {
                    "🟠 Bearish"
                };
                let bar = "█".repeat((rsi / 5.0) as usize);
                println!("    {}: {:.1} {} {}", point.date, rsi, signal, bar);
            }
        }

        if let Some(current_rsi) = rsi_14.first().and_then(|r| r.rsi) {
            println!();
            println!("  Current RSI: {:.1}", current_rsi);
            let momentum_strength = if current_rsi > 80.0 {
                "Extremely overbought - Consider selling"
            } else if current_rsi > 70.0 {
                "Overbought - Potential pullback"
            } else if current_rsi < 20.0 {
                "Extremely oversold - Consider buying"
            } else if current_rsi < 30.0 {
                "Oversold - Potential bounce"
            } else if current_rsi > 50.0 {
                "Bullish momentum"
            } else {
                "Bearish momentum"
            };
            println!("  Assessment: {}", momentum_strength);
        }
    }
    println!();

    // 3. MACD Trend Analysis
    println!("🔄 MACD TREND ANALYSIS");
    println!("─────────────────────────────────────────────────────────");
    let macd = client
        .technical_indicators()
        .get_macd(symbol, 12, 26, 9, None, None)
        .await?;

    if !macd.is_empty() {
        println!("  MACD(12,26,9) Last 5 Days:");
        for point in macd.iter().take(5) {
            if let (Some(macd_val), Some(signal), Some(histogram)) =
                (point.macd, point.signal, point.histogram)
            {
                let trend = if macd_val > signal {
                    "🟢 Bullish"
                } else {
                    "🔴 Bearish"
                };
                let hist_bar = if histogram > 0.0 {
                    "█".repeat((histogram.abs() * 50.0).min(10.0) as usize)
                } else {
                    "▓".repeat((histogram.abs() * 50.0).min(10.0) as usize)
                };
                println!(
                    "    {}: MACD={:.3}, Signal={:.3}, Hist={:.3} {} {}",
                    point.date, macd_val, signal, histogram, trend, hist_bar
                );
            }
        }

        // Check for MACD crossovers
        if let (Some(current), Some(previous)) = (macd.get(0), macd.get(1)) {
            if let (Some(cur_macd), Some(cur_signal), Some(prev_macd), Some(prev_signal)) =
                (current.macd, current.signal, previous.macd, previous.signal)
            {
                let bullish_crossover = cur_macd > cur_signal && prev_macd <= prev_signal;
                let bearish_crossover = cur_macd < cur_signal && prev_macd >= prev_signal;

                println!();
                if bullish_crossover {
                    println!("  🚀 Bullish MACD crossover detected!");
                } else if bearish_crossover {
                    println!("  📉 Bearish MACD crossover detected!");
                } else {
                    println!("  📊 No recent MACD crossover");
                }
            }
        }
    }
    println!();

    // 4. Bollinger Bands Volatility Analysis
    println!("📊 BOLLINGER BANDS ANALYSIS");
    println!("─────────────────────────────────────────────────────────");
    let bb = client
        .technical_indicators()
        .get_bollinger_bands(symbol, 20, 2.0, None, None)
        .await?;

    if let Some(bb_current) = bb.first() {
        if let (Some(upper), Some(middle), Some(lower)) = (
            bb_current.upper_band,
            bb_current.middle_band,
            bb_current.lower_band,
        ) {
            println!("  Current Bollinger Bands (20, 2.0):");
            println!("    Upper Band:  ${:.2}", upper);
            println!("    Middle Band: ${:.2} (SMA 20)", middle);
            println!("    Lower Band:  ${:.2}", lower);

            let band_width = ((upper - lower) / middle) * 100.0;
            println!("    Band Width:  {:.2}%", band_width);
            println!();

            // Get current price estimate from middle band
            let volatility_level = if band_width > 15.0 {
                "🔥 High volatility - Bands are wide"
            } else if band_width < 5.0 {
                "😴 Low volatility - Bands are tight (squeeze)"
            } else {
                "📊 Normal volatility"
            };
            println!("  Volatility Assessment: {}", volatility_level);

            if band_width < 5.0 {
                println!("  🎯 Bollinger Band squeeze - Potential breakout coming!");
            }
        }
    }
    println!();

    // 5. Stochastic Oscillator Analysis
    println!("🎯 STOCHASTIC OSCILLATOR");
    println!("─────────────────────────────────────────────────────────");
    let stoch = client
        .technical_indicators()
        .get_stochastic(symbol, 14, 3, None, None)
        .await?;

    if let Some(stoch_current) = stoch.first() {
        if let (Some(k), Some(d)) = (stoch_current.slow_k, stoch_current.slow_d) {
            println!("  Current Stochastic (14,3):");
            println!("    %K: {:.1}", k);
            println!("    %D: {:.1}", d);

            let signal = if k > 80.0 && d > 80.0 {
                "🔴 Overbought - Potential sell signal"
            } else if k < 20.0 && d < 20.0 {
                "🟢 Oversold - Potential buy signal"
            } else if k > d {
                "🟡 Bullish momentum"
            } else {
                "🟠 Bearish momentum"
            };
            println!("  Signal: {}", signal);

            // Check for crossovers
            if let Some(stoch_prev) = stoch.get(1) {
                if let (Some(prev_k), Some(prev_d)) = (stoch_prev.slow_k, stoch_prev.slow_d) {
                    let bullish_cross = k > d && prev_k <= prev_d;
                    let bearish_cross = k < d && prev_k >= prev_d;

                    if bullish_cross {
                        println!("  🚀 Bullish %K/%D crossover!");
                    } else if bearish_cross {
                        println!("  📉 Bearish %K/%D crossover!");
                    }
                }
            }
        }
    }
    println!();

    // 6. Average True Range (Volatility)
    println!("📈 VOLATILITY ANALYSIS (ATR)");
    println!("─────────────────────────────────────────────────────────");
    let atr = client
        .technical_indicators()
        .get_atr(symbol, 14, None, None)
        .await?;

    if let Some(atr_current) = atr.first() {
        if let Some(atr_val) = atr_current.atr {
            println!("  Average True Range (14): ${:.2}", atr_val);

            // Calculate ATR as percentage if we have price data
            if let Some(middle_band) = bb.first().and_then(|b| b.middle_band) {
                let atr_percent = (atr_val / middle_band) * 100.0;
                println!("  ATR as % of Price: {:.2}%", atr_percent);

                let volatility_assessment = if atr_percent > 3.0 {
                    "High volatility - Large price swings expected"
                } else if atr_percent < 1.0 {
                    "Low volatility - Small price movements"
                } else {
                    "Moderate volatility - Normal price action"
                };
                println!("  Assessment: {}", volatility_assessment);
            }
        }
    }
    println!();

    // 7. Williams %R
    println!("📉 WILLIAMS %R ANALYSIS");
    println!("─────────────────────────────────────────────────────────");
    let williams = client
        .technical_indicators()
        .get_williams_r(symbol, 14, None, None)
        .await?;

    if let Some(wr_current) = williams.first() {
        if let Some(wr) = wr_current.williams_r {
            println!("  Williams %R (14): {:.1}", wr);

            let wr_signal = if wr > -20.0 {
                "🔴 Overbought (> -20)"
            } else if wr < -80.0 {
                "🟢 Oversold (< -80)"
            } else {
                "📊 Normal range"
            };
            println!("  Signal: {}", wr_signal);
        }
    }
    println!();

    // 8. Technical Analysis Summary
    println!("🎯 TECHNICAL ANALYSIS SUMMARY");
    println!("─────────────────────────────────────────────────────────");
    println!("  📊 Trend Indicators:");
    println!("     • Moving Averages: Check SMA crossovers above");
    println!("     • MACD: Trend direction and momentum");
    println!();
    println!("  ⚡ Momentum Indicators:");
    println!("     • RSI: Overbought/oversold conditions");
    println!("     • Stochastic: Short-term momentum");
    println!("     • Williams %R: Price position in recent range");
    println!();
    println!("  📈 Volatility Indicators:");
    println!("     • Bollinger Bands: Price volatility and squeezes");
    println!("     • ATR: Average daily price movement");
    println!();
    println!("  🔍 Key Signals to Watch:");
    println!("     • Golden/Death Cross (50/200 SMA)");
    println!("     • MACD line crossing signal line");
    println!("     • RSI above 70 (overbought) or below 30 (oversold)");
    println!("     • Bollinger Band squeezes (low volatility before breakouts)");
    println!("     • Stochastic crossovers in extreme zones");
    println!();

    println!("═══════════════════════════════════════════════════════════");
    println!("⚠️  DISCLAIMER: This is for educational purposes only.");
    println!("Technical analysis should be combined with fundamental");
    println!("analysis and proper risk management. Past performance");
    println!("does not guarantee future results.");
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}
