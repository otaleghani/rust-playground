use leptos::prelude::*;

#[component]
pub fn StaticView() -> impl IntoView {
    let values = vec![0, 1, 2];
    view! {
        <p>{values.clone()}</p>
        <ul>
            {values.into_iter()
                .map(|n| view! { <li>{n}</li> })
                .collect::<Vec<_>>()
            }
        </ul>
    }
}

#[component]
pub fn StaticList(length: usize) -> impl IntoView {
    let counters = (1..=length).map(|idx: usize| RwSignal::new(idx));

    let counter_buttons = counters
        .map(|count| {
            view! {
               <li>
                   <button
                       on:click=move |_| *count.write() += 1
                   >{count}</button>
               </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        {counter_buttons}
    }
}

#[component]
pub fn DynamicList(initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..=initial_length)
        .map(|idx: usize| (idx, ArcRwSignal::new(idx + 1)))
        .collect::<Vec<_>>();

    let (counters, set_counters) = signal(initial_counters);
    let add_counter = move |_| {
        let sig = ArcRwSignal::new(next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig));
        });
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>
                "Add counter"
            </button>
        </div>
        <ul>
            <For
                each=move || counters.get()
                key=|counter| counter.0
                children= move |(id, counter)| {
                    let count = RwSignal::from(counter);
                    view! {
                       <button
                           on:click=move |_| *count.write() += 1
                       >
                           {count}
                       </button>
                       <button on:click=move |_| {
                           set_counters
                               .write()
                               .retain(|(counter_id, _)| {
                                   counter_id != &id
                               });
                       }
                       >"Remove"</button>
                    }
                }
            />
        </ul>
    }
}
