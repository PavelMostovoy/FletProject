#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info};

use crate::pages::common::BackToLanding;

pub fn Details() -> Element {
    info!("Details page opened");
    rsx! {
        h3{
            "Details Page"
        },
        BackToLanding{}
    }
}