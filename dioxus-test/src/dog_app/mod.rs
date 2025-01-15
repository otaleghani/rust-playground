use dioxus::prelude::*;
use serde::{Serialize, Deserialize};
use crate::save_dog;

use self::server_fn::client::reqwest;

#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String
}

#[derive(Clone)]
struct TitleState(String);

#[component]
pub fn App() -> Element {
    // use_context_provider lets you create sharable vars between parent and 
    // child, making it easy to share stuff without changing the Signature
    use_context_provider(|| TitleState("AnvediNando".to_string()));

    rsx! {
        Title {},
        DogView {},
    }
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();

    rsx! {
        div { id: "title",
            h1 { "{title.0}" }
        }
    }
}


#[component]
fn DogViewOld() -> Element {
    // use_hook -> save just a value
    // use_signal -> save the value and change it afterwards
    
    //let img_src = use_hook(|| "https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    let mut img_src = use_signal(|| "https://images.dog.ceo/breeds/schnauzer-miniature/n02097047_3093.jpg");

    //let skip = move |evt: Event<MouseData>| {};
    let skip = move |_evt: Event<MouseData>| {};
    let save = move |_evt: Event<MouseData>| {
        img_src.set("https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg");
    };

    rsx! {
        div { id: "dogview",
            img { src: "{img_src}" }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: save, id: "save", "save" }
        }
    }
}

// Data fetching
#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
pub fn DogView() -> Element {
    // Old, unsafe fetch
    //let mut img_src = use_signal(|| "".to_string());
    //let skip = move |_evt: Event<MouseData>| {};
    //let save = move |_evt: Event<MouseData>| async move {
    //    let response = reqwest::get("https://dog.ceo/api/breeds/image/random")
    //        .await
    //        .unwrap()
    //        .json::<DogApi>()
    //        .await;
    //    img_src .set(response.unwrap().message);
    //};


    //let mut img_src = use_signal(||  "some".to_string());
    let mut img_src = use_resource(|| async move {
        match fetch_dog_image().await {
            Ok(dog) => dog.message,
            Err(_err) => "".to_string(),
        }
    });

    let skip = move |_evt: Event<MouseData>| {};
    //let _save = move |_evt: Event<MouseData>| {
    //    img_src.set("https://images.dog.ceo/breeds/pitbull/dog-3981540_1280.jpg".to_string());
    //};

    //let fetch_new = move |_evt: Event<MouseData>| async move {
    //    match fetch_dog_image().await {
    //        Ok(dog) => img_src.set(dog.message),
    //        Err(err) => tracing::info!("Sus sas no doggo found {err}"),
    //    }
    //};

    let fetch_new = move |_evt: Event<MouseData>| async move {
        //match fetch_dog_image().await {
        //    Ok(dog) => dog.message,
        //    Err(_err) => "".to_string(),
        //}
        img_src.restart();
        //save_dog(img_src.unwrap());

    };

    rsx! {
        div { id: "dogview",
            img { src: img_src.cloned().unwrap_or_default() }
        }
        div { id: "buttons",
            button { onclick: skip, id: "skip", "skip" }
            button { onclick: fetch_new, id: "save", "save" }
        }
    }
}

async fn fetch_dog_image() -> Result<DogApi, reqwest::Error> {
    let response = reqwest::get("https://dog.ceo/api/breeds/image/random").await?;
    let dog_api = response.json::<DogApi>().await?;
    Ok(dog_api)
}

#[derive(Serialize, Deserialize)]
struct Request {
    name: String,
}

//async fn save_dogz(image: String) -> Result<(), ServerFnError> { }
