
use std::fs::OpenOptions; // for writing file
use html2md::parse_html; // part of spider rs crate
use spider::website::Website;
use spider::tokio;
use env_logger::Env;
use spider::percent_encoding::{percent_encode, NON_ALPHANUMERIC};

#[tokio::main]
async fn main() {

    // start crawling sitemaps
    let url = "https://heygoody.com/";
    let mut website = Website::new(url)
        .with_chrome_intercept(spider::features::chrome_common::RequestInterceptConfiguration::new(true))
        .with_limit(50)
        .build()
        .unwrap();

    website.scrape_sitemap().await;

    // convert html to md and stored as a file
    use std::io::{Write}; // use write just for download markdown content, nothing else
    std::fs::create_dir_all("./target/downloads").unwrap_or_default();
    let env = Env::default()
        .filter_or("RUST_LOG", "info")
        .write_style_or("RUST_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    for page in website.get_pages().unwrap().iter() {
        let download_file = percent_encode(page.get_url().as_bytes(), NON_ALPHANUMERIC).to_string();
        let download_file = if download_file.is_empty() {
            "index"
        } else {
            &download_file
        };
        let download_file = format!("./target/downloads/{}.md", download_file);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&download_file)
            .expect("Unable to open file");
        let a = page.get_html();
        let b = parse_html(&a.to_string(), false);
        file.write_all(b.as_bytes()).unwrap_or_default();
    }
    // swtich to chrome
    website.crawl_sitemap_chrome().await;
    // make smart crawler
    website.crawl_smart().await;

}
