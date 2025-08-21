use anyhow::Result;
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path, PathBuf};

fn extract_api_spec_from_html(html_content: &str) -> Result<String> {
    let document = Html::parse_document(html_content);
    
    // タイトルを取得
    let title_selector = Selector::parse("h2.c-title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|el| el.text().collect::<String>())
        .unwrap_or_else(|| "API仕様書".to_string());
    
    // 仕様書のメインコンテンツを取得
    let doc_selector = Selector::parse("div.c-document").unwrap();
    let doc_elem = document.select(&doc_selector).next();
    
    if doc_elem.is_none() {
        return Err(anyhow::anyhow!("No document content found"));
    }
    
    let mut markdown = format!("# {}\n\n", title.trim());
    
    // 各セクションを処理
    let section_selector = Selector::parse("section.c-document__section").unwrap();
    
    for section in document.select(&section_selector) {
        let section_html = section.html();
        
        // HTMLをMarkdownに変換
        let section_md = html2md::parse_html(&section_html);
        
        // 不要な空白行を削除
        let cleaned_md = section_md
            .lines()
            .filter(|line| !line.trim().is_empty() || line.is_empty())
            .collect::<Vec<_>>()
            .join("\n");
        
        markdown.push_str(&cleaned_md);
        markdown.push_str("\n\n");
    }
    
    // テーブルの整形を改善
    markdown = markdown.replace("|  ", "| ");
    markdown = markdown.replace("  |", " |");
    
    Ok(markdown)
}

fn process_all_api_docs() -> Result<()> {
    let base_dir = Path::new("/home/scoop/web-novel-scraper/target_pages/api_documentation/source_html");
    let output_dir = Path::new("/home/scoop/web-novel-scraper/target_pages/api_documentation/extracted_specs");
    
    // 出力ディレクトリを作成
    fs::create_dir_all(output_dir)?;
    
    // 処理対象のファイル
    let api_files = vec![
        ("narou_api", vec![
            "なろう小説API _ なろうデベロッパー.html",
            "なろう小説ランキングAPI _ なろうデベロッパー.html",
            "なろう殿堂入りAPI _ なろうデベロッパー.html",
            "なろうユーザ検索API _ なろうデベロッパー.html",
        ]),
        ("nocturne_api", vec![
            "なろうR18小説API _ なろうデベロッパー.html",
        ]),
    ];
    
    for (folder, files) in api_files {
        for file_name in files {
            let input_path = base_dir.join(folder).join(file_name);
            
            if !input_path.exists() {
                eprintln!("File not found: {:?}", input_path);
                continue;
            }
            
            println!("Processing: {:?}", input_path);
            
            // HTMLファイルを読み込み
            let html_content = fs::read_to_string(&input_path)?;
            
            // HTMLからMarkdownに変換
            match extract_api_spec_from_html(&html_content) {
                Ok(markdown_content) => {
                    // 出力ファイル名を決定
                    let output_name = file_name.replace(" _ なろうデベロッパー.html", ".md");
                    let output_path = output_dir.join(output_name);
                    
                    // Markdownファイルとして保存
                    fs::write(&output_path, markdown_content)?;
                    
                    println!("  -> Saved to: {:?}", output_path);
                }
                Err(e) => {
                    eprintln!("  -> Error: {}", e);
                }
            }
        }
    }
    
    println!("\nAPI specification extraction completed!");
    Ok(())
}

fn main() -> Result<()> {
    process_all_api_docs()
}