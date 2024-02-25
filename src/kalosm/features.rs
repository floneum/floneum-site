use dioxus::prelude::*;

pub fn Features() -> Element {
    rsx! {
        div { class: "bg-white py-24 sm:py-32",
            div { class: "mx-auto max-w-7xl px-6 lg:px-8",
                div { class: "mx-auto max-w-2xl lg:text-center",
                    h2 { class: "text-base font-semibold leading-7 text-indigo-600",
                        "Powerful Private AI"
                    }
                    p { class: "mt-2 text-3xl font-bold tracking-tight text-gray-900 sm:text-4xl",
                        "Everything you need to build with local AI"
                    }
                    p { class: "mt-6 text-lg leading-8 text-gray-600",
                        "Kalosm is a powerful, private AI platform for building and deploying AI models locally. It's designed to be easy to use, controllable, and private."
                    }
                }
                div { class: "mx-auto mt-16 max-w-2xl sm:mt-20 lg:mt-24 lg:max-w-4xl",
                    dl { class: "grid max-w-xl grid-cols-1 gap-x-8 gap-y-10 lg:max-w-none lg:grid-cols-2 lg:gap-y-16",
                        div { class: "relative pl-16",
                            dt { class: "text-base font-semibold leading-7 text-gray-900",
                                div { class: "absolute left-0 top-0 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",
                                    svg {
                                        "stroke": "currentColor",
                                        "aria-hidden": "true",
                                        "viewBox": "0 0 24 24",
                                        "stroke-width": "1.5",
                                        "fill": "none",
                                        class: "h-6 w-6 text-white",
                                        path {
                                            "stroke-linejoin": "round",
                                            "stroke-linecap": "round",
                                            "d": "M9.813 15.904 9 18.75l-.813-2.846a4.5 4.5 0 0 0-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 0 0 3.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 0 0 3.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 0 0-3.09 3.09ZM18.259 8.715 18 9.75l-.259-1.035a3.375 3.375 0 0 0-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 0 0 2.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 0 0 2.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 0 0-2.456 2.456ZM16.894 20.567 16.5 21.75l-.394-1.183a2.25 2.25 0 0 0-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 0 0 1.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 0 0 1.423 1.423l1.183.394-1.183.394a2.25 2.25 0 0 0-1.423 1.423Z"
                                        }
                                    }
                                }
                                "\n\t\t\t  AI Models\n\t\t\t"
                            }
                            dd { class: "mt-2 text-base leading-7 text-gray-600",
                                "Kalosm supports over "
                                span { class: "bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text whitespace-pre-wrap",
                                    "35 models"
                                }
                                " across "
                                span { class: "bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text whitespace-pre-wrap",
                                    "5 different model types"
                                }
                                ". Models can be easily swapped out and updated depending on your hardware requirements and use case."
                            }
                        }
                        div { class: "relative pl-16",
                            dt { class: "text-base font-semibold leading-7 text-gray-900",
                                div { class: "absolute left-0 top-0 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",
                                    svg {
                                        "fill": "none",
                                        "viewBox": "0 0 24 24",
                                        "stroke-width": "1.5",
                                        "aria-hidden": "true",
                                        "stroke": "currentColor",
                                        class: "h-6 w-6 text-white",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "d": "M9 17.25v1.007a3 3 0 0 1-.879 2.122L7.5 21h9l-.621-.621A3 3 0 0 1 15 18.257V17.25m6-12V15a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 15V5.25m18 0A2.25 2.25 0 0 0 18.75 3H5.25A2.25 2.25 0 0 0 3 5.25m18 0V12a2.25 2.25 0 0 1-2.25 2.25H5.25A2.25 2.25 0 0 1 3 12V5.25"
                                        }
                                    }
                                }
                                "\n\t\t\t  Local Processing\n\t\t\t"
                            }
                            dd { class: "mt-2 text-base leading-7 text-gray-600",
                                "Kalosm models run locally which means "
                                span { class: "bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text whitespace-pre-wrap",
                                    "no expensive cloud bills, connection issues, or privacy concerns"
                                }
                                ". User data never needs to leave the device."
                            }
                        }
                        div { class: "relative pl-16",
                            dt { class: "text-base font-semibold leading-7 text-gray-900",
                                div { class: "absolute left-0 top-0 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",
                                    svg {
                                        "aria-hidden": "true",
                                        "fill": "none",
                                        "stroke-width": "1.5",
                                        "stroke": "currentColor",
                                        "viewBox": "0 0 24 24",
                                        class: "h-6 w-6 text-white",
                                        path {
                                            "stroke-linejoin": "round",
                                            "stroke-linecap": "round",
                                            "d": "M3.375 19.5h17.25m-17.25 0a1.125 1.125 0 0 1-1.125-1.125M3.375 19.5h7.5c.621 0 1.125-.504 1.125-1.125m-9.75 0V5.625m0 12.75v-1.5c0-.621.504-1.125 1.125-1.125m18.375 2.625V5.625m0 12.75c0 .621-.504 1.125-1.125 1.125m1.125-1.125v-1.5c0-.621-.504-1.125-1.125-1.125m0 3.75h-7.5A1.125 1.125 0 0 1 12 18.375m9.75-12.75c0-.621-.504-1.125-1.125-1.125H3.375c-.621 0-1.125.504-1.125 1.125m19.5 0v1.5c0 .621-.504 1.125-1.125 1.125M2.25 5.625v1.5c0 .621.504 1.125 1.125 1.125m0 0h17.25m-17.25 0h7.5c.621 0 1.125.504 1.125 1.125M3.375 8.25c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125m17.25-3.75h-7.5c-.621 0-1.125.504-1.125 1.125m8.625-1.125c.621 0 1.125.504 1.125 1.125v1.5c0 .621-.504 1.125-1.125 1.125m-17.25 0h7.5m-7.5 0c-.621 0-1.125.504-1.125 1.125v1.5c0 .621.504 1.125 1.125 1.125M12 10.875v-1.5m0 1.5c0 .621-.504 1.125-1.125 1.125M12 10.875c0 .621.504 1.125 1.125 1.125m-2.25 0c.621 0 1.125.504 1.125 1.125M13.125 12h7.5m-7.5 0c-.621 0-1.125.504-1.125 1.125M20.625 12c.621 0 1.125.504 1.125 1.125v1.5c0 .621-.504 1.125-1.125 1.125m-17.25 0h7.5M12 14.625v-1.5m0 1.5c0 .621-.504 1.125-1.125 1.125M12 14.625c0 .621.504 1.125 1.125 1.125m-2.25 0c.621 0 1.125.504 1.125 1.125m0 1.5v-1.5m0 0c0-.621.504-1.125 1.125-1.125m0 0h7.5"
                                        }
                                    }
                                }
                                "\n\t\t\t  Data Integrations\n\t\t\t"
                            }
                            dd { class: "mt-2 text-base leading-7 text-gray-600",
                                "Kalosm understands "
                                span { class: "bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text whitespace-pre-wrap",
                                    "10 different data formats"
                                }
                                " making it easy to integrate with your text, audio, or image data."
                            }
                        }
                        div { class: "relative pl-16",
                            dt { class: "text-base font-semibold leading-7 text-gray-900",
                                div { class: "absolute left-0 top-0 flex h-10 w-10 items-center justify-center rounded-lg bg-indigo-600",
                                    svg {
                                        "viewBox": "0 0 24 24",
                                        "stroke-width": "1.5",
                                        "stroke": "currentColor",
                                        "fill": "none",
                                        "aria-hidden": "true",
                                        class: "h-6 w-6 text-white",
                                        path {
                                            "stroke-linecap": "round",
                                            "stroke-linejoin": "round",
                                            "d": "M11.42 15.17 17.25 21A2.652 2.652 0 0 0 21 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 1 1-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 0 0 4.486-6.336l-3.276 3.277a3.004 3.004 0 0 1-2.25-2.25l3.276-3.276a4.5 4.5 0 0 0-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437 1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008Z"
                                        }
                                    }
                                }
                                "\n\t\t\t  Controllable Output\n\t\t\t"
                            }
                            dd { class: "mt-2 text-base leading-7 text-gray-600",
                                "Kalosm is built with controls to "
                                span { class: "bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text whitespace-pre-wrap",
                                    "tailor the output"
                                }
                                " of your models. This means you can adjust the output to fit your specific use case."
                            }
                        }
                    }
                }
            }
        }
    }
}
