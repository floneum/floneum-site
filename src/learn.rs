use crate::router_floneum::LAZY_BOOK;

use crate::*;
use mdbook_shared::SummaryItem;

#[component]
pub fn Learn() -> Element {
    rsx! {
        div { class: "relative mx-auto flex w-full max-w-8xl flex-auto justify-center sm:px-2 lg:px-8 xl:px-12",
            LeftNav {}
            learn::Content {}
            RightNav {}
        }
    }
}

fn LeftNav() -> Element {
    let chapters = vec![
        &LAZY_BOOK.summary.prefix_chapters,
        &LAZY_BOOK.summary.numbered_chapters,
        &LAZY_BOOK.summary.suffix_chapters,
    ];

    rsx! {
        div { class: "hidden lg:relative lg:block lg:flex-none",
            div { class: "absolute bottom-0 right-0 top-16 hidden h-12 w-px bg-gradient-to-t from-slate-800 dark:block" }
            div { class: "absolute bottom-0 right-0 top-28 hidden w-px bg-slate-800 dark:block" }
            div { class: "sticky top-[4.75rem] -ml-0.5 h-[calc(100vh-4.75rem)] w-64 overflow-y-auto overflow-x-hidden py-16 pl-0.5 pr-8 xl:w-72 xl:pr-16",
                nav { class: "font-normal text-slate-500 hover:text-slate-700",
                    for chapter in chapters.into_iter().flatten().filter(|chapter| chapter.maybe_link().is_some()) {
                        SidebarSection { chapter }
                    }
                }
            }
        }
    }
}

#[component]
fn SidebarSection(chapter: &'static SummaryItem<BookRoute>) -> Element {
    let link = match chapter.maybe_link() {
        Some(link) => link,
        None => return rsx! {},
    };

    let sections = link.nested_items.iter().map(|link| {
        rsx! {
            SidebarChapter { link }
        }
        .unwrap()
    });

    rsx! {
        div { class: "pb-4",
            if let Some(url) = &link.location {
                Link { to: Route::Docs { child: *url },
                    h2 { class: "font-semibold", "{link.name}" }
                }
            }
            ul { class: "pl-2", {sections} }
        }
    }
}

#[component]
fn SidebarChapter(link: &'static SummaryItem<BookRoute>) -> Element {
    let link = match link.maybe_link() {
        Some(link) => link,
        None => return rsx! {},
    };
    let url = link.location.as_ref().unwrap();
    let list_toggle = use_signal(|| false);

    // current route of the browser, trimmed to the book url
    let book_url = use_book().to_string();

    // for instance, if the current page is /docs/0.4/en/learn/overview
    // then we want to show the dropdown for /docs/0.4/en/learn
    let show_dropdown = list_toggle() || book_url.starts_with(&*url.to_string());
    let show_chevron = !link.nested_items.is_empty();
    let shevron = if show_chevron { "> " } else { "" };

    rsx! {
        li { class: "pt-1",
            Link {
                class: "font-normal text-slate-500 hover:text-slate-700",
                to: Route::Docs { child: *url },
                "{shevron}{link.name}"
            }
            if show_chevron && show_dropdown {
                ul {
                    for nest in link.nested_items.iter() {
                        LocationLink { chapter: nest }
                    }
                }
            }
        }
    }
}

#[component]
fn LocationLink(chapter: &'static SummaryItem<BookRoute>) -> Element {
    let book_url = use_book().to_string();

    let link = match chapter.maybe_link() {
        Some(link) => link,
        None => return rsx! {},
    };
    let url = link.location.as_ref().unwrap();

    let current_class = match book_url == url.to_string() {
        true => "block w-full pl-3.5 before:pointer-events-none before:absolute before:-left-1 before:top-1/2 before:h-1.5 before:w-1.5 before:-translate-y-1/2 before:rounded-full font-semibold text-sky-500 before:bg-sky-500",
        false => "block w-full pl-3.5 before:pointer-events-none before:absolute before:-left-1 before:top-1/2 before:h-1.5 before:w-1.5 before:-translate-y-1/2 before:rounded-full text-slate-500 before:hidden before:bg-slate-300 hover:text-slate-600 hover:before:block",
    };

    rsx! {
        Link { to: Route::Docs { child: *url },
            li {
                span { class: "{current_class}", "{link.name}" }
            }
        }
    }
}

fn RightNav() -> Element {
    let page = use_book();

    rsx! {
        crate::table_of_contents::TableOfContents { sections: page.sections().to_vec() }
    }
}

pub fn Content() -> Element {
    rsx! {
        section { class: "min-w-0 max-w-2xl flex-auto px-4 py-16 lg:max-w-none lg:pl-8 lg:pr-0 xl:px-16",
            div { class: "-my-8",
                div { class: "w-full mb-20 flex-wrap list-none rounded-md",
                    article { class: "markdown-body pt-1", Outlet::<Route> {} }
                }
            }
        }
    }
}

fn use_book() -> BookRoute {
    let route = use_route::<Route>();
    match route {
        Route::Docs { child } => child,
        _ => unreachable!(),
    }
}
