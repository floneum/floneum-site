use dioxus::prelude::*;
use std::rc::Rc;

#[component]
pub fn PluginsList() -> Element {
    let plugins: Rc<Vec<PluginInfo>> = use_hook(|| {
        let bytes = include_bytes!("../plugins.bin");
        Rc::new(postcard::from_bytes(bytes).unwrap())
    });

    const ROWS: usize = 5;

    fn translation(row: usize) -> String {
        let scroll_speed = ((row as i32 % 2) * 2 - 1) * 100;
        let offset = if row % 2 == 0 { 50 } else { -150 };
        format!("translateX(calc(var(--scroll)*{scroll_speed}% + {offset}%))")
    }

    rsx! {
        h2 { class: "text-4xl font-bold text-center", "{plugins.len()} built in plugins" }
        for (i , col) in plugins.chunks_exact((plugins.len() / ROWS).max(1)).enumerate() {
            div {
                class: "flex flex-row w-[600vw] overflow-clip",
                transform: "{translation(i)}",
                for plugin in col.iter().chain(col.iter()).chain(col.iter()) {
                    Plugin { plugin: plugin.clone() }
                }
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
pub struct PluginInfo {
    name: String,
    description: String,
}

#[component]
pub fn Plugin(plugin: PluginInfo) -> Element {
    let mut show_description = use_signal(|| false);

    rsx! {
        button {
            class: "bg-slate-500 rounded-md p-2 w-full h-40 flex flex-col justify-center items-center m-2 text-2xl font-bold text-center",
            onclick: move |_| show_description.toggle(),
            onmouseenter: move |_| show_description.set(true),
            onmouseleave: move |_| show_description.set(false),

            if show_description() {
                "{plugin.description}"
            } else {
                "{plugin.name}"
            }
        }
    }
}
