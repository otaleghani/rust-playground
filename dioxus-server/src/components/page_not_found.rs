use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}"}
    }
}
