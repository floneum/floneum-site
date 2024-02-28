use crate::shortcut;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_search::SearchResult;

use crate::{Link, Route};

static SEARCH_ACTIVE: GlobalSignal<bool> = Signal::global(|| false);
static SEARCH_TEXT: GlobalSignal<String> = Signal::global(String::new);

#[derive(Clone, Props)]
pub struct SearchModalProps {
    index: &'static dioxus_search::LazySearchIndex<Route>,
}

impl PartialEq for SearchModalProps {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.index, other.index)
    }
}

pub fn SearchModal(props: SearchModalProps) -> Element {
    let index = props.index;
    let mut results = use_signal(move || {
        log::info!("searching for {}", &*SEARCH_TEXT.read());
        index.search(&SEARCH_TEXT.read())
    });

    let mut last_key_press = use_signal(|| {
        #[cfg(not(target_arch = "wasm32"))]
        return 0.;
        js_sys::Date::now()
    });
    use_future(move || {
        async move {
            // debounce the search
            if *last_key_press.read() - js_sys::Date::now() > 100. {
                log::info!("searching for {}", &*SEARCH_TEXT.read());
                results.set(index.search(&*SEARCH_TEXT.read()));
                last_key_press.set(js_sys::Date::now());
            } else {
                gloo_timers::future::TimeoutFuture::new(100).await;
                log::info!("searching for {}", &*SEARCH_TEXT.read());
                results.set(index.search(&*SEARCH_TEXT.read()));
            }
        }
    });

    rsx! {
        if SEARCH_ACTIVE() {
            div {
                role: "dialog",
                "aria-modal": "true",
                class: "relative z-10",
                div { class: "fixed inset-0 bg-gray-500 bg-opacity-25 transition-opacity" }
                div {
                    class: "fixed inset-0 z-10 w-screen overflow-y-auto p-4 sm:p-6 md:p-20",
                    onclick: move |_| { *SEARCH_ACTIVE.write() = false },
                    onkeydown: move |evt| {
                        if evt.key() == Key::Escape {
                            *SEARCH_ACTIVE.write() = false;
                        }
                    },
                    div {
                        class: "mx-auto max-w-2xl transform divide-y divide-gray-500 divide-opacity-10 overflow-hidden rounded-xl bg-white bg-opacity-80 shadow-2xl ring-1 ring-black ring-opacity-5 backdrop-blur backdrop-filter transition-all",
                        onclick: move |evt| evt.stop_propagation(),
                        div { class: "relative",
                            svg {
                                "fill": "currentColor",
                                "viewBox": "0 0 20 20",
                                "aria-hidden": "true",
                                class: "pointer-events-none absolute left-4 top-3.5 h-5 w-5 text-gray-900 text-opacity-40",
                                path {
                                    "clip-rule": "evenodd",
                                    "fill-rule": "evenodd",
                                    "d": "M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z"
                                }
                            }
                            input {
                                r#type: "text",
                                class: "h-12 w-full border-0 bg-transparent pl-11 pr-4 text-gray-900 focus:ring-0 sm:text-sm",
                                oninput: move |evt| {
                                    *SEARCH_TEXT.write() = evt.value();
                                },
                                onmounted: move |evt| async move {
                                    let _ = evt.set_focus(true).await;
                                },
                                placeholder: "Search the docs",
                                value: "{SEARCH_TEXT}"
                            }
                        }
                        match &*results.read() {
                            Ok(results) => {
                                rsx! {
                                    if SEARCH_TEXT.read().is_empty() {
                                        SearchHints {}
                                    } else if results.is_empty() {
                                        SearchResultsEmpty {}
                                    } else {
                                        ul { class: "max-h-96 overflow-y-auto p-2 text-sm text-gray-700",
                                            for result in results {
                                                SearchResult { result: result.clone() }
                                            }
                                        }
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

#[component]
pub fn SearchResult(result: SearchResult<Route>) -> Element {
    let title = &result.title;
    let top_excerpt_segments = &match result.excerpts.first() {
        Some(excerpt) => &*excerpt.text,
        None => &[],
    };

    rsx! {
        li { class: "group flex cursor-default select-none items-center rounded-md px-3 py-2",
            Link {
                to: result.route.clone(),
                onclick: move |_| {
                    *SEARCH_ACTIVE.write() = false;
                },
                svg {
                    "stroke": "currentColor",
                    "aria-hidden": "true",
                    "viewBox": "0 0 24 24",
                    "fill": "none",
                    "stroke-width": "1.5",
                    class: "h-6 w-6 flex-none text-gray-900 text-opacity-40",
                    path {
                        "stroke-linejoin": "round",
                        "stroke-linecap": "round",
                        "d": "M12 10.5v6m3-3H9m4.06-7.19l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z"
                    }
                }
                span { class: "ml-3 flex-auto truncate", "{title}" }
                span { class: "ml-3 flex-none text-xs truncate text-gray-500",
                    for segment in top_excerpt_segments {
                        if segment.highlighted {
                            span { class: "text-blue-500", "{segment.text}" }
                        } else {
                            span { "{segment.text}" }
                        }
                    }
                }
            }
        }
    }
}

pub fn Search() -> Element {
    shortcut::use_shortcut(Key::Character("/".to_string()), Modifiers::CONTROL, {
        move || {
            *SEARCH_ACTIVE.write() = true;
        }
    });

    rsx! {
        label { r#for: "search", class: "sr-only", "Search" }
        div { class: "relative",
            div { class: "pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3",
                svg {
                    "viewBox": "0 0 20 20",
                    "fill": "currentColor",
                    "aria-hidden": "true",
                    class: "h-5 w-5 text-gray-400",
                    path {
                        "fill-rule": "evenodd",
                        "clip-rule": "evenodd",
                        "d": "M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z"
                    }
                }
            }
            button {
                name: "search",
                class: "block w-full rounded-md border-0 bg-white py-1.5 pl-10 pr-3 text-gray-400 ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6",
                id: "search",
                onclick: move |_| {
                    *SEARCH_ACTIVE.write() = true;
                    log::info!("Search modal opened");
                },
                div { class: "h-full my-auto flex flex-row align-middle justify-between",
                    span { class: "md:pl-2", "Search" }
                    div { class: "ml-3 flex-none font-semibold text-gray-500",
                        kbd { class: "font-sans", "⌘" }
                        kbd { class: "font-sans", "/" }
                    }
                }
            }
        }
    }
}

fn SearchHints() -> Element {
    let example_searches = ["Resource augmented generation", "Transcription", "Chatbot"];

    rsx! {
        h2 {
            // centered text that says "Try searching for..."
            class: "flex items-center justify-center text-gray-900 text-opacity-40 text-sm font-semibold",
            "Try searching for..."
        }
        ul { class: "max-h-80 scroll-py-2 divide-y divide-gray-500 divide-opacity-10 overflow-y-auto",
            li { class: "p-2",
                ul { class: "text-sm text-gray-700",
                    for example in example_searches {
                        li { class: "group flex cursor-default select-none items-center rounded-md px-3 py-2",
                            button { onclick: move |_| {
                                    *SEARCH_TEXT.write() = example.to_string();
                                },
                                span { class: "ml-3 flex-auto truncate", "{example}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn SearchResultsEmpty() -> Element {
    rsx! {
        div { class: "px-6 py-14 text-center sm:px-14",
            svg {
                "stroke-width": "1.5",
                "aria-hidden": "true",
                "fill": "none",
                "viewBox": "0 0 24 24",
                "stroke": "currentColor",
                class: "mx-auto h-6 w-6 text-gray-900 text-opacity-40",
                path {
                    "stroke-linecap": "round",
                    "d": "M2.25 12.75V12A2.25 2.25 0 014.5 9.75h15A2.25 2.25 0 0121.75 12v.75m-8.69-6.44l-2.12-2.12a1.5 1.5 0 00-1.061-.44H4.5A2.25 2.25 0 002.25 6v12a2.25 2.25 0 002.25 2.25h15A2.25 2.25 0 0021.75 18V9a2.25 2.25 0 00-2.25-2.25h-5.379a1.5 1.5 0 01-1.06-.44z",
                    "stroke-linejoin": "round"
                }
            }
            p { class: "mt-4 text-sm text-gray-900",
                "We couldn't find any pages with that term. Please try again."
            }
        }
    }
}
