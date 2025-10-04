//! Transcripts & Communications Dashboard
//!
//! This example demonstrates earnings transcript analysis, press release monitoring,
//! and conference call schedule tracking for investor communications.

use fmp_rs::FmpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = FmpClient::new()?;

    println!("═══════════════════════════════════════════════════════════");
    println!("        📝 TRANSCRIPTS & COMMUNICATIONS DASHBOARD 📝");
    println!("═══════════════════════════════════════════════════════════\n");

    // 1. Earnings Transcript Discovery
    println!("🔍 EARNINGS TRANSCRIPT DISCOVERY");
    println!("─────────────────────────────────────────────────────────");

    let transcript_companies = ["AAPL", "MSFT", "GOOGL"];
    let company_names = ["Apple Inc.", "Microsoft Corp.", "Alphabet Inc."];

    for (symbol, name) in transcript_companies.iter().zip(company_names.iter()) {
        println!("  📊 {} ({}) - Available Transcripts:", name, symbol);

        let transcripts = client
            .transcripts()
            .get_transcript_list(symbol, Some(2024))
            .await?;
        if !transcripts.is_empty() {
            println!();
            for transcript in transcripts.iter().take(4) {
                let quarter = transcript.quarter.as_deref().unwrap_or("N/A");
                let year = transcript.year.unwrap_or(0);
                let date = transcript.date.as_deref().unwrap_or("N/A");
                let available = transcript.is_available.unwrap_or(false);
                let duration = transcript.duration.unwrap_or(0);
                let call_type = transcript.call_type.as_deref().unwrap_or("Earnings");

                let status_emoji = if available { "✅" } else { "⏳" };
                let status_text = if available { "Available" } else { "Pending" };

                println!(
                    "    {} {} {} - {} ({})",
                    status_emoji, quarter, year, date, status_text
                );
                println!("      Type: {} | Duration: {} min", call_type, duration);

                if let Some(excerpt) = &transcript.excerpt {
                    let preview = if excerpt.len() > 100 {
                        format!("{}...", &excerpt[..97])
                    } else {
                        excerpt.clone()
                    };
                    println!("      Preview: {}", preview);
                }
                println!();
            }
        } else {
            println!("    No transcripts available for 2024");
        }
        println!();
    }

    // 2. Full Transcript Analysis
    println!("📄 FULL TRANSCRIPT ANALYSIS");
    println!("─────────────────────────────────────────────────────────");

    println!("  📖 Apple Q4 2023 Earnings Call Analysis:");
    let full_transcript = client
        .transcripts()
        .get_earnings_transcript("AAPL", 2023, 4)
        .await?;

    if let Some(transcript) = full_transcript.first() {
        let company = transcript
            .company_name
            .as_deref()
            .unwrap_or("Unknown Company");
        let date = transcript.date.as_deref().unwrap_or("N/A");
        let quarter = transcript.quarter.as_deref().unwrap_or("N/A");
        let year = transcript.year.unwrap_or(0);
        let duration = transcript.duration.unwrap_or(0);
        let analyst_count = transcript.analyst_count.unwrap_or(0);

        println!(
            "    Company: {} | Date: {} | Duration: {} minutes",
            company, date, duration
        );
        println!(
            "    Period: {} {} | Analysts Participating: {}",
            quarter, year, analyst_count
        );

        if let Some(content) = &transcript.content {
            let word_count = content.split_whitespace().count();
            let char_count = content.len();

            println!("    Content Statistics:");
            println!("      Total Characters: {}", char_count);
            println!("      Word Count: {}", word_count);
            println!(
                "      Average Words/Minute: {:.0}",
                word_count as f64 / duration as f64
            );

            // Key phrase analysis
            println!("    Key Topics Mentioned:");
            let key_phrases = [
                ("revenue", "Revenue/Sales"),
                ("growth", "Growth"),
                ("margin", "Profit Margins"),
                ("guidance", "Future Guidance"),
                ("AI", "Artificial Intelligence"),
                ("innovation", "Innovation"),
                ("market", "Market Conditions"),
                ("competition", "Competition"),
            ];

            for (phrase, topic) in key_phrases {
                let count = content.to_lowercase().matches(phrase).count();
                if count > 0 {
                    let frequency_bar = "█".repeat((count / 2).min(20));
                    println!("      {}: {} mentions {}", topic, count, frequency_bar);
                }
            }

            // Extract first few sentences as preview
            let sentences: Vec<&str> = content.split('.').take(3).collect();
            let preview = sentences.join(". ").trim().to_string();
            if !preview.is_empty() {
                println!("    Opening Statement Preview:");
                println!("      \"{}...\"", preview);
            }
        } else {
            println!("    Full content not available");
        }
    } else {
        println!("    Transcript not available for AAPL Q4 2023");
    }
    println!();

    // 3. Press Release Monitoring
    println!("📰 PRESS RELEASE MONITORING");
    println!("─────────────────────────────────────────────────────────");

    for (symbol, name) in transcript_companies.iter().zip(company_names.iter()) {
        println!("  📢 {} ({}) - Recent Press Releases:", name, symbol);

        let press_releases = client
            .transcripts()
            .get_press_releases(symbol, Some(5))
            .await?;
        if !press_releases.is_empty() {
            println!();
            for release in press_releases.iter().take(5) {
                let date = release.date.as_deref().unwrap_or("N/A");
                let title = release.title.as_deref().unwrap_or("No Title");
                let release_type = release.release_type.as_deref().unwrap_or("General");
                let source = release.source.as_deref().unwrap_or("Unknown");
                let word_count = release.word_count.unwrap_or(0);

                // Determine release type emoji
                let type_emoji = match release_type.to_lowercase().as_str() {
                    "earnings" => "💰",
                    "product" => "📱",
                    "corporate" => "🏢",
                    "acquisition" => "🤝",
                    "partnership" => "🤝",
                    _ => "📄",
                };

                println!("    {} {} - {} ({})", type_emoji, date, title, release_type);
                println!("      Source: {} | Length: {} words", source, word_count);

                if let Some(summary) = &release.summary {
                    let short_summary = if summary.len() > 120 {
                        format!("{}...", &summary[..117])
                    } else {
                        summary.clone()
                    };
                    println!("      Summary: {}", short_summary);
                }

                if let Some(tags) = &release.tags {
                    if !tags.is_empty() {
                        let tag_str = tags.join(", ");
                        println!("      Tags: {}", tag_str);
                    }
                }
                println!();
            }
        } else {
            println!("    No recent press releases found");
        }
        println!();
    }

    // 4. Conference Call Schedule
    println!("📅 CONFERENCE CALL SCHEDULE");
    println!("─────────────────────────────────────────────────────────");

    // Get upcoming calls for the next month
    let current_date = chrono::Utc::now();
    let next_month = current_date + chrono::Duration::days(30);

    let conference_calls = client
        .transcripts()
        .get_conference_schedule(
            Some(&current_date.format("%Y-%m-%d").to_string()),
            Some(&next_month.format("%Y-%m-%d").to_string()),
        )
        .await?;

    if !conference_calls.is_empty() {
        println!("  🗓️  Upcoming Conference Calls (Next 30 Days):");
        println!();
        println!("    ┌────────────┬──────┬─────────────────────────────┬─────────────────┐");
        println!("    │    Date    │ Symb │           Title             │ Type/Duration   │");
        println!("    ├────────────┼──────┼─────────────────────────────┼─────────────────┤");

        for call in conference_calls.iter().take(15) {
            let date_time = call.date_time.as_deref().unwrap_or("TBD");
            let symbol = call.symbol.as_deref().unwrap_or("N/A");
            let title = call.title.as_deref().unwrap_or("Earnings Call");
            let call_type = call.call_type.as_deref().unwrap_or("Earnings");
            let duration = call.estimated_duration.unwrap_or(60);
            let status = call.status.as_deref().unwrap_or("Scheduled");

            // Format date for display
            let display_date = if date_time.len() >= 10 {
                &date_time[..10]
            } else {
                date_time
            };

            // Truncate title for table display
            let display_title = if title.len() > 27 {
                format!("{}...", &title[..24])
            } else {
                format!("{:27}", title)
            };

            let type_duration = format!("{} ({}m)", call_type, duration);

            // Status emoji (for future use)
            let _status_emoji = match status {
                "Scheduled" => "📅",
                "In Progress" => "🔴",
                "Completed" => "✅",
                _ => "⏰",
            };

            println!(
                "    │ {} │ {:4} │ {} │ {:15} │",
                display_date, symbol, display_title, type_duration
            );

            // Show additional details for some calls
            if let Some(webcast_url) = &call.webcast_url {
                if !webcast_url.is_empty() {
                    println!(
                        "    │            │      │ Webcast: Available          │                 │"
                    );
                }
            }
        }

        println!("    └────────────┴──────┴─────────────────────────────┴─────────────────┘");

        // Summary statistics
        let earnings_calls = conference_calls
            .iter()
            .filter(|call| {
                call.call_type
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains("earnings")
            })
            .count();
        let other_calls = conference_calls.len() - earnings_calls;

        println!();
        println!("    📊 Schedule Summary:");
        println!(
            "      Total Calls: {} | Earnings: {} | Other Events: {}",
            conference_calls.len(),
            earnings_calls,
            other_calls
        );

        // Industry breakdown
        let mut industry_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for call in &conference_calls {
            if let Some(industry) = &call.industry {
                *industry_counts.entry(industry.clone()).or_insert(0) += 1;
            }
        }

        if !industry_counts.is_empty() {
            println!("      Top Industries:");
            let mut sorted_industries: Vec<_> = industry_counts.iter().collect();
            sorted_industries.sort_by(|a, b| b.1.cmp(a.1));

            for (industry, count) in sorted_industries.iter().take(5) {
                println!("        {}: {} calls", industry, count);
            }
        }
        println!();
    } else {
        println!("    No conference calls scheduled for the next 30 days");
    }

    // 5. Communication Insights & Analysis
    println!("💡 COMMUNICATION INSIGHTS & ANALYSIS");
    println!("─────────────────────────────────────────────────────────");
    println!("  📈 Earnings Call Best Practices:");
    println!("     • Listen for forward-looking guidance and management tone");
    println!("     • Pay attention to analyst question themes and concerns");
    println!("     • Note any changes in key performance indicators (KPIs)");
    println!("     • Track management's confidence level and word choice");
    println!("     • Compare current call tone to previous quarters");
    println!();
    println!("  📰 Press Release Analysis:");
    println!("     • Timing: Early releases may indicate confidence");
    println!("     • Language: Positive vs. cautious terminology patterns");
    println!("     • Frequency: Increased releases may signal major changes");
    println!("     • Content: Focus on financial metrics vs. strategic updates");
    println!("     • Source credibility: Direct company vs. third-party releases");
    println!();
    println!("  🗓️  Conference Call Strategy:");
    println!("     • Pre-call: Review previous transcripts and analyst estimates");
    println!("     • Live listening: Note real-time analyst reactions");
    println!("     • Post-call: Compare transcript to market reaction");
    println!("     • Follow-up: Track subsequent press releases and filings");
    println!("     • Comparative analysis: Benchmark against industry peers");
    println!();
    println!("  🔍 Key Metrics to Track:");
    println!("     • Call duration: Shorter calls may indicate fewer questions/concerns");
    println!("     • Analyst participation: High participation suggests strong interest");
    println!("     • Management accessibility: Willingness to answer tough questions");
    println!("     • Guidance clarity: Specific vs. vague future projections");
    println!("     • Question themes: Recurring topics across multiple quarters");
    println!();
    println!("  📊 Investment Decision Integration:");
    println!("     • Combine transcript analysis with financial statement review");
    println!("     • Cross-reference management statements with actual results");
    println!("     • Track management credibility over multiple quarters");
    println!("     • Use communication tone as early warning indicator");
    println!("     • Correlate communication patterns with stock performance");
    println!();

    println!("═══════════════════════════════════════════════════════════");
    println!("⚠️  COMMUNICATION ANALYSIS DISCLAIMER: Transcripts and press");
    println!("releases provide qualitative insights but should be combined");
    println!("with quantitative financial analysis. Management statements");
    println!("are forward-looking and may not reflect actual future results.");
    println!("Always verify information through multiple sources and SEC filings.");
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}
