use dioxus::prelude::*;

pub fn KalosmHome(cx: Scope) -> Element {
    render!{
        div { class: "bg-[#FEF9EF] flex flex-col items-center pt-7",
            div { class: "flex w-full max-w-[1694px] items-stretch justify-between gap-5 px-5 max-md:max-w-full max-md:flex-wrap max-md:justify-center",
                div { class: "text-black text-5xl font-bold self-center my-auto max-md:text-4xl",
                    "Kalosm"
                }
                div { class: "flex-col justify-center bg-gray-400 bg-opacity-20 text-black text-opacity-20 text-center text-5xl font-bold relative w-1/2 fill-zinc-300 fill-opacity-40 overflow-hidden items-center px-8 py-4 max-md:text-4xl max-md:px-5 rounded-md",
                    "Search the docs"
                }
                div { class: "text-black text-5xl font-bold self-center whitespace-nowrap my-auto max-md:text-4xl",
                    a {
                        class: "github-button",
                        href: "https://github.com/floneum/floneum",
                        "data-size": "large",
                        "data-show-count": "true",
                        aria_label: "Star floneum/floneum on GitHub",
                        "Star"
                    }
                }
            }
            div { class: "text-center text-8xl font-bold max-w-[1945px] mt-[30vh] max-md:max-w-full max-md:text-4xl max-md:mt-10 bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text",
                "Build with controllable, private AI"
            }
            div { class: "justify-center text-black text-center text-5xl font-bold max-w-[1150px] mt-20 max-md:max-w-full max-md:text-4xl max-md:mt-10",
                "Kalosm makes it easy to interact with local, language, audio, and image models"
            }
            img {
                srcset: "https://cdn.builder.io/api/v1/image/assets/TEMP/02eebe30ae892586bd9e8f5abb1f49a097533aafa4634af897ac64c4c0005df4?apiKey=04af1066e8604f98b159e5700077f35c&width=100 100w, https://cdn.builder.io/api/v1/image/assets/TEMP/02eebe30ae892586bd9e8f5abb1f49a097533aafa4634af897ac64c4c0005df4?apiKey=04af1066e8604f98b159e5700077f35c&width=200 200w, https://cdn.builder.io/api/v1/image/assets/TEMP/02eebe30ae892586bd9e8f5abb1f49a097533aafa4634af897ac64c4c0005df4?apiKey=04af1066e8604f98b159e5700077f35c&width=400 400w, https://cdn.builder.io/api/v1/image/assets/TEMP/02eebe30ae892586bd9e8f5abb1f49a097533aafa4634af897ac64c4c0005df4?apiKey=04af1066e8604f98b159e5700077f35c&width=800 800w, https://cdn.builder.io/api/v1/image/assets/TEMP/02eebe30ae892586bd9e8f5abb1f49a097533aafa4634af897ac64c4c0005df4?apiKey=04af1066e8604f98b159e5700077f35c&width=1200 1200w, https://cdn.builder.io/api/v1/image/assets/TEMP/02eebe30ae892586bd9e8f5abb1f49a097533aafa4634af897ac64c4c0005df4?apiKey=04af1066e8604f98b159e5700077f35c&width=1600 1600w, https://cdn.builder.io/api/v1/image/assets/TEMP/02eebe30ae892586bd9e8f5abb1f49a097533aafa4634af897ac64c4c0005df4?apiKey=04af1066e8604f98b159e5700077f35c&width=2000 2000w, https://cdn.builder.io/api/v1/image/assets/TEMP/02eebe30ae892586bd9e8f5abb1f49a097533aafa4634af897ac64c4c0005df4?apiKey=04af1066e8604f98b159e5700077f35c&",
                "loading": "lazy",
                class: "aspect-[2.71] object-contain object-center w-full overflow-hidden self-stretch max-md:max-w-full"
            }
            div { class: "self-stretch w-full mt-28 max-md:max-w-full max-md:mt-10",
                div { class: "gap-5 flex max-md:flex-col max-md:items-stretch max-md:gap-0",
                    div { class: "flex flex-col items-stretch w-[60%] max-md:w-full max-md:mx-0",
                        div { class: "bg-cyan-700 flex grow flex-col items-stretch w-full pl-16 pr-10 py-12 rounded-[0px_100px_100px_0px] max-md:max-w-full max-md:mt-10 max-md:px-5",
                            div { class: "text-black text-center text-7xl font-bold self-center max-w-[1116px] mt-8 max-md:text-4xl",
                                "Your data"
                            }
                            div { class: "text-black text-center text-5xl font-bold self-center max-w-[700px] mt-12 max-md:text-4xl max-md:mt-10",
                                "Kalosm understands "
                                span {
                                    class: "bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text whitespace-pre-wrap",
                                    " 10 different data formats "
                                }
                                " making it easy to integrate with your local text, audio, or image data"
                            }
                        }
                    }
                    div { class: "flex flex-col items-stretch w-[40%] max-md:w-full max-md:mx-0",
                        div { class: "bg-teal-500 flex grow flex-col items-stretch w-full pl-16 pr-10 py-12 rounded-[100px_0px_0px_100px] max-md:max-w-full max-md:mt-10 max-md:px-5",
                            div { class: "text-black text-center text-7xl font-bold mt-8 max-md:max-w-full max-md:text-4xl",
                                "Your models"
                            }
                            div { class: "text-black text-center text-5xl font-bold self-center max-w-[700px] mt-12 max-md:text-4xl max-md:mt-10",
                                "Kalosm supports over "
                                span {
                                    class: "bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text whitespace-pre-wrap",
                                    " 35 models "
                                }
                                " across 5 different model types"
                            }
                        }
                    }
                }
            }
            div { class: "self-stretch flex w-full flex-col mt-1 pt-8 pb-12 max-md:max-w-full",
                div { class: "self-stretch max-md:max-w-full",
                    div { class: "gap-5 flex max-md:flex-col max-md:items-stretch max-md:gap-0",
                        div { class: "flex flex-col items-stretch w-[40%] max-md:w-full max-md:mx-0",
                            div { class: "bg-teal-500 flex grow flex-col items-stretch w-full pl-16 pr-10 py-12 rounded-[0px_100px_100px_0px] max-md:max-w-full max-md:mt-10 max-md:px-5",
                                div { class: "justify-center text-black text-center text-7xl font-bold self-stretch mt-7 max-md:max-w-full max-md:text-4xl",
                                    "Your outputs"
                                }
                                div { class: "text-black text-center text-5xl font-bold self-stretch mt-12 max-w-[700px] max-md:text-4xl max-md:mt-10",
                                    "Kalosm is built for "
                                    span {
                                        class: "bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text whitespace-pre-wrap",
                                        " controllable models "
                                    }
                                    " that let you tailor the output"
                                }
                            }
                        }
                        div { class: "flex flex-col items-stretch w-[60%] ml-5 max-md:w-full max-md:ml-0",
                            div { class: "bg-cyan-700 flex grow flex-col items-stretch w-full pl-16 pr-10 py-12 rounded-[100px_0px_0px_100px] max-md:max-w-full max-md:mt-10 max-md:px-5",
                                div { class: "text-black text-center self-center text-7xl font-bold max-md:max-w-full max-md:text-4xl",
                                    "Your compute"
                                }
                                div { class: "text-black text-center self-center text-5xl font-bold mt-12 max-w-[700px] max-md:text-4xl max-md:mt-10",
                                    "Kalosm models run locally which means "
                                    span {
                                        class: "bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-transparent bg-clip-text",
                                        " no expensive cloud bills, connection issues, or privacy concerns"
                                    }
                                }
                            }
                        }
                    }
                }
                div { class: "flex-col justify-center text-black text-center text-6xl font-bold relative z-[1] max-w-[1301px] overflow-hidden self-stretch pt-0 mt-36 px-16 items-end max-md:max-w-full max-md:text-4xl max-md:mt-10 max-md:pl-5 max-md:pr-8 max-md:pb-10",
                    "Text recognition"
                    img {
                        "loading": "lazy",
                        srcset: "https://cdn.builder.io/api/v1/image/assets/TEMP/d78cb2fad3c36bb1d14dcb69bfbb86416a9fae714cdbc08e65a45ca4f584a2a7?apiKey=04af1066e8604f98b159e5700077f35c&width=100 100w, https://cdn.builder.io/api/v1/image/assets/TEMP/d78cb2fad3c36bb1d14dcb69bfbb86416a9fae714cdbc08e65a45ca4f584a2a7?apiKey=04af1066e8604f98b159e5700077f35c&width=200 200w, https://cdn.builder.io/api/v1/image/assets/TEMP/d78cb2fad3c36bb1d14dcb69bfbb86416a9fae714cdbc08e65a45ca4f584a2a7?apiKey=04af1066e8604f98b159e5700077f35c&width=400 400w, https://cdn.builder.io/api/v1/image/assets/TEMP/d78cb2fad3c36bb1d14dcb69bfbb86416a9fae714cdbc08e65a45ca4f584a2a7?apiKey=04af1066e8604f98b159e5700077f35c&width=800 800w, https://cdn.builder.io/api/v1/image/assets/TEMP/d78cb2fad3c36bb1d14dcb69bfbb86416a9fae714cdbc08e65a45ca4f584a2a7?apiKey=04af1066e8604f98b159e5700077f35c&width=1200 1200w, https://cdn.builder.io/api/v1/image/assets/TEMP/d78cb2fad3c36bb1d14dcb69bfbb86416a9fae714cdbc08e65a45ca4f584a2a7?apiKey=04af1066e8604f98b159e5700077f35c&width=1600 1600w, https://cdn.builder.io/api/v1/image/assets/TEMP/d78cb2fad3c36bb1d14dcb69bfbb86416a9fae714cdbc08e65a45ca4f584a2a7?apiKey=04af1066e8604f98b159e5700077f35c&width=2000 2000w, https://cdn.builder.io/api/v1/image/assets/TEMP/d78cb2fad3c36bb1d14dcb69bfbb86416a9fae714cdbc08e65a45ca4f584a2a7?apiKey=04af1066e8604f98b159e5700077f35c&",
                        class: "object-contain object-center w-full overflow-hidden self-stretch max-md:max-w-full"
                    }
                }
                img {
                    "loading": "lazy",
                    srcset: "https://cdn.builder.io/api/v1/image/assets/TEMP/9ddc85949b39a4df849e25afc9b8d99e465b471433a910b870daac603af124cd?apiKey=04af1066e8604f98b159e5700077f35c&width=100 100w, https://cdn.builder.io/api/v1/image/assets/TEMP/9ddc85949b39a4df849e25afc9b8d99e465b471433a910b870daac603af124cd?apiKey=04af1066e8604f98b159e5700077f35c&width=200 200w, https://cdn.builder.io/api/v1/image/assets/TEMP/9ddc85949b39a4df849e25afc9b8d99e465b471433a910b870daac603af124cd?apiKey=04af1066e8604f98b159e5700077f35c&width=400 400w, https://cdn.builder.io/api/v1/image/assets/TEMP/9ddc85949b39a4df849e25afc9b8d99e465b471433a910b870daac603af124cd?apiKey=04af1066e8604f98b159e5700077f35c&width=800 800w, https://cdn.builder.io/api/v1/image/assets/TEMP/9ddc85949b39a4df849e25afc9b8d99e465b471433a910b870daac603af124cd?apiKey=04af1066e8604f98b159e5700077f35c&width=1200 1200w, https://cdn.builder.io/api/v1/image/assets/TEMP/9ddc85949b39a4df849e25afc9b8d99e465b471433a910b870daac603af124cd?apiKey=04af1066e8604f98b159e5700077f35c&width=1600 1600w, https://cdn.builder.io/api/v1/image/assets/TEMP/9ddc85949b39a4df849e25afc9b8d99e465b471433a910b870daac603af124cd?apiKey=04af1066e8604f98b159e5700077f35c&width=2000 2000w, https://cdn.builder.io/api/v1/image/assets/TEMP/9ddc85949b39a4df849e25afc9b8d99e465b471433a910b870daac603af124cd?apiKey=04af1066e8604f98b159e5700077f35c&",
                    class: "aspect-[10.26] object-contain object-center w-full overflow-hidden self-stretch max-md:max-w-full"
                }
                div { class: "self-stretch flex w-full flex-col mt-16 pl-20 items-start max-md:max-w-full max-md:mt-10 max-md:pl-5",
                    div { class: "text-orange-50 text-center text-2xl font-bold whitespace-nowrap bg-stone-900 justify-center items-stretch px-9 py-12 rounded-2xl max-md:max-w-full max-md:px-5",
                        "industry , Mr. Brown commented icily . \" Let us have a"
                    }
                    div { class: "justify-center text-black text-center text-6xl font-bold self-center max-w-[1301px] mt-44 max-md:max-w-full max-md:text-4xl max-md:mt-10",
                        "Chat Models"
                        br {}
                        span {
                            class: "text-lg font-bold mt-0",
                            "(lines to chat with a pirate: 5)"
                        }
                    }
                    img {
                        "loading": "lazy",
                        srcset: "https://cdn.builder.io/api/v1/image/assets/TEMP/15f18c295f5d600fcd7545e354038616697813a0c309ab064b80073c9e90c097?apiKey=04af1066e8604f98b159e5700077f35c&width=100 100w, https://cdn.builder.io/api/v1/image/assets/TEMP/15f18c295f5d600fcd7545e354038616697813a0c309ab064b80073c9e90c097?apiKey=04af1066e8604f98b159e5700077f35c&width=200 200w, https://cdn.builder.io/api/v1/image/assets/TEMP/15f18c295f5d600fcd7545e354038616697813a0c309ab064b80073c9e90c097?apiKey=04af1066e8604f98b159e5700077f35c&width=400 400w, https://cdn.builder.io/api/v1/image/assets/TEMP/15f18c295f5d600fcd7545e354038616697813a0c309ab064b80073c9e90c097?apiKey=04af1066e8604f98b159e5700077f35c&width=800 800w, https://cdn.builder.io/api/v1/image/assets/TEMP/15f18c295f5d600fcd7545e354038616697813a0c309ab064b80073c9e90c097?apiKey=04af1066e8604f98b159e5700077f35c&width=1200 1200w, https://cdn.builder.io/api/v1/image/assets/TEMP/15f18c295f5d600fcd7545e354038616697813a0c309ab064b80073c9e90c097?apiKey=04af1066e8604f98b159e5700077f35c&width=1600 1600w, https://cdn.builder.io/api/v1/image/assets/TEMP/15f18c295f5d600fcd7545e354038616697813a0c309ab064b80073c9e90c097?apiKey=04af1066e8604f98b159e5700077f35c&width=2000 2000w, https://cdn.builder.io/api/v1/image/assets/TEMP/15f18c295f5d600fcd7545e354038616697813a0c309ab064b80073c9e90c097?apiKey=04af1066e8604f98b159e5700077f35c&",
                        class: "aspect-[3.26] object-contain object-center w-full overflow-hidden self-stretch max-md:max-w-full"
                    }
                    div { class: "justify-center text-orange-50 md:text-2xl font-bold bg-stone-900 w-[644px] max-w-full items-stretch md:ml-10 mt-6 pl-6 pr-7 py-10 rounded-2xl max-md:max-w-full max-md:px-5",
                        "> Who are you?"
                        br {}
                        "¡Ahoy matey! I be yer trusty AI helper, ready to assist ye with anythingye needs. Whatcha need me teach ye today, cap'n? Just let me know whatit is and me crew of code monkeys will get crackin'."
                    }
                }
                div { class: "flex-col justify-center text-black text-center text-6xl font-bold relative z-[1] max-w-[1301px] overflow-hidden self-stretch pt-0 mt-36 px-16 items-end max-md:max-w-full max-md:text-4xl max-md:mt-10 max-md:pl-5 max-md:pr-8 max-md:pb-10",
                    "Audio transcription"
                    img {
                        "loading": "lazy",
                        src: "/assets/whisper-code.png",
                        class: "object-contain object-center w-full overflow-hidden self-stretch max-md:max-w-full"
                    }
                }
                div { class: "self-center flex w-[1159px] max-w-full flex-col mt-16 px-5 items-end max-md:mt-10",
                    div { class: "justify-center text-orange-50 text-2xl font-bold whitespace-nowrap bg-stone-900 items-stretch pl-8 pr-10 py-11 rounded-2xl max-md:max-w-full max-md:px-5",
                        "This is a test of voice transcription"
                    }
                }
                for (i, chunk, mut colors) in FEATURES.chunks_exact(3).enumerate().map(|(i, chunk)| (i, chunk, FEATURE_COLORS.iter().cycle().skip(i))) {
                    div {
                        class: "{(i==0).then_some(\"mt-24\").unwrap_or(\"mt-6\")} self-stretch flex flex-row items-start justify-between md:gap-5 sm:gap-1",
                        if i % 2 == 0 {
                            render! {
                                div { class: "flex flex-col items-center whitespace-pre h-24 justify-center text-white text-center lg:text-xl font-bold {colors.next().unwrap()} w-1/12 pt-2 pb-2 md:px-6 self-start",
                                    " "
                                }
                            }
                        }
                        for feature in chunk {
                            div { class: "flex flex-col items-center h-24 justify-center text-white text-center lg:text-xl font-bold {colors.next().unwrap()} w-1/3 pt-2 pb-2 md:px-6 self-start",
                                *feature
                            }
                        }
                        if i % 2 != 0 {
                            render! {
                                div { class: "flex flex-col items-center whitespace-pre h-24 justify-center text-white text-center lg:text-xl font-bold {colors.next().unwrap()} w-1/12 pt-2 pb-2 md:px-6 self-start",
                                    " "
                                }
                            }
                        }
                    }
                }
                div { class: "justify-center bg-gradient-to-r from-[#FE6D73] to-[#FFCB77] text-[#FEF9EF] text-center text-4xl font-bold w-1/2 self-center items-center mt-32 mb-11 py-12 px-8 rounded-[100px] max-md:text-2xl max-md:my-10 max-md:px-5 max-md:py-10",
                    "Get Started"
                }
            }
        }
    }
}

const FEATURES: &[&str] = &[
    "resource augmented generation",
    "live voice context",
    "text recognition",
    "Web crawling",
    "Image generation",
    "image segmentation",
    "web scraping",
    "constrained generation",
    "audio transcription",
    "Text Embedding",
    "custom text classifier training",
    "search result",
    "context",
];

const FEATURE_COLORS: &[&str] = &[
    "bg-cyan-700",
    "bg-red-400",
    "bg-teal-500",
    "bg-orange-300",
];