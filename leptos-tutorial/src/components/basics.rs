use leptos::prelude::*;

#[component]
pub fn Comp(count: ReadSignal<i32>) -> impl IntoView {
    view! {
        <div>
            {count}
        </div>
    }
}

#[component]
pub fn ProgressOld(count: ReadSignal<i32>) -> impl IntoView {
    view! {
        <progress max="50" value=count />
    }
}

#[component]
pub fn ButtonExample() -> impl IntoView {
    view! {
        <button>
            "nome bottonz"
        </button>
    }
}
