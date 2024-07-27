// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release
#![allow(non_snake_case)]
pub use crate::blog_route::BookRoute as BlogRoute;
use crate::kalosm::learn::KalosmLearn;
use crate::{docs::BookRoute, kalosm_docs::BookRoute as KalosmBookRoute, search::SearchModal};
use blog::Blog;
use dioxus::prelude::*;

use home::Home;
use kalosm::{KalosmHeaderFooter, KalosmHome};
use learn::Learn;
use serde::{Deserialize, Serialize};

mod blog;
mod components;
#[cfg(feature = "check_docs")]
mod doc_snippets;
mod footer;
mod header;
mod home;
mod kalosm;
mod learn;
mod plugin;
mod search;
mod shortcut;
mod structured_generation_visualized;
mod table_of_contents;

#[component]
fn HeaderFooter() -> Element {
    rsx! {
        SearchModal { index: &SEARCH_INDEX }
        header::Header {}
        Outlet::<Route> {}
        footer::Footer {}
    }
}

#[derive(Routable, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(KalosmHeaderFooter)]
        #[nest("/kalosm")]
            #[route("/")]
            KalosmHome {},
            #[layout(KalosmLearn)]
                #[child("/docs")]
                KalosmDocs { child: KalosmBookRoute },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[layout(HeaderFooter)]
        #[route("/")]
        Home {},
        #[layout(Learn)]
            #[child("/docs")]
            Docs { child: BookRoute },
        #[end_layout]
        #[layout(Blog)]
            #[child("/blog")]
            Blog { child: BlogRoute },
}

mod kalosm_docs {
    use dioxus::prelude::*;

    use_mdbook::mdbook_router! {"kalosm_doc_src"}
}

mod docs {
    use dioxus::prelude::*;

    use_mdbook::mdbook_router! {"doc_src"}
}

mod blog_route {
    use crate::structured_generation_visualized::{
        HtmlStructuredGenerationAcceleratedVisualization,
        StructuredGenerationAcceleratedVisualization, StructuredGenerationVisualization,
        TokenizationVisualization,
    };
    use dioxus::prelude::*;

    use_mdbook::mdbook_router! {"blog"}
}

fn main() {
    #[cfg(feature = "web")]
    {
        wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    #[cfg(feature = "prebuild")]
    {
        simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Warn)
            .init()
            .unwrap();

        std::env::remove_var("DIOXUS_ACTIVE");
        std::env::remove_var("CARGO");

        LaunchBuilder::new()
            .with_cfg(dioxus::static_site_generation::Config::new().github_pages())
            .launch(app);
        println!("prebuilt");

        dioxus_search::SearchIndex::<Route>::create(
            "search",
            dioxus_search::BaseDirectoryMapping::new(std::path::PathBuf::from("./docs")).map(
                |route: Route| {
                    let route = route.to_string();
                    if route.contains("kalosm") {
                        return None;
                    }
                    let mut path = std::path::PathBuf::new();
                    for segment in route.split('/') {
                        path.push(segment);
                    }
                    Some(path.join("index.html"))
                },
            ),
        );

        dioxus_search::SearchIndex::<Route>::create(
            "kalosm-search",
            dioxus_search::BaseDirectoryMapping::new(std::path::PathBuf::from("./docs")).map(
                |route: Route| {
                    let route = route.to_string();
                    if !route.contains("kalosm") {
                        return None;
                    }
                    let mut path = std::path::PathBuf::new();
                    for segment in route.split('/') {
                        path.push(segment);
                    }
                    Some(path.join("index.html"))
                },
            ),
        );
    }

    #[cfg(not(feature = "prebuild"))]
    launch(|| {
        rsx! {
            Router::<Route> {}
        }
    });
}

static SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "search"
};

static KALOSM_SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "kalosm-search"
};
