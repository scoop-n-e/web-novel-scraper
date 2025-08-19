mod fetcher;

use anyhow::Result;
use fetcher::{HtmlFetcher, UserAgentMode};

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== User Agent Mode Examples ===\n");

    println!("1. RandomEveryRequest mode (default):");
    let fetcher = HtmlFetcher::new()?;
    let url = "https://example.com";
    let html = fetcher.fetch(url).await?;
    println!("   Fetched {} bytes", html.len());
    println!("   Fixed UA: {:?}\n", fetcher.get_current_user_agent());

    println!("2. Fixed mode with custom UA:");
    let fetcher_custom = HtmlFetcher::with_fixed_user_agent(
        "MyNovelScraper/1.0 (compatible)".to_string()
    )?;
    let html = fetcher_custom.fetch(url).await?;
    println!("   Fetched {} bytes", html.len());
    println!("   Fixed UA: {:?}\n", fetcher_custom.get_current_user_agent());

    println!("3. Fixed mode with random-generated UA:");
    let fetcher_fixed = HtmlFetcher::with_mode(UserAgentMode::Fixed)?;
    let generated_ua = fetcher_fixed.set_user_agent_from_random();
    println!("   Generated UA: {}", generated_ua);
    let html = fetcher_fixed.fetch(url).await?;
    println!("   Fetched {} bytes", html.len());
    let html2 = fetcher_fixed.fetch(url).await?;
    println!("   Fetched again: {} bytes (same UA)", html2.len());

    println!("4. Switching modes at runtime:");
    let fetcher_dynamic = HtmlFetcher::new()?;
    
    fetcher_dynamic.set_user_agent("CustomBot/2.0".to_string());
    println!("   Set custom UA: {:?}", fetcher_dynamic.get_current_user_agent());
    
    fetcher_dynamic.set_user_agent_from_random();
    println!("   Set random-generated UA: {:?}", fetcher_dynamic.get_current_user_agent());
    
    fetcher_dynamic.set_random_mode();
    println!("   Switched to RandomEveryRequest mode: {:?}\n", fetcher_dynamic.get_current_user_agent());

    println!("5. Using with cookies:");
    let fetcher_with_cookies = HtmlFetcher::new()?;
    fetcher_with_cookies.set_user_agent("NovelReader/1.0".to_string());
    
    let cookies = vec![
        ("session_id", "abc123"),
        ("language", "ja"),
    ];
    
    let html = fetcher_with_cookies.fetch_with_options(
        url,
        Some(cookies),
        None,
    ).await?;
    println!("   Fetched with cookies: {} bytes", html.len());
    
    fetcher_with_cookies.add_cookie(url, "preferences=dark_mode")?;
    println!("   Added cookie to jar");

    println!("\n6. Override with one-time custom UA:");
    let fetcher_override = HtmlFetcher::with_fixed_user_agent("BaseUA/1.0".to_string())?;
    let html = fetcher_override.fetch_with_options(
        url,
        None,
        Some("TemporaryUA/2.0"),
    ).await?;
    println!("   Fetched with temporary UA override: {} bytes", html.len());
    println!("   Fixed UA remains: {:?}", fetcher_override.get_current_user_agent());

    Ok(())
}