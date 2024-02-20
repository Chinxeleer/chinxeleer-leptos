use crate::{
    error_template::{AppError, ErrorTemplate},
    routes::{
        about_section::AboutSection, blog::Blog, landing_page::LandingSection, main_nav::Nav,
        projects::ProjectsView, resume::ResumeView,
    },
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/chinxeleer.css"/>
        // <link
        // href="https://cdn.jsdelivr.net/npm/quill@2.0.0-rc.2/dist/quill.snow.css"
        // rel="stylesheet"
        // />
        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>

            <main class="bg-gradient-to-r from-orange-400 via-orange-300 to-orange-400  lg:px-40 px-10 md:px-20 ">
                <section class="min-h-screen">
                    <Nav/>
                    <Routes>
                        <Route path="" view=LandingSection/>
                        <Route path="about" view=AboutSection/>
                        <Route path="blog" view=Blog/>
                        <Route path="resume" view=ResumeView/>
                        <Route path="projects" view=ProjectsView/>
                    </Routes>
                </section>
            </main>
        </Router>
    }
}
