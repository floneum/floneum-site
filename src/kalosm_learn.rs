use crate::kalosm_docs::LAZY_BOOK;

use crate::*;
use mdbook_shared::SummaryItem;

#[component]
pub fn KalosmLearn(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "bg-[#FEF9EF] w-full h-full pt-12 text-sm backdrop-blur-lg", min_height: "100vh",
            div { class: "max-w-screen-2xl flex flex-row justify-between mx-auto",
                learn::Content {}
                LeftNav {}
                RightNav {}
            }
        }
    })
}

fn LeftNav(cx: Scope) -> Element {
    let chapters = vec![
        &LAZY_BOOK.summary.prefix_chapters,
        &LAZY_BOOK.summary.numbered_chapters,
        &LAZY_BOOK.summary.suffix_chapters,
    ];

    render! {
        nav { class: "z-20 text-base hidden md:block fixed top-0 left-4 mt-36 mb-16 ml-2 w-[calc(100%-1rem)] md:w-60 h-full max-h-screen md:text-[13px] text-navy content-start overflow-y-auto leading-5 flex-wrap",
            for chapter in chapters.into_iter().flatten().filter(|chapter| chapter.maybe_link().is_some()) {
                SidebarSection { chapter: chapter }
            }
        }
    }
}

#[component]
fn SidebarSection(cx: Scope, chapter: &'static SummaryItem<KalosmBookRoute>) -> Element {
    let link = chapter.maybe_link()?;

    let sections = link
        .nested_items
        .iter()
        .filter_map(|link| render! { SidebarChapter { link: link } });

    render! {
        div { class: "pb-4",
            if let Some(url) = &link.location {
                rsx! {
                    Link { to: Route::KalosmDocs { child: *url }, h2 { class: "font-semibold", "{link.name}" } }
                }
            }
            ul { class: "pl-2", sections }
        }
    }
}

#[component]
fn SidebarChapter(cx: Scope, link: &'static SummaryItem<KalosmBookRoute>) -> Element {
    let link = link.maybe_link()?;
    let url = link.location.as_ref().unwrap();
    let list_toggle = use_state(cx, || false);

    // current route of the browser, trimmed to the book url
    let book_url = use_book(cx).to_string();

    // for instance, if the current page is /docs/0.4/en/learn/overview
    // then we want to show the dropdown for /docs/0.4/en/learn
    let show_dropdown = *list_toggle.get() || book_url.starts_with(&*url.to_string());
    let show_chevron = !link.nested_items.is_empty();
    let shevron = if show_chevron { "> " } else { "" };

    render! {
        li { class: "pt-1",
            Link { to: Route::KalosmDocs { child: *url }, "{shevron}{link.name}" }
            if show_chevron && show_dropdown {
                rsx! {
                    ul { class: "ml-6 border-l border-gray-300 py-1",
                        for nest in link.nested_items.iter() {
                            LocationLink { chapter: nest }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn LocationLink(cx: Scope, chapter: &'static SummaryItem<KalosmBookRoute>) -> Element {
    let book_url = use_book(cx).to_string();

    let link = chapter.maybe_link()?;
    let url = link.location.as_ref().unwrap();

    let current_class = match book_url == url.to_string() {
        true => "bg-gray-200",
        false => "",
    };

    render! {
        Link { to: Route::KalosmDocs { child: *url },
            li { span { class: "m-1 px-2 rounded-md {current_class}", "{link.name}" } }
        }
    }
}

// Todo: wire this up to the sections of the current page and a scroll controller
fn RightNav(cx: Scope) -> Element {
    let page = use_book(cx);

    render! {
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

fn use_book(cx: &ScopeState) -> KalosmBookRoute {
    let route = use_route(cx).unwrap();
    match route {
        Route::KalosmDocs { child } => child,
        _ => unreachable!(),
    }
}
