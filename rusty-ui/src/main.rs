use std::sync::Arc;
use std::{net::SocketAddr, sync::RwLock};

use axum::extract::State;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use minijinja::{context, path_loader, Environment, Error, ErrorKind};
use serde::Serialize;

use rust_embed::RustEmbed;

#[tokio::main]
async fn main() {
    let state = Arc::new(FrontendState {
        templates: Templates::default(),
    });

    let routes = Router::new()
        .route("/name", get(name_template))
        .route("/", get(home_handler))
        .with_state(state.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Listening on addr {}:{}\n", addr.ip(), addr.port());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

async fn home_handler(State(state): State<Arc<FrontendState>>) -> impl IntoResponse {
    let output = state
        .templates
        .render_templates("pages/home.jinja2", context!())
        .unwrap();
    Html(output)
}

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

#[derive(RustEmbed)]
#[folder = "./templates"]
struct TemplateFiles;

#[derive(Clone)]
pub struct Templates {
    env: Arc<RwLock<Environment<'static>>>,
}

impl Default for Templates {
    fn default() -> Self {
        let mut env = Environment::new();
        env.set_loader(embedded_loader);
        env.add_filter("scream", scream);
        env.add_test("is_sandro", is_sandro);
        let env = Arc::new(RwLock::new(env));
        Self { env }
    }
}

impl Templates {
    pub fn render_templates<S: serde::Serialize>(
        &self,
        name: &str,
        ctx: S,
    ) -> Result<String, Error> {
        let some = self.env.read().unwrap();
        some.get_template(name)
            .map_err(|e| {
                println!("Encountered some error: {}", e.to_string());
                e
            })?
            .render(ctx)
            .map_err(|e| {
                println!("Encountered some rendering error: {}", e.to_string());
                e
            })
    }
}

fn embedded_loader(name: &str) -> Result<Option<String>, minijinja::Error> {
    let Some(file) = TemplateFiles::get(name) else {
        return Ok(None);
    };

    let val = String::from_utf8(file.data.to_vec())
        .map_err(|_| minijinja::Error::from(ErrorKind::CannotDeserialize))?;

    Ok(Some(val))
}

async fn name_template() -> impl IntoResponse {
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));
    env.add_filter("scream", scream);
    env.add_test("is_sandro", is_sandro);
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

fn scream(value: String) -> String {
    value.to_uppercase()
}
fn is_sandro(value: String) -> bool {
    value == "Sandro".to_string()
}
