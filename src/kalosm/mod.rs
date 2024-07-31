use crate::{search::SearchModal, Route, KALOSM_SEARCH_INDEX};
use dioxus::prelude::*;

mod cta;
mod data_roller;
mod faq;
mod features;
mod footer;
mod header;
mod hero;
pub mod learn;

pub fn KalosmHome() -> Element {
    rsx! {
        hero::Hero {}
        features::Features {}
        faq::Faq {}
        cta::CallToAction {}
    }
}

#[component]
pub fn KalosmHeaderFooter() -> Element {
    rsx! {
        Title {
            "Kalosm"
        } 
        SearchModal { index: &KALOSM_SEARCH_INDEX }
        header::Header {}
        Outlet::<Route> {}
        footer::Footer {}
    }
}
