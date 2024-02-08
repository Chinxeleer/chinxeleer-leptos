use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav class="px-10 py-5 mb-12 flex justify-between md:flex-wrap">
            <a href="/">
                <h1 class="text-xl font-bold hover:text-orange-800 font-body cursor-pointer">
                    chinxeleer
                </h1>
            </a>
            <ul class="flex justify-between space-x-4">
                <a href="/about">
                    <li class="hover:bg-orange-800 px-4 py-2 rounded-md hover:text-orange-300 hover:font-bold text-gray-700 font-body">
                        About
                    </li>
                </a>
                <a href="/resume">
                    <li class="px-4 hover:bg-orange-800 py-2 text-gray-700 hover:text-orange-300 hover:font-bold rounded-md font-body">
                        Resume
                    </li>
                </a>
                <a href="/projects">
                    <li class="rounded-md hover:bg-orange-800 px-4 py-2 text-gray-700 hover:text-orange-300 hover:font-bold font-body">
                        Projects
                    </li>
                </a>
                <a href="/blog">
                    <li class="rounded-md hover:bg-orange-800 px-4 py-2 text-gray-700 hover:text-orange-300 hover:font-bold font-body">
                        Blog
                    </li>
                </a>
            </ul>

        </nav>
    }
}
