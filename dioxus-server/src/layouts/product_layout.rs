use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn ProductLayout() -> Element {
    rsx! {
        div {
            "Products pages (from the layout)",
            Link {
                to: Route::ProductList {},
                "product"
            }
            Outlet::<Route> {}
        }
    }
}
