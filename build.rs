use std::{env::current_dir, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    for (path, router) in [
        ("./kalosm_doc_src", "router.rs"),
        ("./blog", "router_blog.rs"),
        ("./doc_src", "router_floneum.rs"),
    ] {
        let mdbook_dir = manifest_dir.join(path).canonicalize().unwrap();
        println!("cargo:rerun-if-changed={}", mdbook_dir.display());
        let out_dir = current_dir().unwrap().join("src");

        let mut out = mdbook_gen::generate_router_build_script(mdbook_dir);
        out.push_str("\n");
        out.push_str("use super::*;\n");

        std::fs::write(out_dir.join(router), out).unwrap();
    }
}
