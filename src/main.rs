// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release
#![allow(non_snake_case)]
pub use crate::blog_route::BookRoute as BlogRoute;
use blog::Blog;
use dioxus::prelude::*;
use dioxus_fullstack::{prelude::*, router::FullstackRouterConfig};
use dioxus_router::prelude::*;
use home::Home;
use learn::Learn;
use serde::{Deserialize, Serialize};
use kalosm::KalosmHome;

mod blog;
mod home;
mod learn;
mod plugin;
mod search;
mod kalosm;

#[inline_props]
fn HeaderFooter(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SearchActive(false));

    render! {
        SearchModal {}
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

fn GithubLink(cx: Scope) -> Element {
    render! {
        a {
            class: "md:m-2 w-5 md:w-8 h-5 md:h-8",
            href: "https://github.com/floneum/floneum",
            img { src: "/assets/github-mark.png", alt: "Github Logo" }
        }
    }
}

fn DiscordLink(cx: Scope) -> Element {
    render! {
        a { margin: "10px", right: "10px", href: "https://discord.gg/dQdmhuB8q5",
            svg {
                class: "w-5 md:w-8 h-5 md:h-8",
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
    #[route("/kalosm")]
    KalosmHome {},
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

use crate::{
    docs::BookRoute,
    search::{Search, SearchActive, SearchModal},
};

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
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    #[cfg(feature = "prebuild")]
    {
        use dioxus_fullstack::prelude::*;
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                pre_cache_static_routes_with_props(
                    &ServeConfigBuilder::new_with_router(
                        dioxus_fullstack::router::FullstackRouterConfig::<Route>::default(),
                    )
                    .assets_path("docs")
                    .incremental(IncrementalRendererConfig::default().static_dir("docs"))
                    .build(),
                )
                .await
                .unwrap();
            });
        simple_logger::SimpleLogger::new()
            .with_level(log::LevelFilter::Warn)
            .init()
            .unwrap();

        dioxus_search::SearchIndex::<Route>::create(
            "search",
            dioxus_search::BaseDirectoryMapping::new(std::path::PathBuf::from("./docs")).map(
                |route: Route| {
                    let route = route.to_string();
                    let mut path = std::path::PathBuf::new();
                    for (i, segment) in route.split('/').enumerate() {
                        if (1, "docsite") == (i, segment) {
                            continue;
                        }
                        path.push(segment);
                    }
                    Some(path.join("index.html"))
                },
            ),
        );
    }

    #[cfg(not(feature = "prebuild"))]
    {
        #[allow(unused_mut)]
        let mut config = LaunchBuilder::<FullstackRouterConfig<Route>>::router();
    
        #[cfg(feature = "ssr")]
        {
            config = config.server_cfg(
                ServeConfigBuilder::new_with_router(Default::default())
                    .assets_path("docs")
                    .incremental(IncrementalRendererConfig::default()),
            );
        }
    
        config.launch();
    }
}

static SEARCH_INDEX: dioxus_search::LazySearchIndex<Route> = dioxus_search::load_search_index! {
    "search"
};
