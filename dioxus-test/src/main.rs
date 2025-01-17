use dioxus::prelude::*;
//use tokio::time::Duration;

//use self::server_fn::axum;
mod dog_app;
mod testinggrounds;

static CSS: Asset = asset!("/assets/main.css");
// static ICON: Asset = asset!("/assets/favicon.ico", ImageAssetOptions::new().with_avif());
// Anvedi come balla nando
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let resultamentos = use_server_future(get_data);
    let resultamento = use_resource(get_data);

    rsx! {
        div { "{resultamento.value():?}" }
        button {
            "sbottonamento"
        }
        document::Stylesheet { href: CSS }

        //testinggrounds::Testone {}
        //testinggrounds::Testone {}
        //testinggrounds::SingleProps { id: "soso".to_string() }
        //testinggrounds::Optional {}
        //testinggrounds::List {}
        //testinggrounds::SyntaxSugar { count: 5, show_title: true }
        //testinggrounds::StructProp { helo: "heloz" }
        testinggrounds::MusicPlayers { }
        testinggrounds::ChangeGlobal { }

        dog_app::App { }
    }
}

#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    use std::io::Write;

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("dogs.txt")
        .unwrap();

    let _ = file.write_fmt(format_args!("{image}\n"));

    Ok(())
}

#[server]
async fn get_data() -> Result<String, ServerFnError> {
    //let count
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    Ok(String::from("sus"))
}

//async fn launch(config: ServeConfig, app: fn() -> Element) {
//    let router = axum::Router::new().serve_dioxus_application(config, app);
//
//    let socket_addr = dioxus_cli_config::fullstack_address_or_localhost();
//    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
//    axum::serve(listener, router).await.unwrap();
//}
