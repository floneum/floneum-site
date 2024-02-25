use crate::*;
use mdbook_shared::SummaryItem;

#[component]
pub fn Blog() -> Element {
    rsx! {
        div {
            class: "w-full pt-12 backdrop-blur-lg bg-white/75",
            min_height: "100vh",
            div { class: "max-w-screen-2xl flex flex-row justify-between mx-auto",
                Content {}
                RightNav {}
            }
        }
    }
}

#[component]
fn LocationLink(chapter: &'static SummaryItem<BookRoute>) -> Element {
    let book_url = use_book().to_string();

    let link = chapter.maybe_link()?;
    let url = link.location.as_ref().unwrap();

    let current_class = match book_url.starts_with(&*url.to_string()) {
        true => "bg-gray-200",
        false => "",
    };

    rsx! {
        Link { to: Route::Docs { child: *url }, li { class: "m-1 rounded-md pl-2 {current_class}", "{link.name}" } }
    }
}

fn RightNav() -> Element {
    let page = use_book();

    rsx! {
        div {
            class: "z-20 overflow-y-auto hidden xl:block fixed top-0 mt-36 pb-16 pl-3.5 -ml-3.5 w-60 h-full md:text-[13px] leading-5 text-navy docs-right-sidebar",
            right: "calc(40vw - 40.875rem)",
            h2 { class: "pb-4 font-semibold", "On this page" }
            ul { class: "",
                for section in page.sections().iter().filter(|s| s.level <= 2) {
                    li { class: "pb-2",
                        Link { to: NavigationTarget::<Route>::External("#".to_string() + &section.id), "{section.title}" }
                    }
                }
            }
        }
    }
}

fn Content() -> Element {
    rsx! {
        section { class: "body-font overflow-hidden mx-auto container pt-12 pb-12 w-2/3",
            div { class: "-my-8",
                div { class: "w-full mb-20 flex-wrap list-none rounded-md",
                    style {
                        ".markdown-body ul {{ list-style: disc; }}"
                        ".markdown-body li {{ display: list-item; }}"
                        ".markdown-body h1 {{ font-size: 4rem; padding-top: 2rem; padding-bottom: 2rem; }}"
                        ".markdown-body h2 {{ font-size: 3rem; padding-top: 1.5rem; padding-bottom: 1.5rem; }}"
                        ".markdown-body h3 {{ font-size: 2rem; padding-top: 1rem; padding-bottom: 1rem; }}"
                        ".markdown-body h4 {{ font-size: 1.5rem; padding-top: 0.75rem; padding-bottom: 0.75rem; }}"
                        ".markdown-body h5 {{ font-size: 1rem; padding-top: 0.5rem; padding-bottom: 0.5rem; }}"
                        ".markdown-body h6 {{ font-size: 0.875rem; padding-top: 0.5rem; padding-bottom: 0.5rem; }}"
                        ".markdown-body p {{ font-size: 1.5rem; padding-top: 0.5rem; padding-bottom: 0.5rem; }}"
                        ".markdown-body a {{ color: #3182ce; text-decoration: none; }}"
                        ".markdown-body a:hover {{ text-decoration: underline; }}"
                        ".markdown-body ul {{ padding-left: 1.5rem; }}"
                        ".markdown-body ol {{ padding-left: 1.5rem; }}"
                        ".markdown-body li {{ padding-top: 0.25rem; padding-bottom: 0.25rem; }}"
                        ".markdown-body code {{ border-radius: 0.1rem; word-wrap: normal; background-color: rgba(100, 100, 100, .5) }}"
                        ".markdown-body pre {{ border-radius: 0.375rem; padding: 1em; word-wrap: normal; white-space: pre; }}"
                        ".markdown-body span {{ word-wrap: normal; white-space: pre; }}"
                        ".markdown-body video {{ border-radius: 0.5rem; }}"
                    }
                    article { class: "markdown-body pt-1", Outlet::<Route> {} }
                }
            }
        }
    }
}

fn use_book() -> BlogRoute {
    let route = use_route::<Route>();
    match route {
        Route::Blog { child } => child,
        _ => unreachable!(),
    }
}
