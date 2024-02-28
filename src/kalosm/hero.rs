use super::data_roller::DataAnimation;
use crate::{BlogRoute, KalosmBookRoute, Route};
use dioxus::prelude::*;

use syntect_html::syntect_html_fs;

pub fn Hero() -> Element {
    rsx! {
        div { class: "bg-white",
            div { class: "relative isolate overflow-hidden bg-gradient-to-b from-indigo-100/20",
                div { class: "mx-auto max-w-7xl pb-24 pt-10 sm:pb-32 lg:grid lg:grid-cols-2 lg:gap-x-8 lg:px-8 lg:py-40",
                    div { class: "px-6 lg:px-0 lg:pt-4",
                        div { class: "mx-auto max-w-2xl",
                            div { class: "max-w-lg",
                                div { class: "mt-24 sm:mt-32 lg:mt-16",
                                    Link {
                                        to: Route::Blog {
                                            child: BlogRoute::Kalosm02 {},
                                        },
                                        class: "inline-flex space-x-6",
                                        span { class: "rounded-full bg-indigo-600/10 px-3 py-1 text-sm font-semibold leading-6 text-indigo-600 ring-1 ring-inset ring-indigo-600/10",
                                            "What's new"
                                        }
                                        span { class: "inline-flex items-center space-x-2 text-sm font-medium leading-6 text-gray-600",
                                            span { "Just shipped v0.2.0" }
                                            svg {
                                                "fill": "currentColor",
                                                "arifa-hidden": "true",
                                                "viewBox": "0 0 20 20",
                                                class: "h-5 w-5 text-gray-400",
                                                path {
                                                    "clip-rule": "evenodd",
                                                    "fill-rule": "evenodd",
                                                    "d": "M7.21 14.77a.75.75 0 01.02-1.06L11.168 10 7.23 6.29a.75.75 0 111.04-1.08l4.5 4.25a.75.75 0 010 1.08l-4.5 4.25a.75.75 0 01-1.06-.02z"
                                                }
                                            }
                                        }
                                    }
                                }
                                h1 { class: "mt-10 text-4xl font-bold tracking-tight text-gray-900 sm:text-6xl",
                                    "AI you can trust with your "
                                    DataAnimation {}
                                }
                                p { class: "mt-14 text-lg leading-8 text-gray-600",
                                    "Kalosm makes it easy to interact with private language, audio, and image models in Rust"
                                }
                                div { class: "mt-10 flex items-center gap-x-6",
                                    Link {
                                        to: Route::KalosmDocs {
                                            child: KalosmBookRoute::Index {},
                                        },
                                        class: "rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                                        "Documentation"
                                    }
                                    Link {
                                        to: "https://discord.gg/dQdmhuB8q5",
                                        class: "text-sm font-semibold leading-6 text-gray-900",
                                        "Join the community"
                                        span { "aria-hidden": "true", "→" }
                                    }
                                    Link {
                                        to: "https://github.com/floneum/floneum/tree/master/interfaces/kalosm",
                                        class: "text-sm font-semibold leading-6 text-gray-900",
                                        "View on GitHub "
                                        span { "aria-hidden": "true", "→" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "mt-20 sm:mt-24 md:mx-auto md:max-w-2xl lg:mx-0 lg:mt-0 lg:w-screen",
                        div {
                            "aria-hidden": "true",
                            class: "absolute inset-y-0 right-1/2 -z-10 -mr-10 w-[200%] skew-x-[-30deg] bg-white shadow-xl shadow-indigo-600/10 ring-1 ring-indigo-50 md:-mr-20 lg:-mr-36"
                        }
                        div { class: "shadow-lg md:rounded-3xl",
                            div { class: "bg-indigo-500 [clip-path:inset(0)] md:[clip-path:inset(0_round_theme(borderRadius.3xl))]",
                                div {
                                    "aria-hidden": "true",
                                    class: "absolute -inset-y-px left-1/2 -z-10 ml-10 w-[200%] skew-x-[-30deg] bg-indigo-100 opacity-20 ring-1 ring-inset ring-white md:ml-20 lg:ml-36"
                                }
                                div { class: "relative px-6 pt-8 sm:pt-16 md:pl-16 md:pr-0",
                                    div { class: "mx-auto max-w-2xl md:mx-0 md:max-w-none",
                                        div { class: "w-screen overflow-hidden rounded-tl-xl bg-gray-900",
                                            div { class: "flex bg-gray-800/40 ring-1 ring-white/5",
                                                div { class: "-mb-px flex text-sm font-medium leading-6 text-gray-400",
                                                    div { class: "border-b border-r border-b-white/20 border-r-white/10 bg-white/5 px-4 py-2 text-white",
                                                        "microphone_rag.rs"
                                                    }
                                                    div { class: "border-r border-gray-600/10 px-4 py-2",
                                                        "app.rs"
                                                    }
                                                }
                                            }
                                            div { class: "px-6 pt-6",
                                                div {
                                                    class: "w-full h-full min-h-0 pl-4 pt-4 pb-14 rounded-tl-xl",
                                                    background_color: "#2b303b",
                                                    dangerous_inner_html: syntect_html_fs!("./src/kalosm/hero_code_snippet.rs")
                                                }
                                            }
                                        }
                                    }
                                    div {
                                        "aria-hidden": "true",
                                        class: "pointer-events-none absolute inset-0 ring-1 ring-inset ring-black/10 md:rounded-3xl"
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "absolute inset-x-0 bottom-0 -z-10 h-24 bg-gradient-to-t from-white sm:h-32" }
            }
        }
    }
}
