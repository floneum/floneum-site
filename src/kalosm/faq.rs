use dioxus::prelude::*;

#[component]
fn Question(question: String, index: usize, children: Element) -> Element {
    let mut visible = use_signal(|| false);

    rsx! {
        div { class: "pt-6",
            dt {
                button {
                    onclick: move |_| visible.toggle(),
                    "aria-controls": "faq-{index}",
                    r#type: "button",
                    "aria-expanded": "false",
                    class: "flex w-full items-start justify-between text-left text-gray-900",
                    span { class: "text-base font-semibold leading-7", {question} }
                    span { class: "ml-6 flex h-7 items-center",
                        svg {
                            "fill": "none",
                            "stroke-width": "1.5",
                            "viewBox": "0 0 24 24",
                            "stroke": "currentColor",
                            "aria-hidden": "true",
                            class: if visible() { "h-6 w-6 hidden" } else { "h-6 w-6" },
                            path {
                                "stroke-linejoin": "round",
                                "d": "M12 6v12m6-6H6",
                                "stroke-linecap": "round"
                            }
                        }
                        svg {
                            "stroke-width": "1.5",
                            "viewBox": "0 0 24 24",
                            "stroke": "currentColor",
                            "fill": "none",
                            "aria-hidden": "true",
                            class: if visible() { "h-6 w-6 hidden" } else { "h-6 w-6" },
                            path {
                                "d": "M18 12H6",
                                "stroke-linejoin": "round",
                                "stroke-linecap": "round"
                            }
                        }
                    }
                }
            }
            dd { class: "mt-2 pr-12", id: "faq-{index}",
                p {
                    class: if visible() { "text-base leading-7 text-gray-600" } else { "text-base leading-7 text-gray-600 hidden" },
                    {children}
                }
            }
        }
    }
}

pub fn Faq() -> Element {
    rsx! {
        div { class: "bg-white",
            div { class: "mx-auto max-w-7xl px-6 py-24 sm:py-32 lg:px-8 lg:py-40",
                div { class: "mx-auto max-w-4xl divide-y divide-gray-900/10",
                    h2 { class: "text-2xl font-bold leading-10 tracking-tight text-gray-900",
                        "Frequently asked questions"
                    }
                    dl { class: "mt-10 space-y-6 divide-y divide-gray-900/10",
                        Question {
                            question: "What can you build with Kalosm?",
                            index: 0,
                            "You can think of Kalosm as the plumbing between different pre-trained models and the surrounding world. Kalosm makes it easy to build applications that use pre-trained models to generate text, audio, and images. You can build anything from "
                            Link {
                                to: "https://github.com/floneum/floneum/blob/main/interfaces/kalosm/examples/remote.rs",
                                class: "text-indigo-600",
                                "chatbots"
                            }
                            " and "
                            Link {
                                to: "https://github.com/floneum/floneum/tree/main/interfaces/kalosm#voice-transcription",
                                class: "text-indigo-600",
                                "note-taking apps"
                            }
                            " to "
                            Link {
                                to: "https://github.com/floneum/floneum/tree/main/interfaces/kalosm#image-generation",
                                class: "text-indigo-600",
                                "artistic tools"
                            }
                            " and "
                            Link {
                                to: "https://github.com/floneum/floneum/tree/main/interfaces/kalosm#embedding-powered-search",
                                class: "text-indigo-600",
                                "search engines"
                            }
                        }
                        Question {
                            question: "Do you need a GPU to use Kalosm models?",
                            index: 1,
                            "No! Kalosm uses quantization and other techniques to make it possible to run models on a CPU. Kalosm uses "
                            Link {
                                to: "https://github.com/huggingface/candle",
                                class: "text-indigo-600",
                                "Candle"
                            }
                            " which supports running quantized models on a CPU with upcoming support for Metal and CUDA."
                        }
                        Question {
                            question: "Can Kalosm be used with remote models like GPT-3?",
                            index: 2,
                            "Yes! Kalosm is local-first, but it can be "
                            Link {
                                to: "https://github.com/floneum/floneum/blob/main/interfaces/kalosm/examples/remote.rs",
                                class: "text-indigo-600",
                                "used with remote models"
                            }
                            " like GPT-3. You can also split up work between local and remote models or use "
                            Link {
                                to: "https://github.com/floneum/floneum/blob/main/interfaces/kalosm/examples/remote-open-ai-compatable.rs",
                                class: "text-indigo-600",
                                "remote open source models"
                            }
                        }
                    }
                }
            }
        }
    }
}
