use dioxus::prelude::*;

#[derive(Clone)]
struct InputField {
    id: usize,
    value: String
}

#[component]
pub fn Contact() -> Element {
    // Surely I need another thing here
    let mut input_fields: Signal<Vec<InputField>> = use_signal(|| vec![]);
    let mut counter = use_signal(|| 0);
    let action = move |event: Event<FormData>| {
        let name = &event.values()["sandro"][0];
        tracing::info!("{name:?}")
    };

    // On click, add a new component after it

    rsx! {
        form {
            onsubmit: action,
            //oninput: move |event| values.set(event.value()),
            input {
                name: "pertini"
                //value: "{name}",
                //oninput: move |event: Event<FormData>| name.set(event.value())
            }
            //{(i..counter).map(|i| rsx! {input { name: i.to_string()}})}
            {(0..5).map(|i| rsx! { "{i}" })}
            input { name: "sandro" }
            input { r#type: "submit" }
        }

        button {
            onclick: move |_| {
                counter += 1;
                input_fields.modify(|fields| {
                    fields.push(InputField{
                        id: *counter.current(),
                        value: String::new(),
                    })
                })

                tracing::info!("clicked");
            },
            "add new"
        }
    }
}
