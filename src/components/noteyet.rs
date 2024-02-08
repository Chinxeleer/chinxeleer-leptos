use icondata as i;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn NotYet() -> impl IntoView {
    view! {
        <div class="mx-auto inline">
            <h1 class="text-teal-800 text-2xl m-auto">TO BE IMPLEMENTED</h1>
            <Icon class="text-4xl" icon=i::AiExclamationOutlined/>
        </div>
    }
}
