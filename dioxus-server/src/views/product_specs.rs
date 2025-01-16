use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn ProductSpecs(name: String) -> Element {
    rsx! {
        div {
            "You are view the specs of {name}",
            Link {
                to: Route::ProductItem { name: name },
                "Return to item"
            }
        }
    }
}
