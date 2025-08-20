use anyhow::Result;
use clap::Parser;
use web_novel_scraper::{HtmlFetcher, NarouNovelScraper, NovelType, RequestDelayConfig};
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Fetch all episodes from Narou novel", long_about = None)]
struct Args {
    /// Novel code (ncode)
    #[arg(short, long)]
    ncode: String,

    /// Novel type (short or serial)
    #[arg(short = 't', long, value_enum)]
    novel_type: NovelTypeArg,

    /// Total number of episodes (required for serial novels)
    #[arg(short = 'e', long)]
    episodes: Option<u32>,

    /// Output directory
    #[arg(short, long, default_value = "./output")]
    output: PathBuf,

    /// Minimum delay between requests in milliseconds
    #[arg(long, default_value_t = 1000)]
    min_delay: u64,

    /// Maximum delay between requests in milliseconds
    #[arg(long, default_value_t = 3000)]
    max_delay: u64,

    /// Use Nocturne (18+) site instead of regular Narou
    #[arg(long)]
    nocturne: bool,

    /// Save as single file instead of separate files
    #[arg(long)]
    single_file: bool,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum NovelTypeArg {
    Short,
    Serial,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // 引数検証
    let novel_type = match args.novel_type {
        NovelTypeArg::Short => NovelType::ShortStory,
        NovelTypeArg::Serial => {
            let total_episodes = args.episodes
                .ok_or_else(|| anyhow::anyhow!("--episodes is required for serial novels"))?;
            NovelType::Serial { total_episodes }
        }
    };

    // Fetcherの設定
    let fetcher = HtmlFetcher::default();
    let delay_config = RequestDelayConfig::new(args.min_delay, args.max_delay);
    fetcher.set_delay_config(delay_config);

    // スクレイパーの作成
    let scraper = if args.nocturne {
        NarouNovelScraper::new_nocturne(fetcher)
    } else {
        NarouNovelScraper::new(fetcher)
    };

    println!("🔍 Fetching novel: {}", args.ncode);
    println!("📚 Type: {:?}", novel_type);
    if args.nocturne {
        println!("🔞 Using Nocturne site");
    }
    println!("📁 Output directory: {}", args.output.display());
    println!();

    // 小説の取得
    let novel_content = scraper.fetch_all_episodes(&args.ncode, novel_type).await?;

    println!("\n✅ Successfully fetched {} episodes", novel_content.episode_count());
    println!("📊 Total size: {} bytes", novel_content.total_size_bytes());

    // 出力ディレクトリの作成
    let output_dir = if args.single_file {
        args.output.clone()
    } else {
        args.output.join(&args.ncode)
    };
    
    fs::create_dir_all(&output_dir)?;

    // ファイルの保存
    if args.single_file {
        // 単一ファイルとして保存
        let file_path = output_dir.join(format!("{}.html", args.ncode));
        let mut combined_html = String::new();
        
        for episode in &novel_content.episodes {
            combined_html.push_str(&format!("<!-- Episode {} -->\n", episode.episode_number));
            combined_html.push_str(&episode.html);
            combined_html.push_str("\n\n");
        }
        
        fs::write(&file_path, combined_html)?;
        println!("\n💾 Saved to: {}", file_path.display());
    } else {
        // エピソードごとに個別ファイルとして保存
        for episode in &novel_content.episodes {
            let filename = match novel_content.novel_type {
                NovelType::ShortStory => format!("{}.html", args.ncode),
                NovelType::Serial { .. } => format!("{:04}.html", episode.episode_number),
            };
            
            let file_path = output_dir.join(&filename);
            fs::write(&file_path, &episode.html)?;
            
            if novel_content.episode_count() <= 10 {
                println!("💾 Saved: {}", file_path.display());
            }
        }
        
        if novel_content.episode_count() > 10 {
            println!("\n💾 All episodes saved to: {}", output_dir.display());
        }
    }

    // メタデータの保存
    let metadata_path = output_dir.join("metadata.json");
    let metadata = serde_json::json!({
        "ncode": novel_content.ncode,
        "type": match novel_content.novel_type {
            NovelType::ShortStory => "short".to_string(),
            NovelType::Serial { total_episodes } => format!("serial_{}", total_episodes),
        },
        "episode_count": novel_content.episode_count(),
        "total_bytes": novel_content.total_size_bytes(),
        "nocturne": args.nocturne,
        "fetched_at": chrono::Utc::now().to_rfc3339(),
    });
    
    fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;
    println!("📄 Metadata saved to: {}", metadata_path.display());

    Ok(())
}