use leptos::prelude::*;

#[component]
pub fn ControlledInput() -> impl IntoView {
    let name = RwSignal::new("someinput".to_string());
    view! {
        <form>
            <input
                bind:value=name
            ></input>
        </form>
    }
}

#[component]
pub fn Textarea() -> impl IntoView {
    let (some_value, set_some_value) = signal("walue".to_string());
    view! {
        <textarea
            prop:value=move || some_value.get()
        >
            {some_value.get_untracked()}
        </textarea>
    }
}

#[component]
pub fn Select() -> impl IntoView {
    view! {}
}
