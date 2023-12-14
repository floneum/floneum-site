use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_search::SearchResult;
use std::ops::Deref;

use crate::{Link, Route, SEARCH_INDEX};

pub struct SearchActive(pub bool);

impl Deref for SearchActive {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn SearchModal(cx: Scope) -> Element {
    let show_modal = use_shared_state::<SearchActive>(cx).unwrap();
    let search_text = use_state(cx, String::new);
    let results = SEARCH_INDEX.search(search_text.get());

    render! {
        if **show_modal.read() {
            rsx! {
                div {
                    height: "100vh",
                    width: "100vw",
                    class: "fixed top-0 left-0 z-50 hidden md:block backdrop-blur-lg bg-white/75 overflow-y-hidden",
                    onclick: move |_| {
                        *show_modal.write() = SearchActive(false)
                    },
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            *show_modal.write() = SearchActive(false)
                        }
                    },

                    // A little weird, but we're putting an empty div with a scaled height to buffer the top of the modal
                    div { class: "max-w-screen-md mx-auto h-full flex flex-col",
                        onclick: move |evt| evt.stop_propagation(),
                        div { class: "h-30" }

                        // The actual modal
                        div { class: "border border-gray-600 p-6 rounded-2xl m-8 max-h-[calc(100%-8rem)] overflow-y-auto",
                            // Search input
                            div { class: "flex flex-row flex-grow border-b border-gray-300 pb-4",
                                div { class: "my-auto flex flex-row",
                                    input {
                                        oninput: move |evt| {
                                            search_text.set(evt.value.clone());
                                        },
                                        onmounted: move |evt| {
                                            evt.inner().set_focus(true);
                                        },
                                        class: "flex-grow bg-transparent border-none outline-none text-xl pl-2",
                                        placeholder: "Search the docs",
                                        value: "{search_text}",
                                    }
                                }
                                div {}
                            }

                            // Results
                            div { class: "overflow-y-auto",
                                ul {
                                    match results {
                                        Ok(results) => {
                                            rsx! {
                                                for result in results {
                                                    SearchResult { result: result }
                                                }
                                            }
                                        }
                                        Err(err) => {
                                            rsx! {
                                                div { class: "text-red-500", "{err}" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SearchResult(cx: Scope, result: SearchResult<Route>) -> Element {
    let show_modal = use_shared_state::<SearchActive>(cx).unwrap();
    let title = &result.title;
    let top_excerpt_segments = &match result.excerpts.first() {
        Some(excerpt) => &*excerpt.text,
        None => &[],
    };

    render! {
        li { class: "w-full mt-4 p-2 rounded hover:bg-gray-100/50 transition-colors duration-200 ease-in-out",
            Link {
                to: result.route.clone(),
                onclick: move |_| {
                    *show_modal.write() = SearchActive(false);
                },
                div { class: "flex flex-col justify-between pb-1", h2 { class: "font-semibold", "{title}" } }
                p { class: "text-sm text-gray-500 pr-8",
                    for segment in top_excerpt_segments {
                        if segment.highlighted {
                            rsx! {
                                span { class: "text-blue-500", "{segment.text}" }
                            }
                        }
                        else {
                            rsx! {
                                span { "{segment.text}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn Search(cx: Scope) -> Element {
    let show_modal = use_shared_state::<SearchActive>(cx).unwrap();

    render! {
        // Pop up a modal
        button {
            // Pop up a modal
            class: "md:mx-3 rounded-lg md:px-3 md:py-3 text-left md:pr-[50vw] my-auto flex flex-row align-middle justify-between",
            onclick: move |_| {
                *show_modal.write() = SearchActive(true);
                log::info!("Search modal opened");
            },
            div { class: "h-full my-auto flex flex-row align-middle justify-between",
                span { class: "md:pl-2", "Search" }
            }
        }
    }
}
