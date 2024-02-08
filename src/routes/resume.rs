use leptos::*;

use crate::components::noteyet::NotYet;

#[component]
pub fn ResumeView() -> impl IntoView {
    view! {
        <div class="text-center my-auto">
            <p>"Resume View"</p>
            <NotYet/>
        </div>
    }
}
