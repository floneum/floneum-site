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

    let link = match chapter.maybe_link() {
        Some(link) => link,
        None => return rsx! {},
    };
    let url = link.location.as_ref().unwrap();

    let current_class = match book_url.starts_with(&*url.to_string()) {
        true => "bg-gray-200",
        false => "",
    };

    rsx! {
        Link { to: Route::Docs { child: *url },
            li { class: "m-1 rounded-md pl-2 {current_class}", "{link.name}" }
        }
    }
}

fn RightNav() -> Element {
    let page = use_book();

    rsx! {
        crate::table_of_contents::TableOfContents { sections: page.sections().to_vec() }
    }
}

fn Content() -> Element {
    rsx! {
        section { class: "body-font overflow-hidden mx-auto container pt-12 pb-12 md:w-2/3",
            div { class: "-my-8",
                div { class: "w-full mb-20 flex-wrap list-none rounded-md",
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
