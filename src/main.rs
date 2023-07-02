// Build with:
// dioxus build --release --features web
// cargo run --features ssr --release
#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use learn::{Learn, LearnProps};
use serde::{Deserialize, Serialize};

mod learn;
mod search;

#[inline_props]
fn HeaderFooter(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SearchActive(false));
    let should_blur = use_shared_state::<SearchActive>(cx).unwrap();
    let content_class = if should_blur.read().0 { "blur-md" } else { "" };

    render! {
        SearchModal {}
        div {
            class: "{content_class}",
            div {
                class: "z-30 flex flex-row justify-between items-center border-b-2 border-gray-700 w-full",
                Link {
                    class: "text-xl font-bold m-2 mr-12",
                    target: Route::Home {},
                    "Floneum"
                }
                Search {}
                div {
                    class: "flex flex-row items-center",
                    Link {
                        class: "text-xl font-bold m-2 mr-12",
                        target: Route::Docs {
                            child: BookRoute::Index {}
                        },
                        "Documentation"
                    }
                    GithubLink{}
                    DiscordLink{}
                }
            }
            Outlet {}
            div {
                class: "bg-gray-700 px-80 py-5 flex flex-row items-center justify-evenly",
                div {
                    class: "flex flex-col items-center",
                    "Community"
    
                    DiscordLink {}
                }
                div {
                    class: "flex flex-col items-center",
                    "About"
    
                    GithubLink {}
                }
            }
        }
    }
}

fn GithubLink(cx: Scope) -> Element {
    render! {
        a {
            class: "m-2 w-6 h-6",
            href: "https://github.com/floneum/floneum",
            img {
                src: "/assets/GitHub-Mark-Light-32px.png",
                alt: "Github",
            }
        }
    }
}

fn DiscordLink(cx: Scope) -> Element {
    render! {
        a { margin: "10px", right: "10px", href: "https://discord.gg/dQdmhuB8q5",
            svg { width: "32", height: "32", view_box: "0 -28.5 256 256", preserve_aspect_ratio: "xMidYMid",
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

#[inline_props]
fn Home(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col items-center",
            h1 {
                class: "font-bold text-9xl m-2 ml-12 animate-shine bg-clip-text text-transparent",
                "Floneum"
            }
            p {
                "A graph editor for local AI workflows"
            }
        }
        div {
            class: "w-full flex flex-col mt-12",
            div {
                class: "w-full flex flex-row justify-between items-center",
                div {
                    class: "animate-fade-in-left m-4",
                    h2 {
                        class: "text-4xl font-bold mb-2",
                        "Build AI powered workflows with ease"
                    }
                    p {
                        "Floneum is a workflow engine that allows you to build AI powered workflows visually"
                    }
                }
                div {
                    class: "animate-fade-in-right m-4",
                    img {
                        class: "max-w-2xl",
                        src: "./assets/demo-img.png",
                    }
                }
            }
            div {
                class: "w-full flex flex-row justify-between items-center",
                div {
                    class: "animate-fade-in-left-slow m-4",
                    img {
                        class: "max-w-2xl",
                        src: "./assets/plugins.png",
                    }
                }
                div {
                    class: "animate-fade-in-right-slow m-4",
                    h2 {
                        class: "text-4xl font-bold mb-2",
                        "Securely extend Floneum with plugins"
                    }
                    p {
                        "Floneum uses WebAssembly to load plugins in a sandboxed environment and provides them with access to only the resources they need instead of giving them full access to the system"
                    }
                }
            }
            div {
                class: "w-full flex flex-row justify-between items-center",
                div {
                    class: "animate-fade-in-left-slower m-4",
                    h2 {
                        class: "text-4xl font-bold mb-2",
                        "Write plugins in your language of choice"
                    }
                    p {
                        "You can write plugins in any language that can be compiled to WebAssembly. Floneum provides ergonomic wrappers for rust, but you can also use C, Java, or Go"
                    }
                }
                div {
                    class: "animate-fade-in-right-slower m-4",
                    div {
                        class: "grid grid-cols-2 gap-4 justify-items-center items-center max-w-2xl",
                        img {
                            src: "./assets/rust_logo.svg",
                        }
                        img {
                            src: "./assets/c_logo.png",
                        }
                        img {
                            src: "./assets/java_logo.png",
                        }
                        img {
                            src: "./assets/go_logo.png",
                        }
                    }
                }
            }
        }

        div {
            class: "flex flex-row items-center justify-evenly w-full",

            Link {
                class: "text-5xl font-bold p-4 m-12 rounded-md bg-orange-900 text-center w-1/3",
                target: Route::Docs {
                    child: BookRoute::UserIndex {}
                },
                "User Documentation"
            }

            Link {
                class: "text-5xl font-bold p-4 m-12 rounded-md bg-blue-900 text-center w-1/3",
                target: Route::Docs {
                    child: BookRoute::DeveloperIndex {}
                },
                "Developer Documentation"
            }
        }
    }
}

#[derive(Routable, Clone, Serialize, Deserialize)]
#[rustfmt::skip]
enum Route {
    #[layout(HeaderFooter)]
        #[route("/")]
        Home {},
        #[layout(Learn)]
            #[child("/docs")]
            Docs { child: BookRoute },
}

use crate::{docs::BookRoute, search::{SearchActive, SearchModal, Search}};

mod docs {
    use dioxus::prelude::*;

    use_mdbook::mdbook_router! {"doc_src"}
}

fn main() {
    #[cfg(feature = "web")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));
    #[cfg(feature = "ssr")]
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
    }

    dioxus_fullstack::launch_router!(@([127, 0, 0, 1], 8080), Route, {
        serve_cfg: {
            dioxus_fullstack::prelude::ServeConfigBuilder::new_with_router(
                dioxus_fullstack::router::FullstackRouterConfig::<Route>::default(),
            )
            .assets_path("docs")
            .incremental(IncrementalRendererConfig::default())
        },
    });
    log::info!("started");
}