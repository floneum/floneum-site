use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{docs::BookRoute, plugin::PluginsList, Route};

#[component]
pub(crate) fn Home(cx: Scope) -> Element {
    render! {
        Pitch {}
        div { height: "125vh" }
        Demo {}
        Plugins {}
        CallToAction {}
    }
}

fn Demo(cx: Scope) -> Element {
    render! {
        div { class: "flex flex-col items-center justify-center text-center pb-12",
            div { class: "w-3/4",
                h2 { class: "text-2xl md:text-6xl font-bold rounded-md",
                    "Build AI powered workflows with ease"
                }
                div { class: "m-4", img {
                    src: "/assets/demo-img.png",
                    alt: "Demo question answering workflow"
                } }
                p { class: "text-xl md:text-4xl",
                    "Floneum allows you to build workflows that use large language models with a simple drag and drop interface"
                }
            }
        }
    }
}

fn Plugins(cx: Scope) -> Element {
    render! {
        div { class: "w-screen h-[75vw] flex flex-col justify-center items-center static",
            h2 { class: "bg-[#B95F62] text-2xl md:text-6xl font-bold text-center left-[calc(200vw-var(--scroll)*200vw)] w-screen rounded-t-md p-2",
                "Securely extend Floneum with plugins"
            }
            div { class: "-z-30 grid grid-cols-2 grid-rows-2 w-screen",
                div { class: "bg-[#AACDCF] w-[50vw] h-[25vw] flex flex-col justify-center items-center  overflow-clip",
                    img {
                        class: "w-60",
                        src: "/assets/plugins.png",
                        alt: "Plugins can access LLMs, Embeddings, and Embedding Databases"
                    }
                }
                div { class: "bg-[#283549] w-[50vw] h-[25vw] flex flex-col justify-center items-center overflow-clip lg:text-xl text-white p-2",
                    p { class: "text-xl md:text-2xl p-2",
                        "Floneum uses WebAssembly to load plugins in a sandboxed environment and provides them with access to only the resources they need instead of giving them full access to the system"
                    }
                }
                div { class: "bg-[#283549] w-[50vw] h-[25vw] flex flex-col justify-center items-center overflow-clip lg:text-xl text-white rounded-bl-md p-2",
                    p { class: "text-xl md:text-2xl p-2",
                        "You can write plugins in any language that can be compiled to WebAssembly. Floneum provides ergonomic wrappers for rust, but you can also use C, Java, or Go"
                    }
                }
                div { class: "bg-[#AACDCF] w-[50vw] h-[25vw] flex flex-col justify-center items-center  overflow-clip text-white rounded-br-md",
                    div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 justify-items-center items-center",
                        img {
                            class: "w-28",
                            src: "/assets/rust_logo.svg",
                            alt: "Rust Logo"
                        }
                        img { class: "w-28", src: "/assets/c_logo.png", alt: "C Logo" }
                        img {
                            class: "w-28",
                            src: "/assets/java_logo.png",
                            alt: "Java Logo"
                        }
                        img { class: "w-28", src: "/assets/go_logo.png", alt: "Go Logo" }
                    }
                }
            }
        }
        PluginsList {}
    }
}

fn CallToAction(cx: Scope) -> Element {
    render! {
        div { class: "flex flex-row items-center justify-evenly w-full",
            Link {
                class: "text-xl md:text-5xl font-bold p-4 m-12 text-center w-1/3 rounded-lg bg-[#283549] hover:border-2 border-[#283549] text-white",
                to: "https://github.com/floneum/floneum/releases/tag/v0.2.0",
                "Try Floneum Today"
            }
        }

        div { class: "flex flex-row items-center justify-evenly w-full",
            Link {
                class: "text-xl md:text-5xl font-bold p-4 m-12 text-center w-1/2 md:w-1/3 rounded-lg bg-[#AACDCF] hover:border-2 border-[#283549]",
                to: Route::Docs {
                    child: BookRoute::UserIndex {},
                },
                "Guide"
            }

            Link {
                class: "text-xl md:text-5xl font-bold p-4 m-12 text-center w-1/2 md:w-1/3 rounded-lg bg-[#AACDCF] hover:border-2 border-[#283549]",
                to: Route::Docs {
                    child: BookRoute::DeveloperIndex {},
                },
                "Build Your First Plugin"
            }
        }
    }
}

fn Pitch(cx: Scope) -> Element {
    render! {
        div { class: "-z-30 fixed grid grid-cols-2 grid-rows-2 gap-[calc(var(--scroll)*(200vw*sin(45deg)+200vh*cos(45deg)))] rotate-[calc(45deg+var(--scroll)*90deg)] w-[calc(100vw*sin(45deg)+100vh*cos(45deg))] h-[calc(100vw*sin(45deg)+100vh*cos(45deg))] top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
            div { class: "bg-[#B95F62] w-full h-full flex flex-col justify-end items-end text-white overflow-clip",
                h2 { class: "text-xlg sm:text-4xl md:text-5xl lg:text-6xl font-bold p-8 w-1/2 text-right",
                    "Your Multi Tool for Language Tasks"
                }
            }
            div { class: "bg-[#283549] w-full h-full flex flex-col justify-start items-start -rotate-90 text-white overflow-clip",
                h2 { class: "text-xlg sm:text-4xl md:text-5xl lg:text-6xl font-bold p-8 w-1/2 text-left",
                    "Build AI powered workflows visually"
                }
            }
            div { class: "bg-[#EFFBF8] w-full h-full flex flex-col justify-end items-end -rotate-90 overflow-clip",
                h2 { class: "text-xlg sm:text-4xl md:text-5xl lg:text-6xl font-bold p-8 w-1/2 text-right",
                    "Securely extend Floneum with isolated WASM plugins"
                }
            }
            div { class: "bg-[#AACDCF] w-full h-full flex flex-col justify-start items-start overflow-clip",
                h2 { class: "text-xlg sm:text-4xl md:text-5xl lg:text-6xl font-bold p-8 w-1/2 text-left",
                    "Write plugins in your language of choice"
                }
            }
        }
    }
}
