use dioxus::prelude::*;

use crate::Route;

#[component]
pub fn ProductList() -> Element {
    rsx! {
        ul {
            { (0..5).map(|i| rsx! 
                { li { Link { to: Route::ProductItem { name: i.to_string() }, "{i}" }}}
            )}
        }
    }
}
