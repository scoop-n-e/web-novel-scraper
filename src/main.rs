mod fetcher;

use anyhow::Result;
use fetcher::HtmlFetcher;

#[tokio::main]
async fn main() -> Result<()> {
    let fetcher = HtmlFetcher::new()?;

    let url = "https://example.com";
    println!("Fetching HTML from: {}", url);
    let html = fetcher.fetch(url).await?;
    println!("HTML length: {} characters", html.len());
    println!("First 500 characters:\n{}", &html[..500.min(html.len())]);

    let cookies = vec![
        ("session_id", "abc123"),
        ("user_pref", "dark_mode"),
    ];
    let custom_user_agent = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36";
    
    println!("\n\nFetching with custom options...");
    let html_with_options = fetcher.fetch_with_options(
        url,
        Some(cookies),
        Some(custom_user_agent),
    ).await?;
    println!("HTML with options length: {} characters", html_with_options.len());

    fetcher.add_cookie(url, "new_cookie=value123")?;
    println!("\nAdded new cookie to jar");

    Ok(())
}