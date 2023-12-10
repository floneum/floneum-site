# Web Crawling

Web crawling with Kalosm allows developers to systematically browse the web, extract information from websites, and perform various actions on web pages. In this example, we'll explore the core concepts behind web crawling using Kalosm.

## Kalosm's Web Crawling API

Kalosm provides a powerful web crawling API through the `Page::crawl` method. This method takes a starting URL, a browsing mode, and a closure that defines the crawling logic.

```rust
Page::crawl(
    Url::parse("https://www.nytimes.com/live/2023/09/21/world/zelensky-russia-ukraine-news").unwrap(),
    BrowserMode::Static,
    // Closure defining crawling logic...
).await.unwrap();
```

## Initializing Counters

Two counters, `count` and `real_visited`, are used to keep track of page processing. These counters are implemented using `AtomicUsize` and `Arc` for thread-safe shared state.

```rust
let count = Arc::new(AtomicUsize::new(0));
let real_visited = Arc::new(AtomicUsize::new(0));
```

## Crawling Closure

The closure passed to `Page::crawl` defines the behavior for each visited page. It receives a `Page` object representing the current web page.

```rust
move |page: Page| {
    // Closure logic...
    // Must return a Pin<Box<dyn Future<Output = CrawlFeedback>>>
}
```

## Crawling Logic

Inside the closure, several steps are performed:

- Increment the `real_visited` counter to track the actual number of visited pages.
- Check the `count` counter to determine if the maximum number of pages to process has been reached. If so, stop crawling.
- Attempt to retrieve the article content from the page using `page.article().await`. If unsuccessful, skip the page.
- Check the length of the page's body. If it's less than 100 characters, skip the page.
- Print the title and body of the page.
- Increment the `count` counter.
- Return `CrawlFeedback::follow_all()` to instruct the crawler to follow all links on the current page.

```rust
let Ok(page) = page.article().await else {
    return CrawlFeedback::follow_none();
};

let body = page.body();

if body.len() < 100 {
    return CrawlFeedback::follow_none();
}

println!("Title: {}", page.title());
println!("Article:\n{}", body);

count.fetch_add(1, Ordering::SeqCst);

CrawlFeedback::follow_all()
```

## Conclusion

This example serves as a foundational guide for building web crawling applications with Kalosm. Developers can customize the crawling logic to extract specific information, handle errors, and adapt the application to their unique use cases. Understanding the provided example empowers developers to effectively use Kalosm's web crawling API for tasks such as data extraction, content analysis, and monitoring web pages.
