use leptos::*;

mod app;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> })
}
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
        >
            "SSA please click me por favor : "

            {move || count.get()}
        </button>
        <input>
        </input>
    }
}