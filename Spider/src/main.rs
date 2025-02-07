extern crate http_cache_reqwest;
extern crate tokio;
use crate::http_cache_reqwest::CACacheManager;
// use crate::spider::http_cache_reqwest::CacheManager;
use crate::tokio::io::AsyncWriteExt;
use crate::tokio::io;
use spider::string_concat::{string_concat, string_concat_impl};
//use spider::tokio::io;
use spider::website::Website;
use std::io::stdout;
use std::io::Write;
use std::ops::Deref;
use std::sync::Arc;
use http_cache_reqwest::CacheManager;
use spider::chromiumoxide::cdp::browser_protocol::accessibility::AxPropertyName::Atomic;

static GLOBAL_URL_COUNT: AtomicUsize = AtomicUsize::new(0);


#[tokio::main]
async fn main() {
    //let a = CACacheManager::default();
    let mut website: Website = Website::new("https://heygoody.com/")
        .with_caching(true)
        .build()
        .unwrap();
    //website.with_limit(100);
    let mut rx2 = website.subscribe(500).unwrap();

    let start = std::time::Instant::now();

    tokio::spawn(async move {
        let mut stdout = stdout();
        //let mut v = Vec::new();
        while let Ok(res) = rx2.recv().await {
            let message = format!("Crawling - {:?}\n", res.get_url());
            let _ = stdout.write_all(message.as_bytes());

        }
    });


    website.crawl_sitemap().await;


    for i in website.get_links().iter(){
        let mut u = Website::new(i).build().unwrap();
        u.with_limit(2);
        u.scrape_sitemap().await;
        for i in u.get_pages().unwrap().iter(){
            println!("{:?}", parse_html(&i.get_html(),false));
        }
    }

    website.unsubscribe();



    let mut rx2 = website.subscribe(500).unwrap();

    let subscription = async move {
        let mut stdout = stdout();
        let mut v = Vec::new();
        while let Ok(res) = rx2.recv().await {
            let b = res.get_url().to_string();
            v.push(b);
            let message = format!("Crawling2 - {:?}\n", res.get_url());
            let _ = stdout.write_all(message.as_bytes());
        }


        for i in v{
            let mut u = Website::new(&i).build().unwrap();
            u.with_limit(2);
            u.scrape_sitemap().await;
            for i in u.get_pages().unwrap().iter(){
                println!("{:?}", parse_html(&i.get_html(),false));
            }
        }
        GLOBAL_URL_COUNT.fetch_add(1, Ordering::Relaxed);

        println!("end ========= ");

    };

    let crawl = async move {
        website.crawl_sitemap().await;
        website.unsubscribe();
    };


    tokio::pin!(subscription);
    //
    tokio::select! {
        _ = crawl => (),
        _ = subscription => (),
    };



    let duration = start.elapsed();

    println!(
        "Time elapsed in website.crawl() is: {:?} for total pages: {:?}",
        duration, GLOBAL_URL_COUNT
    )
}
