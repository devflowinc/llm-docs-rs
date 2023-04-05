//! `cargo run --example download`
use spider::utils::log;
use spider::website::Website;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

pub async fn scrape_site(website_name: &str) -> Result<(), Box<dyn Error>> {
    // view the target dist for the downloads
    std::fs::create_dir_all("./target/downloads").unwrap_or_default();

    let mut website: Website = Website::new("https://docs.rs/robokit/0.2.0/robokit/index.html");
    website.configuration.http2_prior_knowledge = true;
    website.configuration.delay = 0;

    website.scrape().await;

    log::info!("Scraped website: {}", website_name);
    log::info!("Website has {} pages", website.get_pages().unwrap().len());

    for page in website.get_pages().expect("Website to have pages").iter() {
        log::info!("Downloading: {}", page.get_url());
        let download_file = page.get_url().clone();
        let download_file = download_file.replace(website_name, "");
        let download_file = download_file.replace(".", "-");
        let download_file = download_file.replace("/", "-");

        let download_file = if download_file.starts_with("-") {
            &download_file[1..]
        } else {
            &download_file
        };

        let download_file = if download_file.is_empty() {
            "index"
        } else {
            &download_file
        };

        let download_file = format!("./target/downloads/{}.html", download_file);

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&download_file)
            .expect("Unable to open file");

        let html = page.get_html();
        let html = html.as_bytes();

        file.write_all(html).expect("Unable to write data");

        log("downloaded", download_file)
    }

    Ok(())
}