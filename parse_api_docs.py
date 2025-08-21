#!/usr/bin/env python3
import os
import re
from pathlib import Path
from bs4 import BeautifulSoup
import html2text

def extract_api_spec_from_html(html_file_path):
    """HTMLファイルからAPI仕様書部分を抽出してMarkdownに変換"""
    
    with open(html_file_path, 'r', encoding='utf-8') as f:
        html_content = f.read()
    
    soup = BeautifulSoup(html_content, 'html.parser')
    
    # タイトルを取得
    title_elem = soup.find('h2', class_='c-title')
    title = title_elem.get_text(strip=True) if title_elem else "API仕様書"
    
    # 仕様書のメインコンテンツを取得
    doc_elem = soup.find('div', class_='c-document')
    
    if not doc_elem:
        print(f"Warning: No document content found in {html_file_path}")
        return None
    
    # html2textの設定
    h = html2text.HTML2Text()
    h.ignore_links = False
    h.body_width = 0  # 改行を無効化
    h.ignore_images = True
    
    # HTMLをMarkdownに変換
    markdown_content = f"# {title}\n\n"
    
    # 各セクションを処理
    sections = doc_elem.find_all('section', class_='c-document__section')
    
    for section in sections:
        # セクションのヘッダーを処理
        headers = section.find_all(['h3', 'h4', 'h5'])
        
        # セクション内のコンテンツをMarkdownに変換
        section_html = str(section)
        section_md = h.handle(section_html)
        
        # 不要な空白行を削除
        section_md = re.sub(r'\n\s*\n', '\n\n', section_md)
        
        markdown_content += section_md + "\n"
    
    # テーブルの整形を改善
    markdown_content = re.sub(r'\|\s+', '| ', markdown_content)
    markdown_content = re.sub(r'\s+\|', ' |', markdown_content)
    
    return markdown_content

def process_all_api_docs():
    """すべてのAPI仕様書HTMLファイルを処理"""
    
    base_dir = Path('/home/scoop/web-novel-scraper/target_pages/api_documentation/source_html')
    output_dir = Path('/home/scoop/web-novel-scraper/target_pages/api_documentation/extracted_specs')
    
    # 出力ディレクトリを作成
    output_dir.mkdir(parents=True, exist_ok=True)
    
    # 処理対象のファイル
    api_files = {
        'narou_api': [
            'なろう小説API _ なろうデベロッパー.html',
            'なろう小説ランキングAPI _ なろうデベロッパー.html',
            'なろう殿堂入りAPI _ なろうデベロッパー.html',
            'なろうユーザ検索API _ なろうデベロッパー.html',
        ],
        'nocturne_api': [
            'なろうR18小説API _ なろうデベロッパー.html',
        ]
    }
    
    for folder, files in api_files.items():
        for file_name in files:
            input_path = base_dir / folder / file_name
            
            if not input_path.exists():
                print(f"File not found: {input_path}")
                continue
            
            print(f"Processing: {input_path}")
            
            # HTMLからMarkdownに変換
            markdown_content = extract_api_spec_from_html(input_path)
            
            if markdown_content:
                # 出力ファイル名を決定
                output_name = file_name.replace(' _ なろうデベロッパー.html', '.md')
                output_path = output_dir / output_name
                
                # Markdownファイルとして保存
                with open(output_path, 'w', encoding='utf-8') as f:
                    f.write(markdown_content)
                
                print(f"  -> Saved to: {output_path}")
    
    print("\nAPI specification extraction completed!")

if __name__ == "__main__":
    process_all_api_docs()