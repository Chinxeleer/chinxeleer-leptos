use leptos::*;

#[component]
pub fn ProjectsView() -> impl IntoView {
    view! {
        <section>
            <div class="max-w-[85rem] px-4 py-8 sm:px-6 lg:px-8 lg:py-14 mx-auto">
                // <!-- Title -->
                <div class="max-w-2xl mx-auto text-center mb-10 lg:mb-14">
                    <h2 class="text-2xl font-bold md:text-4xl md:leading-tight text-teal-800">
                        "Projects"
                    </h2>
                    // TODO: will add a better description or subheading
                    <p class="mt-1 text-teal-800"></p>
                </div>
                // <!-- End Title -->

                // <!-- Grid -->
                <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
                    // <!-- Card -->
                    <a
                        class="group flex flex-col h-full border border-teal-900 hover:border-transparent hover:shadow-lg transition-all duration-300 rounded-xl p-5"
                        href="https://github.com/Chinxeleer/password_manager"
                    >
                        <div class="aspect-w-16 aspect-h-11">
                            <img
                                class="w-full object-cover rounded-xl"
                                src="https://images.unsplash.com/photo-1618060932014-4deda4932554?w=900&auto=format&fit=crop&q=60&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1yZWxhdGVkfDV8fHxlbnwwfHx8fHw%3D"
                                alt="Image Description"
                            />
                        </div>
                        <div class="my-6">
                            <h3 class="text-xl font-semibold text-teal-800 font-body">
                                "Chinxeleer Password Manager"
                            </h3>
                            <p class="mt-5 text-teal-800 font-body">
                                "A simple password manager that stores your passwords from browser through getting CSVs of passwords stored in your browser."
                            </p>
                        </div>
                        <div class="mt-auto flex items-center gap-x-3">
                            <img
                                class="w-8 h-8 rounded-full"
                                src="/profile.jpg"
                                alt="Image Description"
                            />
                            <div>
                                <h5 class="text-sm text-teal-800 font-body">"By Blessing Kodze"</h5>
                            </div>
                        </div>
                    </a>
                // <!-- End Card -->

                </div>
                // <!-- End Grid -->

                // <!-- Card -->
                <div class="mt-8 text-center">
                    <a
                        class="py-3 px-4 inline-flex items-center gap-x-1 text-sm font-medium rounded-full border border-gray-200 bg-orange-300 text-teal-800 shadow-sm hover:bg-orange-400 disabled:opacity-50 disabled:pointer-events-none "
                        href="https://github.com/Chinxeleer?tab=repositories"
                    >
                        See more projeccts
                        <svg
                            class="flex-shrink-0 w-4 h-4"
                            xmlns="http://www.w3.org/2000/svg"
                            width="24"
                            height="24"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                            stroke-width="2"
                            stroke-linecap="round"
                            stroke-linejoin="round"
                        >
                            <path d="m9 18 6-6-6-6"></path>
                        </svg>
                    </a>
                </div>
            // <!-- End Card -->
            </div>
        </section>
    }
}
