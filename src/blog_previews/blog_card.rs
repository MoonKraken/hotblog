use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use super::blog_preview_details::BlogPreviewDetails;

#[component]
pub fn BlogCard(details: BlogPreviewDetails) -> impl IntoView {
    view! {
        <div class="bg-white p-6 rounded-lg shadow-md mb-6 flex items-center space-x-6">
            // <img src="path_to_thumbnail.jpg" alt="Blog Thumbnail" class="w-32 h-24 rounded-lg object-cover">

            <div class="flex-1">
                <h2 class="text-xl font-semibold mb-2">{details.title}</h2>

                <p class="text-gray-600 mb-4">{details.text}</p>

                <div class="flex justify-between items-center">
                    <span class="text-gray-500">Oct 13, 2023</span>
                    <a href="#" class="text-blue-600 hover:underline">Read More</a>
                </div>
            </div>
        </div>
    }
}
