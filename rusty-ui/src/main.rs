use std::net::SocketAddr;
use std::sync::Arc;
use std::time::SystemTime;

use axum::extract::State;
use axum::routing::get_service;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use minijinja::{context, path_loader, Environment};
use serde::Serialize;

mod templates;
use templates::Templates;
use tower_http::services::ServeDir;
//mod frontend;

#[tokio::main]
async fn main() {
    let state = Arc::new(FrontendState {
        templates: Templates::default(),
    });

    let routes = Router::new()
        .route("/name", get(name_template))
        .route("/", get(home_handler))
        .route("/example", get(example_response))
        .route("/load_polling", get(load_polling))
        .with_state(state.clone())
        .merge(routes_static());
    //.fallback_service(routes_static());;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on addr {}:{}\n", addr.ip(), addr.port());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/public/", get_service(ServeDir::new("./public")))
}

async fn home_handler(State(state): State<Arc<FrontendState>>) -> impl IntoResponse {
    // You need to remember that everytime that you want to create a dynamic page,
    // you'll use context
    //
    // -> incoming request
    // -> parse data using extractors
    // -> compute intensive works using middlewares
    // -> parse computed data with extractors
    // (this is important for like JWT or tokenized auth)
    // -> compute the response and send it
    let output = state
        .templates
        .render_templates("pages/home.jinja2", context!())
        .unwrap();
    Html(output)
}

// about rendering patterns
//
// One of the solutions that you want to implement is the ability to "swap" old html with new one based on
// a user interaction. If im look at a list of elements and I click in the navigation, I want to be able to
// query the server only for that "fragment". That possible using HTMX in theory
// How do I know if I have the shell of the or not?
// You could have 2 different endpoints.
// /about                  for entering the page
// /template/about         for retrieving the data relate to it

#[derive(Clone)]
struct FrontendState {
    templates: Templates,
}

#[derive(Serialize)]
struct Breed {
    id: String,
    name: String,
}

#[derive(Serialize)]
struct User {
    id: String,
    name: String,
    favourite_breed: Breed,
}

async fn example_response() -> impl IntoResponse {
    Html("helo from server")
}

async fn load_polling(State(state): State<Arc<FrontendState>>) -> impl IntoResponse {
    // Artificially adds 2 seconds of cooldown, used to check the spinners
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let result = state
        .templates
        .render_templates(
            "components/load_polling.jinja2",
            context!(
            time => format!("{:?}", SystemTime::now())),
        )
        .unwrap();
    Html(result)
}

// Old example of how to render jinja template using
async fn name_template() -> impl IntoResponse {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env.add_filter("scream", templates::scream);
    env.add_test("is_sandro", templates::is_sandro);
    let tmpl = env.get_template("first.jinja2").unwrap();

    let shiba = Breed {
        id: "0".to_string(),
        name: "Shiba Shibo!".to_string(),
    };

    let user = User {
        id: "1".to_string(),
        name: "Sandro".to_string(),
        favourite_breed: shiba,
    };

    let ctx = context!(
        user => user,
        navigation => vec![
            context!( caption => "sus"),
            context!( caption => "sas"),
            context!( caption => "ses"),
        ]
    );
    let rendered = tmpl.render(ctx).unwrap();
    println!("{}", rendered);

    Html(rendered)
}
