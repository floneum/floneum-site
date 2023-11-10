#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Clone)]
pub struct PluginInfo {
    name: String,
    description: String,
}

#[tokio::main]
async fn main() {
    // let index = floneumite::FloneumPackageIndex::load().await;
    // let packages: Vec<_> = index
    //     .entries()
    //     .iter()
    //     .map(|plugin| {
    //         let metadata = plugin.meta().unwrap();
    //         PluginInfo {
    //             name: metadata.name.clone(),
    //             description: metadata.description.clone(),
    //         }
    //     })
    //     .collect();

    // let bytes = postcard::to_stdvec(&packages).unwrap();
    // std::fs::write("plugins.bin", bytes).unwrap();
}
