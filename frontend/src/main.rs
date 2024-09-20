use leptos::*;
use leptos_router::*;

mod app;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(PasteBinApp)
}

#[component]
fn PasteBinApp() -> impl IntoView {
    view! {
        <Router>
            <h1>My PasteBin</h1>
            <main>
                <Routes>
                    <Route path="" view=DisplayForm />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn DisplayForm() -> impl IntoView {
    let query = use_query_map();
    let text_input = move || query().get("text_input").cloned().unwrap_or_default();
    let file_input = move || query().get("files").cloned().unwrap_or_default();
    let image_input = move || query().get("images").cloned().unwrap_or_default();

    view! {
        <table>
            <tr>
                <td>Text input:</td>
                <td>{text_input}</td>
            </tr>
            <tr>
                <td>Files uploaded:</td>
                <td>{file_input}</td>
            </tr>
            <tr>
                <td>Images uploaded:</td>
                <td>{image_input}</td>
            </tr>
        </table>
        <Form method="GET" action="">
            <fieldset>
                <legend>Copy-paste tool</legend>
                <p>
                    <label for="paste">Text Content:</label>
                    <textarea name="text_input" nameid="paste" placeholder="Paste content here..." />
                </p>
                <p>
                    <label for="text_content">Upload Files:</label>
                    <input name="files" type="file" id="fileUpload" multiple value=file_input />
                </p>
                <p>
                    <label for="imageUpload">Upload Images:</label>
                    <input
                        name="images"
                        type="file"
                        id="imageUpload"
                        accept="image/*"
                        multiple
                        value=image_input
                    />
                </p>
                <input type="submit"/>
                <button class="clear">Clear</button>
            </fieldset>
        </Form>
    }
}
