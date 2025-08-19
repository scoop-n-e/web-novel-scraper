use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use web_novel_scraper::{FetchOptions, HtmlFetcher, UserAgentMode};

#[derive(Parser)]
#[command(name = "fetch")]
#[command(about = "A command-line HTML fetcher with customizable user agents", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch HTML from a URL
    Get {
        /// URL to fetch
        url: String,

        /// User agent mode
        #[arg(short = 'm', long, value_enum, default_value = "random")]
        mode: Mode,

        /// Custom user agent (overrides mode)
        #[arg(short = 'u', long)]
        user_agent: Option<String>,

        /// Cookies in key=value format
        #[arg(short = 'c', long, value_delimiter = ';')]
        cookies: Vec<String>,

        /// Output file (default: stdout)
        #[arg(short = 'o', long)]
        output: Option<String>,

        /// Show only response info (status, size)
        #[arg(short = 'i', long)]
        info_only: bool,

        /// Request timeout in seconds
        #[arg(long, default_value = "10")]
        timeout: u64,
    },

    /// Test different user agent modes
    Test {
        /// URL to test
        url: String,

        /// Number of requests to make
        #[arg(short = 'n', long, default_value = "3")]
        count: usize,
    },
}

#[derive(Clone, ValueEnum)]
enum Mode {
    /// Use a different random user agent for each request
    Random,
    /// Use a fixed user agent for all requests
    Fixed,
    /// Use a custom user agent
    Custom,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get {
            url,
            mode,
            user_agent,
            cookies,
            output,
            info_only,
            timeout,
        } => {
            fetch_url(
                url,
                mode,
                user_agent,
                cookies,
                output,
                info_only,
                timeout,
            )
            .await?;
        }
        Commands::Test { url, count } => {
            test_modes(url, count).await?;
        }
    }

    Ok(())
}

async fn fetch_url(
    url: String,
    mode: Mode,
    custom_ua: Option<String>,
    cookies: Vec<String>,
    output: Option<String>,
    info_only: bool,
    timeout: u64,
) -> Result<()> {
    let fetcher = create_fetcher_with_timeout(mode, custom_ua, timeout)?;

    let parsed_cookies = if !cookies.is_empty() {
        Some(parse_cookies(&cookies)?)
    } else {
        None
    };

    let options = FetchOptions {
        cookies: parsed_cookies.as_ref().map(|c| {
            c.iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect::<Vec<_>>()
        }),
        custom_user_agent: None,
    };

    if let Some(ref ua) = fetcher.get_current_user_agent() {
        eprintln!("Using User-Agent: {}", ua);
    } else {
        eprintln!("Using random User-Agent for each request");
    }

    let html = fetcher.fetch_with_options(&url, options).await?;

    if info_only {
        println!("URL: {}", url);
        println!("Content Length: {} bytes", html.len());
        println!("Lines: {}", html.lines().count());
    } else if let Some(output_file) = output {
        std::fs::write(&output_file, &html)?;
        eprintln!("Saved {} bytes to {}", html.len(), output_file);
    } else {
        print!("{}", html);
    }

    Ok(())
}

async fn test_modes(url: String, count: usize) -> Result<()> {
    println!(
        "Testing different user agent modes with {} requests each\n",
        count
    );

    println!("1. Random User-Agent Mode:");
    let fetcher = HtmlFetcher::new()?;
    for i in 1..=count {
        let html = fetcher.fetch(&url).await?;
        println!("   Request {}: {} bytes", i, html.len());
    }

    println!("\n2. Fixed User-Agent Mode:");
    let fetcher = HtmlFetcher::with_mode(UserAgentMode::Fixed(None))?;
    let ua = fetcher.set_user_agent_from_random();
    println!("   Using: {}", ua);
    for i in 1..=count {
        let html = fetcher.fetch(&url).await?;
        println!("   Request {}: {} bytes", i, html.len());
    }

    println!("\n3. Custom User-Agent:");
    let custom_ua = "TestBot/1.0 (CLI)";
    let fetcher = HtmlFetcher::with_fixed_user_agent(custom_ua.to_string())?;
    println!("   Using: {}", custom_ua);
    for i in 1..=count {
        let html = fetcher.fetch(&url).await?;
        println!("   Request {}: {} bytes", i, html.len());
    }

    Ok(())
}

fn create_fetcher_with_timeout(
    mode: Mode,
    custom_ua: Option<String>,
    timeout_secs: u64,
) -> Result<HtmlFetcher> {
    use std::time::Duration;
    
    let timeout = Duration::from_secs(timeout_secs);
    
    match mode {
        Mode::Random => HtmlFetcher::with_config(UserAgentMode::RandomEveryRequest, timeout),
        Mode::Fixed => {
            let fetcher = HtmlFetcher::with_config(UserAgentMode::Fixed(None), timeout)?;
            fetcher.set_user_agent_from_random();
            Ok(fetcher)
        }
        Mode::Custom => {
            let ua = custom_ua.unwrap_or_else(|| "CustomBot/1.0".to_string());
            HtmlFetcher::with_config(UserAgentMode::Fixed(Some(ua)), timeout)
        }
    }
}

fn parse_cookies(cookies: &[String]) -> Result<Vec<(String, String)>> {
    cookies
        .iter()
        .map(|cookie| {
            let parts: Vec<&str> = cookie.splitn(2, '=').collect();
            if parts.len() != 2 {
                anyhow::bail!("Invalid cookie format: {}. Expected key=value", cookie);
            }
            Ok((parts[0].to_string(), parts[1].to_string()))
        })
        .collect()
}