use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TableOfContentsProps {
    sections: Vec<mdbook_shared::Section>,
}

pub fn TableOfContents(TableOfContentsProps { sections }: TableOfContentsProps) -> Element {
    let padding_map = ["pl-2", "pl-4", "pl-6", "pl-8", "pl-10"];

    rsx! {
        div { class: "hidden xl:sticky xl:top-[4.75rem] xl:-mr-6 xl:block xl:h-[calc(100vh-4.75rem)] xl:flex-none xl:overflow-y-auto xl:py-16 xl:pr-6",
            nav { "aria-labelledby": "on-this-page-title", class: "w-56",
                if !sections.is_empty() {
                    h2 {
                        id: "on-this-page-title",
                        class: "font-display text-sm font-medium text-slate-900",
                        "On this page"
                    }
                    ol { role: "list", class: "mt-4 space-y-3 text-sm",
                        for section in sections.iter().filter(|s| s.level <= 2) {
                            li {
                                key: "{section.id}",
                                class: "pb-2 {padding_map[section.level-1]}",
                                h3 {
                                    a {
                                        class: "font-normal text-slate-500 hover:text-slate-700",
                                        href: "#{section.id}",
                                        "{section.title}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
