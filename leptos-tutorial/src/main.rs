use leptos::prelude::*;
mod components;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = signal(1);

    let double_count = move || count.get() * 2;
    let set_bg = move || count.get() % 2 == 0;

    view! {
        <button
            class:red = move || count.get() % 2 == 1
            class=("text-color", set_bg)
            on:click = move |_| { *set_count.write() += 1; }
        >
            "Click me: " {count}
        </button>
        <p>
            {count}
        </p>
        <p>
            "Double count: " {move || count.get() * 2}
        </p>
        <progress
            max="50"
            value=double_count
        />
        <components::Comp count=count/>
        <components::Progress progress=count />
        <components::Progress progress=Signal::derive(double_count) />
        <components::ProgressOld count=count />
        <components::StaticList length=5 />
        <components::DynamicList initial_length=5 />
        <components::ControlledInput />
        <components::Textarea />
    }
}

fn main() {
    leptos::mount::mount_to_body(|| view! {<App/>});
    console_error_panic_hook::set_once();
}
