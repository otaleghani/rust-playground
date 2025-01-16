use dioxus::prelude::*;

use components::Navbar;
use views::{Blog, Home, Contact, ProductList, ProductItem, ProductSpecs};
use components::PageNotFound;
use layouts::ProductLayout;

mod layouts;
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {

    #[layout(Navbar)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
    #[end_layout]

    #[nest("/product")]
        #[layout(ProductLayout)]
            #[route("/")]
            ProductList {},
            #[route("/:name")]
            ProductItem { name: String },
            #[nest("/:name")]
                #[route("/specs")]
                ProductSpecs { name: String },
            #[end_nest]
        #[end_layout]
    #[end_nest]

    #[nest("/myProd")]
        #[redirect("/", || Route::ProductList {})]
        #[redirect("/:name", |name: String| Route::ProductItem { name })]
    #[end_nest]

    #[route("/contact")]
    Contact {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: "./input.css" }

        Router::<Route> {}
    }
}
