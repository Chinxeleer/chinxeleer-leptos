use leptos::*;

#[component]
pub fn ProjectsView() -> impl IntoView {
    view! {
        <div class="grid sm:grid-cols-2 lg:grid-cols-3 gap-6">
            // new card starts
            <a
                class="group flex flex-col h-full border border-gray-200 hover:border-transparent hover:shadow-lg transition-all duration-300 rounded-xl p-5 dark:border-gray-700 dark:hover:border-transparent dark:hover:shadow-black/[.4] dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                href="#"
            >
                <div class="aspect-w-16 aspect-h-11">
                    <img
                        class="w-full object-cover rounded-xl"
                        src="https://images.unsplash.com/photo-1633114128174-2f8aa49759b0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2070&q=80"
                        alt="Image Description"
                    />
                </div>
                <div class="my-6">
                    <h3 class="text-xl font-semibold text-teal-700">
                        Announcing a free plan for small teams
                    </h3>
                    <p class="mt-5 text-teal-700">
                        At Wake, our mission has always been focused on bringing openness.
                    </p>
                </div>
                // <div class="flex mb-3 gap-2">
                // <p class="inline-flex items-center gap-1.5 py-1.5 px-3 rounded-md text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200">
                // Announcements
                // </p>
                // <p class="inline-flex items-center gap-1.5 py-1.5 px-3 rounded-md text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200">
                // Announcements
                // </p>
                // <p class="inline-flex items-center gap-1.5 py-1.5 px-3 rounded-md text-xs font-medium bg-gray-100 text-gray-800 dark:bg-gray-800 dark:text-gray-200">
                // Announcements
                // </p>
                // </div>

                // strts here
                <div class="grid lg:flex lg:justify-between lg:items-center gap-y-5 lg:gap-y-0">
                    // <!-- Badges/Tags -->
                    <div>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Plan
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Web development
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Free
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Team
                        </a>
                    </div>
                </div>

                <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                // ends here
                <div class="mt-auto flex items-center gap-x-3">
                    <img class="w-8 h-8 rounded-full" src="/profile.jpg" alt="Image Description"/>
                    <div>
                        <h5 class="text-sm text-gray-800">By Blessing Kodze</h5>
                    </div>
                </div>
            </a>
            // new card starts
            <a
                class="group flex flex-col h-full border border-gray-200 hover:border-transparent hover:shadow-lg transition-all duration-300 rounded-xl p-5 dark:border-gray-700 dark:hover:border-transparent dark:hover:shadow-black/[.4] dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                href="#"
            >
                <div class="aspect-w-16 aspect-h-11">
                    <img
                        class="w-full object-cover rounded-xl"
                        src="https://images.unsplash.com/photo-1633114128174-2f8aa49759b0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2070&q=80"
                        alt="Image Description"
                    />
                </div>
                <div class="my-6">
                    <h3 class="text-xl font-semibold text-teal-700">
                        Announcing a free plan for small teams
                    </h3>
                    <p class="mt-5 text-teal-700">
                        At Wake, our mission has always been focused on bringing openness.
                    </p>
                </div>
                // strts here
                <div class="grid lg:flex lg:justify-between lg:items-center gap-y-5 lg:gap-y-0">
                    // <!-- Badges/Tags -->
                    <div>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Plan
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Web development
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Free
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Team
                        </a>
                    </div>
                </div>

                <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                // ends here
                <div class="mt-auto flex items-center gap-x-3">
                    <img class="w-8 h-8 rounded-full" src="/profile.jpg" alt="Image Description"/>
                    <div>
                        <h5 class="text-sm text-gray-800">By Blessing Kodze</h5>
                    </div>
                </div>
            </a>
            // new card starts
            <a
                class="group flex flex-col h-full border border-gray-200 hover:border-transparent hover:shadow-lg transition-all duration-300 rounded-xl p-5 dark:border-gray-700 dark:hover:border-transparent dark:hover:shadow-black/[.4] dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                href="#"
            >
                <div class="aspect-w-16 aspect-h-11">
                    <img
                        class="w-full object-cover rounded-xl"
                        src="https://images.unsplash.com/photo-1633114128174-2f8aa49759b0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2070&q=80"
                        alt="Image Description"
                    />
                </div>
                <div class="my-6">
                    <h3 class="text-xl font-semibold text-teal-700">
                        Announcing a free plan for small teams
                    </h3>
                    <p class="mt-5 text-teal-700">
                        At Wake, our mission has always been focused on bringing openness.
                    </p>
                </div>
                // strts here
                <div class="grid lg:flex lg:justify-between lg:items-center gap-y-5 lg:gap-y-0">
                    // <!-- Badges/Tags -->
                    <div>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Plan
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Web development
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Free
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Team
                        </a>
                    </div>
                </div>

                <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                // ends here
                <div class="mt-auto flex items-center gap-x-3">
                    <img class="w-8 h-8 rounded-full" src="/profile.jpg" alt="Image Description"/>
                    <div>
                        <h5 class="text-sm text-gray-800">By Blessing Kodze</h5>
                    </div>
                </div>
            </a>
            // new card starts
            <a
                class="group flex flex-col h-full border border-gray-200 hover:border-transparent hover:shadow-lg transition-all duration-300 rounded-xl p-5 dark:border-gray-700 dark:hover:border-transparent dark:hover:shadow-black/[.4] dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                href="#"
            >
                <div class="aspect-w-16 aspect-h-11">
                    <img
                        class="w-full object-cover rounded-xl"
                        src="https://images.unsplash.com/photo-1633114128174-2f8aa49759b0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2070&q=80"
                        alt="Image Description"
                    />
                </div>
                <div class="my-6">
                    <h3 class="text-xl font-semibold text-teal-700">
                        Announcing a free plan for small teams
                    </h3>
                    <p class="mt-5 text-teal-700">
                        At Wake, our mission has always been focused on bringing openness.
                    </p>
                </div>
                // strts here
                <div class="grid lg:flex lg:justify-between lg:items-center gap-y-5 lg:gap-y-0">
                    // <!-- Badges/Tags -->
                    <div>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Plan
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Web development
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Free
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Team
                        </a>
                    </div>
                </div>

                <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                // ends here
                <div class="mt-auto flex items-center gap-x-3">
                    <img class="w-8 h-8 rounded-full" src="/profile.jpg" alt="Image Description"/>
                    <div>
                        <h5 class="text-sm text-gray-800">By Blessing Kodze</h5>
                    </div>
                </div>
            </a>
            // new card starts
            <a
                class="group flex flex-col h-full border border-gray-200 hover:border-transparent hover:shadow-lg transition-all duration-300 rounded-xl p-5 dark:border-gray-700 dark:hover:border-transparent dark:hover:shadow-black/[.4] dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                href="#"
            >
                <div class="aspect-w-16 aspect-h-11">
                    <img
                        class="w-full object-cover rounded-xl"
                        src="https://images.unsplash.com/photo-1633114128174-2f8aa49759b0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2070&q=80"
                        alt="Image Description"
                    />
                </div>
                <div class="my-6">
                    <h3 class="text-xl font-semibold text-teal-700">
                        Announcing a free plan for small teams
                    </h3>
                    <p class="mt-5 text-teal-700">
                        At Wake, our mission has always been focused on bringing openness.
                    </p>
                </div>
                // strts here
                <div class="grid lg:flex lg:justify-between lg:items-center gap-y-5 lg:gap-y-0">
                    // <!-- Badges/Tags -->
                    <div>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Plan
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Web development
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Free
                        </a>
                        <a
                            class="m-0.5 inline-flex items-center gap-1.5 py-2 px-3 rounded-full text-sm bg-orange-200 text-teal-800 hover:bg-orange-700"
                            href="#"
                        >
                            Team
                        </a>
                    </div>
                </div>

                <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                // ends here
                <div class="mt-auto flex items-center gap-x-3">
                    <img class="w-8 h-8 rounded-full" src="/profile.jpg" alt="Image Description"/>
                    <div>
                        <h5 class="text-sm text-gray-800">By Blessing Kodze</h5>
                    </div>
                </div>
            </a>
            // new card starts
            <a
                class="group flex flex-col h-full border border-gray-200 hover:border-transparent hover:shadow-lg transition-all duration-300 rounded-xl p-5 dark:border-gray-700 dark:hover:border-transparent dark:hover:shadow-black/[.4] dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                href="#"
            >
                <div class="aspect-w-16 aspect-h-11">
                    <img
                        class="w-full object-cover rounded-xl"
                        src="https://images.unsplash.com/photo-1633114128174-2f8aa49759b0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2070&q=80"
                        alt="Image Description"
                    />
                </div>
                <div class="my-6">
                    <h3 class="text-xl font-semibold text-teal-700">
                        Announcing a free plan for small teams
                    </h3>
                    <p class="mt-5 text-teal-700">
                        At Wake, our mission has always been focused on bringing openness.
                    </p>
                </div>
                <div class="mt-auto flex items-center gap-x-3">
                    <img class="w-8 h-8 rounded-full" src="/profile.jpg" alt="Image Description"/>
                    <div>
                        <h5 class="text-sm text-gray-800">By Blessing Kodze</h5>
                    </div>
                </div>
            </a>
            // new card starts
            <a
                class="group flex flex-col h-full border border-gray-200 hover:border-transparent hover:shadow-lg transition-all duration-300 rounded-xl p-5 dark:border-gray-700 dark:hover:border-transparent dark:hover:shadow-black/[.4] dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                href="#"
            >
                <div class="aspect-w-16 aspect-h-11">
                    <img
                        class="w-full object-cover rounded-xl"
                        src="https://images.unsplash.com/photo-1633114128174-2f8aa49759b0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2070&q=80"
                        alt="Image Description"
                    />
                </div>
                <div class="my-6">
                    <h3 class="text-xl font-semibold text-teal-700">
                        Announcing a free plan for small teams
                    </h3>
                    <p class="mt-5 text-teal-700">
                        At Wake, our mission has always been focused on bringing openness.
                    </p>
                </div>
                <div class="mt-auto flex items-center gap-x-3">
                    <img class="w-8 h-8 rounded-full" src="/profile.jpg" alt="Image Description"/>
                    <div>
                        <h5 class="text-sm text-gray-800">By Blessing Kodze</h5>
                    </div>
                </div>
            </a>
            // new card starts
            <a
                class="group flex flex-col h-full border border-gray-200 hover:border-transparent hover:shadow-lg transition-all duration-300 rounded-xl p-5 dark:border-gray-700 dark:hover:border-transparent dark:hover:shadow-black/[.4] dark:focus:outline-none dark:focus:ring-1 dark:focus:ring-gray-600"
                href="#"
            >
                <div class="aspect-w-16 aspect-h-11">
                    <img
                        class="w-full object-cover rounded-xl"
                        src="https://images.unsplash.com/photo-1633114128174-2f8aa49759b0?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2070&q=80"
                        alt="Image Description"
                    />
                </div>
                <div class="my-6">
                    <h3 class="text-xl font-semibold text-teal-700">
                        Announcing a free plan for small teams
                    </h3>
                    <p class="mt-5 text-teal-700">
                        At Wake, our mission has always been focused on bringing openness.
                    </p>
                </div>
                <div class="mt-auto flex items-center gap-x-3">
                    <img class="w-8 h-8 rounded-full" src="/profile.jpg" alt="Image Description"/>
                    <div>
                        <h5 class="text-sm text-gray-800">By Blessing Kodze</h5>
                    </div>
                </div>
            </a>

        </div>
    }
}
