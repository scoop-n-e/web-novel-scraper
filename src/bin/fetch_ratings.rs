use anyhow::Result;
use clap::Parser;
use web_novel_scraper::{HtmlFetcher, NarouRatingScraper, RequestDelayConfig};

#[derive(Parser, Debug)]
#[command(author, version, about = "Fetch all ratings from Narou user page", long_about = None)]
struct Args {
    /// User ID to fetch ratings for
    #[arg(short, long)]
    user_id: u32,

    /// Minimum delay between requests in milliseconds
    #[arg(long, default_value_t = 1000)]
    min_delay: u64,

    /// Maximum delay between requests in milliseconds
    #[arg(long, default_value_t = 3000)]
    max_delay: u64,

    /// Output format (json, csv, or text)
    #[arg(short, long, default_value = "text")]
    format: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Fetcherを設定（遅延設定付き）
    let fetcher = HtmlFetcher::default();
    
    // RequestDelayConfigを設定
    let delay_config = RequestDelayConfig::new(args.min_delay, args.max_delay);
    fetcher.set_delay_config(delay_config);
    
    let scraper = NarouRatingScraper::new(fetcher);

    println!("Fetching ratings for user ID: {}", args.user_id);
    println!("This may take a while for users with many ratings...\n");

    // 評価を取得
    let ratings = scraper.fetch_all_ratings(args.user_id).await?;

    println!("\n✅ Successfully fetched {} ratings\n", ratings.len());

    // 出力形式に応じて表示
    match args.format.as_str() {
        "json" => {
            // JSON形式で出力
            let json_output = serde_json::json!({
                "user_id": args.user_id,
                "total_count": ratings.len(),
                "ratings": ratings.iter().map(|r| {
                    serde_json::json!({
                        "ncode": r.ncode,
                        "rating_point": r.rating_point,
                        "first_rating_date": r.first_rating_date,
                        "last_rating_date": r.last_rating_date,
                    })
                }).collect::<Vec<_>>()
            });
            println!("{}", serde_json::to_string_pretty(&json_output)?);
        }
        "csv" => {
            // CSV形式で出力
            println!("ncode,rating_point,first_rating_date,last_rating_date");
            for rating in &ratings {
                println!(
                    "{},{},{},{}",
                    rating.ncode,
                    rating.rating_point,
                    rating.first_rating_date,
                    rating.last_rating_date.as_ref().unwrap_or(&String::from(""))
                );
            }
        }
        _ => {
            // テキスト形式で出力（デフォルト）
            for (i, rating) in ratings.iter().enumerate() {
                println!("--- Rating #{} ---", i + 1);
                println!("NCode: {}", rating.ncode);
                println!("Rating: {} stars", rating.rating_point);
                println!("First rated: {}", rating.first_rating_date);
                if let Some(ref last_date) = rating.last_rating_date {
                    println!("Last rated: {}", last_date);
                }
                println!();
            }
        }
    }

    Ok(())
}