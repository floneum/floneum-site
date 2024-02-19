use crate::search::{SearchActive, SearchResult};
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};

use crate::KALOSM_SEARCH_INDEX;

pub fn KalosmSearchModal(cx: Scope) -> Element {
    let show_modal = use_shared_state::<SearchActive>(cx).unwrap();
    let search_text = use_state(cx, String::new);
    let results = KALOSM_SEARCH_INDEX.search(search_text.get());

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
                                        onmounted: move |evt| async move {
                                            let _ = evt.inner().set_focus(true).await;
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
