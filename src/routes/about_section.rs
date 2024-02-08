use leptos::*;

use crate::components::footer::AboutFooter;

#[component]
pub fn AboutSection() -> impl IntoView {
    view! {
        <section>
            <div class="max-w-3xl font-body text-center mx-auto">
                <h3 class="text-3xl py-1">
                    About Me <img class="h-20 w-20 inline" src="/cuddlyferris.svg" alt=""/>
                </h3>
                <p class="text-base text-balance text-gray-700">
                    "I'm a 3" <sup>"rd"</sup> "-year student at Wits University, studying "
                    <span class="text-orange-800">"Computer Science and Applied Mathematics"</span>
                    ".
                    Passionate about solving real-life problems, I primarily work with "
                    <span class="text-orange-800">"Rust"</span> " and
                    " <span class="text-orange-800">"Python"</span>
                    ", particularly for creating games, terminal apps, and web apps using leptos. Expect a mix of tech blogs and discussions about faith on my site,
                    as I'm a committed Christian who believes "
                    <span class="text-orange-800">"Jesus Christ is the Lord of my life"</span>
                    ". In my free time, I enjoy watching anime, exploring Options
                    Trading and Algorithmic Trading. My career goal is to join a fintech institution after completing my degree."
                </p>
            </div>
            <AboutFooter/>
        </section>
    }
}
