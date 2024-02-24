#[tokio::main]
async fn main() {
    {
        // ANCHOR: create_crawler
        use kalosm::language::*;

        Page::crawl(
            Url::parse("https://floneum.com/blog/floneum_0_2").unwrap(),
            BrowserMode::Static,
            move |_page: Page| {
                Box::pin(async move { CrawlFeedback::follow_all() })
                    as std::pin::Pin<Box<dyn std::future::Future<Output = CrawlFeedback>>>
            },
        )
        .await
        .unwrap();
        // ANCHOR_END: create_crawler
    }
    {
        // ANCHOR: read_article
        use kalosm::language::*;

        Page::crawl(
            Url::parse("https://floneum.com/blog/floneum_0_2").unwrap(),
            BrowserMode::Static,
            move |page: Page| {
                Box::pin(async move {
                    println!("URL: {}", page.url());
                    println!("HTML: {:?}", page.html().await);

                    let Ok(page) = page.article().await else {
                        return CrawlFeedback::follow_none();
                    };

                    println!("Title: {}", page.title());
                    println!("Article:\n{}", page.body());

                    CrawlFeedback::follow_all()
                })
                    as std::pin::Pin<Box<dyn std::future::Future<Output = CrawlFeedback>>>
            },
        )
        .await
        .unwrap();
        // ANCHOR_END: read_article
    }
}
