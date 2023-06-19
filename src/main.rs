use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main(){
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element{
    render!{
        Router {}
    }
}

#[inline_props]
fn Header(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-row justify-between items-center border-b-2 border-gray-700",
            p {
                class: "text-2xl font-bold m-2 ml-12",
                "Floneum"
            }
            div {
                class: "flex flex-row justify-self-end items-center",
                Link {
                    class: "text-xl font-bold m-2 mr-12",
                    target: Route::Home {},
                    "Home"
                }
                Link {
                    class: "text-xl font-bold m-2 mr-12",
                    target: Route::Docs {},
                    "Documentation"
                }
                a { margin: "10px", right: "10px", href: "https://github.com/floneum/floneum", img { src: "./GitHub-Mark-Light-32px.png", width: "32px", height: "32px" } }
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
        Outlet {}
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    render!{
        div {
            class: "flex flex-col mt-12",
            div {
                class: "flex flex-row justify-between items-center",
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
                        src: "./demo-img.png",
                        width: "400px",
                        height: "400px"
                    }
                }
            }
            div {
                class: "flex flex-row justify-between items-center",
                div {
                    class: "animate-fade-in-left-slow m-4",
                    img {
                        src: "./plugins.png",
                        width: "400px",
                        height: "400px"
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
                class: "flex flex-row justify-between items-center",
                div {
                    class: "animate-fade-in-left-slower m-4",
                    h2 {
                        class: "text-4xl font-bold mb-2",
                        "Extend Floneum with plugins written in your language of choice"
                    }
                    p {
                        "You can write plugins in any language that can be compiled to WebAssembly. Floneum provides ergonomic wrappers for rust, but you can also use C, Java, or Go"
                    }
                }
                div {
                    class: "animate-fade-in-right-slower m-4",
                    div {
                        class: "grid grid-cols-2 gap-4 justify-items-center items-center",
                        img {
                            src: "./rust_logo.svg",
                            width: "200px",
                            height: "200px"
                        }
                        img {
                            src: "./c_logo.png",
                            width: "200px",
                            height: "200px"
                        }
                        img {
                            src: "./java_logo.png",
                            width: "200px",
                            height: "200px"
                        }
                        img {
                            src: "./go_logo.png",
                            width: "200px",
                            height: "200px"
                        }
                    }
                }
            }
        }
    }
}

#[inline_props]
fn Docs(cx: Scope) -> Element {
    render!{
        "Documentation"
    }
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Header)]
        #[route("/")]
        Home {},
        #[route("/docs")]
        Docs {},
}