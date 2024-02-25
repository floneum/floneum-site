// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release
#![allow(non_snake_case)]
pub use crate::blog_route::BookRoute as BlogRoute;
use crate::kalosm::learn::KalosmLearn;
use crate::{
    docs::BookRoute,
    kalosm_docs::BookRoute as KalosmBookRoute,
    search::{Search, SearchModal},
};
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
mod home;
mod kalosm;
mod learn;
mod plugin;
mod search;
mod shortcut;

#[component]
fn HeaderFooter() -> Element {
    rsx! {
        SearchModal { index: &SEARCH_INDEX }
        div { class: "h-14 z-30 fixed top-0 flex flex-row justify-between items-center bg-gray-100 opacity-50 border-b-2 border-gray-700 w-screen backdrop-blur-lg",
            Link { class: "text-xl font-bold m-2 md:mr-12", to: Route::Home {}, "Floneum" }
            Search {}
            div { class: "flex flex-row justify-evenly items-center text-center",
                Link {
                    class: "text-sm md:text-xl font-bold pr-1 md:p-2",
                    to: Route::Blog {
                        child: BlogRoute::Index {},
                    },
                    "Blog"
                }
                Link {
                    class: "text-sm md:text-xl font-bold md:p-2",
                    to: Route::Docs {
                        child: BookRoute::Index {},
                    },
                    "Documentation"
                }
                GithubLink {}
                DiscordLink {}
            }
        }
        div { class: "pt-14",
            Outlet::<Route> {}
            div { class: "py-5 flex flex-row items-center justify-evenly",
                div { class: "flex flex-col items-center",
                    "Discord Community"
                    DiscordLink {}
                }
                div { class: "flex flex-col items-center",
                    "Source Code"
                    GithubLink {}
                }
            }
        }
    }
}

fn GithubLink() -> Element {
    rsx! {
        a { href: "https://github.com/floneum/floneum",
            span { class: "sr-only", "Github" }
            svg { class: "md:m-2 w-5 md:w-8 h-5 md:h-8", view_box: "0 0 24 24",
                path {
                    d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z",
                    fill: "currentColor",
                    fill_rule: "nonzero"
                }
            }
        }
    }
}

fn DiscordLink() -> Element {
    rsx! {
        a {
            margin: "10px",
            right: "10px",
            href: "https://discord.gg/dQdmhuB8q5",
            span { class: "sr-only", "Discord" }
            svg {
                class: "md:m-2 w-5 md:w-8 h-5 md:h-8",
                view_box: "0 -28.5 256 256",
                preserve_aspect_ratio: "xMidYMid",
                g {
                    path {
                        d: "M216.856339,16.5966031 C200.285002,8.84328665 182.566144,3.2084988 164.041564,0 C161.766523,4.11318106 159.108624,9.64549908 157.276099,14.0464379 C137.583995,11.0849896 118.072967,11.0849896 98.7430163,14.0464379 C96.9108417,9.64549908 94.1925838,4.11318106 91.8971895,0 C73.3526068,3.2084988 55.6133949,8.86399117 39.0420583,16.6376612 C5.61752293,67.146514 -3.4433191,116.400813 1.08711069,164.955721 C23.2560196,181.510915 44.7403634,191.567697 65.8621325,198.148576 C71.0772151,190.971126 75.7283628,183.341335 79.7352139,175.300261 C72.104019,172.400575 64.7949724,168.822202 57.8887866,164.667963 C59.7209612,163.310589 61.5131304,161.891452 63.2445898,160.431257 C105.36741,180.133187 151.134928,180.133187 192.754523,160.431257 C194.506336,161.891452 196.298154,163.310589 198.110326,164.667963 C191.183787,168.842556 183.854737,172.420929 176.223542,175.320965 C180.230393,183.341335 184.861538,190.991831 190.096624,198.16893 C211.238746,191.588051 232.743023,181.531619 254.911949,164.955721 C260.227747,108.668201 245.831087,59.8662432 216.856339,16.5966031 Z M85.4738752,135.09489 C72.8290281,135.09489 62.4592217,123.290155 62.4592217,108.914901 C62.4592217,94.5396472 72.607595,82.7145587 85.4738752,82.7145587 C98.3405064,82.7145587 108.709962,94.5189427 108.488529,108.914901 C108.508531,123.290155 98.3405064,135.09489 85.4738752,135.09489 Z M170.525237,135.09489 C157.88039,135.09489 147.510584,123.290155 147.510584,108.914901 C147.510584,94.5396472 157.658606,82.7145587 170.525237,82.7145587 C183.391518,82.7145587 193.761324,94.5189427 193.539891,108.914901 C193.539891,123.290155 183.391518,135.09489 170.525237,135.09489 Z",
                        fill: "currentColor",
                        fill_rule: "nonzero"
                    }
                }
            }
        }
    }
}

#[derive(Routable, Clone, Serialize, Deserialize, PartialEq)]
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

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let index_html = std::fs::read_to_string("docs/index.html").unwrap();
                let main_tag = r#"<div id="main" class="w-full h-full">"#;
                let (before_body, after_body) =
                    index_html.split_once(main_tag).expect("main id not found");
                let after_body = after_body
                    .split_once("</div>")
                    .expect("main id not found")
                    .1;
                let wrapper = DefaultRenderer {
                    before_body: before_body.to_string() + main_tag,
                    after_body: "</div>".to_string() + after_body,
                };
                let mut renderer = IncrementalRenderer::builder()
                    .static_dir("docs_static")
                    .map_path(|route| {
                        let mut path = std::env::current_dir().unwrap();
                        path.push("docs_static");
                        for segment in route.split('/') {
                            path.push(segment);
                        }
                        println!("build: {path:?}");
                        path
                    })
                    .build();
                renderer.renderer_mut().pre_render = true;
                pre_cache_static_routes::<Route, _>(&mut renderer, &wrapper)
                    .await
                    .unwrap();

                // Copy everything from docs_static to docs
                let mut options = fs_extra::dir::CopyOptions::new();
                options.overwrite = true;
                options.content_only = true;
                options.copy_inside = true;
                std::fs::create_dir_all("./docs").unwrap();
                fs_extra::dir::move_dir("./docs_static", "./docs", &options).unwrap();
            });
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
    launch(|| rsx! { Router::<Route> {} });
}

static SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "search"
};

static KALOSM_SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "kalosm-search"
};
