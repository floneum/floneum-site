use crate::{KalosmBookRoute, Route};
use dioxus::prelude::*;

#[component]
pub fn CallToAction() -> Element {
    rsx! {
        div { class: "bg-white",
            div { class: "px-6 py-24 sm:px-6 sm:py-32 lg:px-8",
                div { class: "mx-auto max-w-2xl text-center",
                    h2 { class: "text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl",
                        "Ready to dive in?"
                        br {}
                        "Get started with Kalosm today."
                    }
                    div { class: "mt-10 flex items-center justify-center gap-x-6",
                        Link {
                            to: Route::KalosmDocs {
                                child: KalosmBookRoute::Index {},
                            },
                            class: "rounded-md bg-indigo-600 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600",
                            "Docs"
                        }
                        Link {
                            to: "https://github.com/floneum/floneum/tree/master/interfaces/kalosm",
                            class: "text-sm font-semibold leading-6 text-gray-900",
                            "View on GitHub"
                            span { "aria-hidden": "true", "→" }
                        }
                    }
                }
            }
        }
    }
}
