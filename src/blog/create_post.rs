use leptos::*;
#[component]
pub fn CreatePost() -> impl IntoView {
    view! {
        <h1 class="text-2xl mx-auto mb-10">Create Post</h1>
        <div class="bg-white" id="editor">
            <p>"Hello World!"</p>
            <p>"Some initial " <strong>"bold"</strong> " text"</p>
            <p>
                <br/>
            </p>
        </div>

        // <!-- Include the Quill library -->
        <script src="https://cdn.jsdelivr.net/npm/quill@2.0.0-rc.2/dist/quill.js"></script>

        // <!-- Initialize Quill editor -->
        <script>
            " const quill = new Quill('#editor', {
               theme: 'snow'
             });"
        </script>
    }
}
