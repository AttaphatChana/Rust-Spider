//! cargo run --example cache_chrome_hybrid --features="spider/sync spider/chrome spider/cache_chrome_hybrid"
extern crate spider;

use html2md::parse_html; // part of spider rs crate
extern crate tokio;
use spider::website::{Website};
use std::io::stdout;
use std::io::Write;

use http_cache_reqwest::{CACacheManager, CacheManager};

#[tokio::main]
async fn main() {
    // let a = CACacheManager::default().get()

    let mut website: Website = Website::new("https://heygoody.com/")
        .with_caching(false)
        .build()
        .unwrap();
    website.with_limit(200);
    let mut rx2 = website.subscribe(100).unwrap();


    //let mut v = Vec::new();
    println!("======sitemap crawler========");
    tokio::spawn(async move {
        let mut stdout = stdout();

        while let Ok(res) = rx2.recv().await {
            let message = format!("Crawling - {:?}\n", res.get_url());
            let _ = stdout.write_all(message.as_bytes());
            //let a = String::from(res.get_url());
            //v.push(a);
        }
    });

    website.crawl_sitemap().await;

    for i in website.get_links() {
        let mut w = Website::new(i.to_string().as_str()).build().unwrap();
        w.with_limit(2);
        w.scrape_sitemap().await;
        for j in w.get_pages().unwrap().iter() {

            let a = parse_html(&j.get_html(), false);
            println!("{:?}", a);
        }
    }
    website.unsubscribe();

    println!("====== chrome/http ========");
    let mut rx2 = website.subscribe(100).unwrap();


    tokio::spawn(async move {
        println!("12312321");
        let mut stdout = stdout();
        //let mut v = Vec::new();

        while let Ok(res) = rx2.recv().await {
            let message = format!("Crawling 2 - {:?}\n", res.get_url());
            let _ = stdout.write_all(message.as_bytes());
        }
    });

    website.crawl_sitemap_chrome().await;

    for i in website.get_links() {
        let mut w = Website::new(i.to_string().as_str()).build().unwrap();
        w.with_limit(2);
        w.scrape_sitemap().await;
        for j in w.get_pages().unwrap().iter() {
            let a = parse_html(&j.get_html(), false);
            println!("{:?}", a);
        }
    }
    website.unsubscribe();


    // for smart crawler

    println!("======smart crawler========");

    let mut rx2 = website.subscribe(100).unwrap();

    tokio::spawn(async move {

        let mut stdout = stdout();
        //let mut v = Vec::new();

        while let Ok(res) = rx2.recv().await {
            let message = format!("Crawling 3 - {:?}\n", res.get_url());
            let _ = stdout.write_all(message.as_bytes());
            //let a = String::from(res.get_url());
            //v.push(a);
        }
    });

    website.crawl_smart().await;

    for i in website.get_links() {
        let mut w = Website::new(i.to_string().as_str()).build().unwrap();
        w.with_limit(2);
        w.scrape_sitemap().await;
        for j in w.get_pages().unwrap().iter() {
            let a = parse_html(&j.get_html(), false);
            println!("{:?}", a);
        }
    }
    website.unsubscribe();
}
