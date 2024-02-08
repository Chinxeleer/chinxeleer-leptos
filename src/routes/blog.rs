use leptos::*;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <div class="max-w-[85rem] px-4 py-10 sm:px-6 lg:px-8 lg:py-14 mx-auto">
            // <!-- Title -->
            <div class="max-w-2xl mx-auto mb-10 lg:mb-14">
                <h2 class="text-2xl font-bold md:text-4xl md:leading-tight">
                    "Random Thoughts..."
                </h2>
            </div>
            // <!-- End Title -->

            <div class="max-w-2xl mx-auto divide-y divide-orange-500">
                <div class="py-8 first:pt-0 last:pb-0">
                    <div class="flex gap-x-5">

                        <div>
                            <a href="#1">
                                <h3 class="md:text-lg font-semibold text-gray-800 hover:text-orange-800">
                                    Can I cancel at anytime?
                                </h3>
                            </a>
                            <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                            <p class="mt-1 text-gray-500">
                                Yes, you can cancel anytime no questions are asked while you cancel but we would highly appreciate if you will give us some feedback.
                            </p>
                            <div class="mt-4 grid lg:flex lg:justify-between lg:items-center gap-y-5 lg:gap-y-0">
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

                        </div>
                    </div>
                </div>

                <div class="py-8 first:pt-0 last:pb-0">
                    <div class="flex gap-x-5">
                        <div>
                            <a href="#2">
                                <h3 class="md:text-lg font-semibold text-gray-800 hover:text-orange-800">
                                    My team has credits. How do we use them?
                                </h3>
                            </a>
                            <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                            <p class="mt-1 text-gray-500">
                                Once your team signs up for a subscription plan. This is where we sit down, grab a cup of coffee and dial in the details.
                            </p>
                        </div>
                    </div>
                </div>

                <div class="py-8 first:pt-0 last:pb-0">
                    <div class="flex gap-x-5">

                        <div>
                            <a href="#3">
                                <h3 class="md:text-lg font-semibold text-gray-800 hover:text-orange-800">
                                    "How does Preline's pricing work?"
                                </h3>
                            </a>
                            <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                            <p class="mt-1 text-gray-500">
                                Our subscriptions are tiered. Understanding the task at hand and ironing out the wrinkles is key.
                            </p>
                        </div>
                    </div>
                </div>

                <div class="py-8 first:pt-0 last:pb-0">
                    <div class="flex gap-x-5">

                        <div>
                            <a href="">
                                <h3 class="md:text-lg font-semibold text-gray-800 hover:text-orange-800">
                                    How secure is Preline?
                                </h3>
                            </a>
                            <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                            <p class="mt-1 text-gray-500">
                                Protecting the data you trust to Preline is our first priority. This part is really crucial in keeping the project in line to completion.
                            </p>
                        </div>
                    </div>
                </div>

                <div class="py-8 first:pt-0 last:pb-0">
                    <div class="flex gap-x-5">

                        <div>
                            <a href="">
                                <h3 class="md:text-lg font-semibold text-gray-800 hover:text-orange-800">
                                    " How do I get access to a theme I purchased?"
                                </h3>
                            </a>
                            <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                            <p class="mt-1 text-gray-500">
                                " If you lose the link for a theme you purchased, don't panic! We've got you covered. You can login to your account, tap your avatar in the upper right corner, and tap Purchases. If you didn't create a login or can't remember the information, you can use our handy Redownload page, just remember to use the same email you originally made your purchases with."
                            </p>
                        </div>
                    </div>
                </div>

                <div class="py-8 first:pt-0 last:pb-0">
                    <div class="flex gap-x-5">

                        <div>
                            <a href="">
                                <h3 class="md:text-lg font-semibold text-gray-800 hover:text-orange-800">
                                    Upgrade License Type
                                </h3>
                            </a>
                            <p class="mt-2 mb-2 text-sm text-teal-800">September 12, 2022</p>
                            <p class="mt-1 text-gray-500">
                                There may be times when you need to upgrade your license from the original type you purchased and we have a solution that ensures you can apply your original purchase cost to the new license purchase.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
