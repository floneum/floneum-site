# Web Crawling

Web crawling with Kalosm allows developers to systematically browse the web, extract information from websites, and perform various actions on web pages. In this example, we'll explore the core concepts behind web crawling using Kalosm.

## Kalosm's Web Crawling API

Kalosm provides a powerful web crawling API through the `Page::crawl` method. This method takes a starting URL, a browsing mode, and a closure that defines the crawling logic. The browser mode controls if the crawler should only retrieve the HTML content of the page or if it should also run a full headless browser to execute JavaScript and load dynamic content. The closure defines the behavior for each visited page. It receives a `Page` object representing the current web page. The closure must return a `Pin<Box<dyn Future<Output = CrawlFeedback>>>` to instruct the crawler to follow links on the current page.

```rust
{{#include src/doc_snippets/web_scraping.rs:create_crawler}}
```

## Reading the HTML Content of a Page

Kalosm provides utilities to extract information from the HTML content of a page. The `Page::article` method extracts an article from a webpage. From that article you can extract the title, and the text of the page. If you need lower level access to the HTML content, you can use the `Page::html` method to get the raw HTML content of the page.

```rust
{{#include src/doc_snippets/web_scraping.rs:read_article}}
```

## Conclusion

This example serves as a foundational guide for building web crawling applications with Kalosm. You can combine a web crawler with a [LLM](./llms/index.md) to perform more complex analysis on the content of web pages. Or you could train a [custom classifier](https://github.com/floneum/floneum/blob/master/interfaces/kalosm-learning/examples/classify.rs) to classify web pages based on their content.
