use crate::docs::LAZY_BOOK;

use crate::*;
use mdbook_shared::SummaryItem;

#[component]
pub fn Learn() -> Element {
    rsx! {
        div {
            class: "w-full pt-12 text-sm backdrop-blur-lg bg-white/75",
            min_height: "100vh",
            div { class: "max-w-screen-2xl flex flex-row justify-between mx-auto",
                Content {}
                LeftNav {}
                RightNav {}
            }
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
        nav { class: "z-20 text-base hidden md:block fixed top-0 left-4 mt-36 mb-16 ml-2 w-[calc(100%-1rem)] md:w-60 h-full max-h-screen md:text-[13px] text-navy content-start overflow-y-auto leading-5 flex-wrap",
            for chapter in chapters.into_iter().flatten().filter(|chapter| chapter.maybe_link().is_some()) {
                SidebarSection { chapter: chapter }
            }
        }
    }
}

#[component]
fn SidebarSection(chapter: &'static SummaryItem<BookRoute>) -> Element {
    let link = chapter.maybe_link()?;

    let sections = link
        .nested_items
        .iter()
        .map(|link| rsx! { SidebarChapter { link: link } }.unwrap());

    rsx! {
        div { class: "pb-4",
            if let Some(url) = &link.location {
                Link { to: Route::Docs { child: *url }, h2 { class: "font-semibold", "{link.name}" } }
            }
            ul { class: "pl-2", {sections} }
        }
    }
}

#[component]
fn SidebarChapter(link: &'static SummaryItem<BookRoute>) -> Element {
    let link = link.maybe_link()?;
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
            Link { to: Route::Docs { child: *url }, "{shevron}{link.name}" }
            if show_chevron && show_dropdown {
                ul { class: "ml-6 border-l border-gray-300 py-1",
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

    let link = chapter.maybe_link()?;
    let url = link.location.as_ref().unwrap();

    let current_class = match book_url == url.to_string() {
        true => "bg-gray-200",
        false => "",
    };

    rsx! {
        Link { to: Route::Docs { child: *url },
            li { span { class: "m-1 px-2 rounded-md {current_class}", "{link.name}" } }
        }
    }
}

fn RightNav() -> Element {
    let page = use_book();

    rsx! { crate::table_of_contents::TableOfContents { sections: page.sections().to_vec() } }
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
