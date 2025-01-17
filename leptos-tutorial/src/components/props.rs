use leptos::prelude::*;

#[component]
pub fn Progress(
    //#[prop(optional)] max: u16,
    #[prop(default = 50)] max: u16,
    //progress: impl Fn() -> i32 + Send + Sync + 'static,
    #[prop(into)] progress: Signal<i32>,
    #[prop(default = "supercalifracivo".to_string())] nando: String,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
        <p>{nando}</p>
    }
}
