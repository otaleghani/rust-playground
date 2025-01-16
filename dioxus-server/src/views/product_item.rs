use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn ProductItem(name: String) -> Element {
    // fetch data eventually if needed, or create a context
    rsx! {
        div {
            "The item is: {name}",
            Link {
                to: Route::ProductSpecs { name: name },
                "View the specs for this item"
            }
        }
    }
}
