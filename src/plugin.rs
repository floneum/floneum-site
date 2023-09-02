use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

#[inline_props]
pub fn PluginsList(cx: Scope) -> Element {
    let plugins = use_server_future(cx, (), move |_| async move {
        #[cfg(feature = "ssr")]
        {
            let index = floneumite::FloneumPackageIndex::load().await;
            index
                .entries().iter()
                .map(|plugin| {
                    let metadata = plugin.meta().unwrap();
                    PluginInfo {
                        name: metadata.name.clone(),
                        description: metadata.description.clone(),
                    }
                })
                .collect()
        }
        #[cfg(not(feature = "ssr"))]
        {
            Vec::new()
        }
    })?;
    let value: std::cell::Ref<'_, Vec<PluginInfo>> = plugins.value();
    
    const ROWS: usize = 3;
    fn translation(row: usize) -> String {
        let scroll_speed = ((row as i32 % 2) * 2 - 1)*100;
        let offset = if row % 2 == 0 { 50 } else { -150 };
        format!("translateX(calc(var(--scroll)*{scroll_speed}% + {offset}%))")
    }

    render! {
        for (i, col) in value.chunks_exact((value.len() / ROWS).max(1)).enumerate() {
            div {
                class: "flex flex-row w-[600vw] overflow-clip",
                transform: "{translation(i)}",
                for plugin in col.iter().chain(col.iter()).chain(col.iter()) {
                    Plugin {
                        plugin: plugin.clone()
                    }
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

#[inline_props]
pub fn Plugin(cx: Scope, plugin: PluginInfo) -> Element {
    let show_description = use_state(cx, || false);

    render! {
        button {
            class: "bg-slate-500 rounded-md p-2 w-full h-40 flex flex-col justify-center items-center m-2 text-2xl font-bold text-center",
            onclick: move |_| show_description.set(!show_description.get()),
            onmouseenter: move |_| show_description.set(true),
            onmouseleave: move |_| show_description.set(false),

            if *show_description.get() {
                plugin.description.to_string()
            }
            else {
                plugin.name.to_string()
            }
        }
    }
}
