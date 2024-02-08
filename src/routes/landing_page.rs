use icondata as i;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn LandingSection() -> impl IntoView {
    view! {
        <div class="text-center py-10 ">
            <div class="relative w-80 h-80 bg-slate-800 mx-auto mb-10 rounded-full overflow-hidden ">
                <img
                    class="object-cover rounded-full antialiased border-1 border-dotted border-orange-800"
                    src="/profile.jpg"
                    alt=""
                />
            </div>
            <h2 class="text-5xl py-2 text-orange-600 font-body antialiased">Blessing Kodze</h2>
            <h3 class="text-2xl py-2 font-body">"Developer and Computer Science Student"</h3>
            <p class="text-md py-5 leading-8 font-body">
                "I'm a 3rd year Computer Science Student at the University of Witwatersrand."
            </p>
        </div>
        <div class="text-5xl flex justify-center gap-16 py-3 text-orange-600">
            <Icon class="hover:text-orange-800" icon=i::BsGithub/>
            <Icon class="hover:text-orange-800" icon=i::BsLinkedin/>
        </div>
    }
}
