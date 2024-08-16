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
        <form>
            <fieldset>
                <legend>Copy-paste tool</legend>
                <p>
                    <label for="paste">Text Content: </label>
                    <textarea id="paste" placeholder="Paste content here..."/>
                </p>
                <p>
                    <label for="text_content">Upload Files: </label>
                    <input type="file" id="fileUpload" multiple/>
                </p>
                <p>
                    <label for="imageUpload">Upload Images: </label>
                    <input type="file" id="imageUpload" accept="image/*" multiple/>
                </p>
                <button class="save">Save</button>
                <button class="clear">Clear</button>
            </fieldset>
        </form>
    }
}
