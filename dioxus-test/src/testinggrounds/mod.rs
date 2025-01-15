use dioxus::prelude::*;

static SONG: GlobalSignal<String> = Signal::global(|| "Sandalo".to_string());

#[component]
pub fn ChangeGlobal() -> Element {
    rsx! {
        button {
            onclick: move |_| *SONG.write() = "Scarpetta".to_string(),
            "{SONG}",
        }
    }
}

#[component]
pub fn Testone() -> Element {
    rsx! { 
        div { "anvedi" }
        button { 
            onclick: move |_| tracing::info!("Clicked"),
            div { "anvedi che button!" }
        }
    }
}

#[component]
pub fn SingleProps(id: String) -> Element {
    rsx! {
        div { 
            button { 
                id: id.clone(),
                onclick: move |_| tracing::info!("Cliecked {id}"),
                div { "dimmi id" }
            }
        }
    }
}


#[derive(Props, PartialEq, Clone)]
pub struct Propoli {
    helo: String
}

#[component]
pub fn StructProp(props: Propoli) -> Element {
    rsx! { 
        div { "{props.helo}" }
    }
}

#[component]
pub fn Optional() -> Element {
    let show: Option<String> = Some("some merlforx".to_string());
    //let show: Option<String> = None;

    //rsx! {
    //    h1 { {show.unwrap_or_else(|| "Nothing found".to_string())}},
    //}
    //match show {
    //    Some(value) => rsx! { h1 { "found {value}" } },
    //    None => rsx! { h1 { "sus sas" } },
    //}
    
    //if let Some(value) = show {
    //    rsx! { h1 { "found this: {value}" } }
    //} else {
    //    rsx! { h1 { "big sad" } }
    //}

    if show.is_some() { 
        tracing::info!("There is a value");
        //let value = show.unwrap();
        //rsx! { h1 { "{value}" } }
    }
    if show.is_none() { 
        tracing::info!("There is a none");
    }

    rsx! { h1 {"sus sas"} }
}

#[component]
pub fn List() -> Element {
    rsx! {
        ul {
            {(0..5).map(|i| rsx! { li { "{i}" }})}
        }
    }
}

#[component]
pub fn SyntaxSugar(show_title: bool, count: i32) -> Element {
    rsx! {
        if show_title {
            "true"
        }
        for item in 0..count {
            div {
                key: item,
                "{item}"
            }
        }
    }
}

// Context and Signals
#[derive(Clone, Copy)]
struct MusicPlayer {
    song: Signal<String>
}

fn use_music_player_provider() {
    let song = use_signal(|| "Nando".to_string());
    use_context_provider(|| MusicPlayer { song });
}

#[component]
pub fn MusicPlayers() -> Element {
    use_music_player_provider();
    rsx! {
        Player { },
        CurrentlyPlaying { },
    }
}

#[component]
pub fn Player() -> Element {
    rsx! {
        button { 
            onclick: move |_| consume_context::<MusicPlayer>().song.set("sempronio".to_string()),
            "Sus"
        }
    }
}

#[component]
pub fn CurrentlyPlaying() -> Element {
    let current_song = use_context::<MusicPlayer>().song;
    rsx! {
        div { "Currently Playing: {current_song}" }
    }
}
