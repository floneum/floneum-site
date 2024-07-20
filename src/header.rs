use crate::{search::Search, BookRoute, Route};
use dioxus::prelude::*;

fn desktop_nav_link_class(route: &Route, link: &Route) -> &'static str {
    if route == link {
        "inline-flex items-center border-b-2 border-indigo-500 px-1 pt-1 text-sm font-medium text-gray-900"
    } else {
        "inline-flex items-center border-b-2 border-transparent px-1 pt-1 text-sm font-medium text-gray-500 hover:border-gray-300 hover:text-gray-700"
    }
}

fn mobile_nav_link_class(route: &Route, link: &Route) -> &'static str {
    if route == link {
        "block border-l-4 border-indigo-500 bg-indigo-50 py-2 pl-3 pr-4 text-base font-medium text-indigo-700"
    } else {
        "block border-l-4 border-transparent py-2 pl-3 pr-4 text-base font-medium text-gray-600 hover:border-gray-300 hover:bg-gray-50 hover:text-gray-800"
    }
}

pub fn Header() -> Element {
    let mut mobile_menu_open = use_signal(|| false);
    let route = use_route::<Route>();

    let links = [
        ("Home", Route::Home {}),
        (
            "Docs",
            Route::Docs {
                child: BookRoute::Index {},
            },
        ),
    ];

    rsx! {
        nav { class: "bg-white shadow",
            div { class: "mx-auto max-w-7xl px-2 sm:px-4 lg:px-8",
                div { class: "flex h-16 justify-between",
                    div { class: "flex px-2 lg:px-0",
                        div { class: "flex flex-shrink-0 items-center",
                            Link {
                                class: "text-xl m-2 md:mr-12 flex flex-row items-center",
                                to: Route::Home {},
                                img {
                                    src: "/assets/Icon.png",
                                    class: "h-8 w-8 mx-2",
                                    alt: "Floneum"
                                }
                                "Floneum"
                            }
                        }
                        div { class: "hidden lg:ml-6 lg:flex lg:space-x-8",
                            for (text , link) in links.clone() {
                                Link {
                                    class: desktop_nav_link_class(&route, &link),
                                    to: link,
                                    {text}
                                }
                            }
                        }
                    }
                    div { class: "flex flex-1 items-center justify-center px-2 lg:ml-6 lg:justify-end",
                        div { class: "w-full max-w-lg lg:max-w-xs", Search {} }
                    }
                    div { class: "flex items-center lg:hidden",
                        button {
                            onclick: move |_| mobile_menu_open.toggle(),
                            "aria-expanded": "false",
                            r#type: "button",
                            "aria-controls": "mobile-menu",
                            class: "relative inline-flex items-center justify-center rounded-md p-2 text-gray-400 hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-indigo-500",
                            span { class: "absolute -inset-0.5" }
                            span { class: "sr-only", "Open main menu" }
                            svg {
                                "stroke": "currentColor",
                                "fill": "none",
                                "stroke-width": "1.5",
                                "viewBox": "0 0 24 24",
                                "aria-hidden": "true",
                                class: "block h-6 w-6",
                                path {
                                    "stroke-linecap": "round",
                                    "stroke-linejoin": "round",
                                    "d": "M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
                                }
                            }
                            svg {
                                "viewBox": "0 0 24 24",
                                "stroke": "currentColor",
                                "stroke-width": "1.5",
                                "aria-hidden": "true",
                                "fill": "none",
                                class: "hidden h-6 w-6",
                                path {
                                    "stroke-linejoin": "round",
                                    "d": "M6 18L18 6M6 6l12 12",
                                    "stroke-linecap": "round"
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: if mobile_menu_open() { "lg:hidden block" } else { "lg:hidden hidden" },
                id: "mobile-menu",
                div { class: "space-y-1 pb-3 pt-2",
                    for (text , link) in links {
                        Link {
                            class: mobile_nav_link_class(&route, &link),
                            to: link,
                            role: "menuitem",
                            tabindex: "-1",
                            {text}
                        }
                    }
                }
            }
        }
    }
}
