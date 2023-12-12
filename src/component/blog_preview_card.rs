use leptos::*;

use crate::model::blog_post::Post;

#[component]
pub fn BlogPreviewCard(blog_preview: Post) -> impl IntoView {
    let dt = format!("{}", blog_preview.dt.format("%b %e, %Y %I:%M%P"));
    view! {
        <a href={format!("/view/{}", blog_preview.id)}>
            <div class="transform transition duration-300 hover:scale-105 hover:shadow-2xl dark:bg-gray-600 p-6 rounded-lg shadow-md mb-6 mr-10 flex flex-none w-96 h-48">
                <img src={blog_preview.image_url} alt="Blog Thumbnail" class="w-32 h-32 rounded-lg object-cover mr-4"/>

                <div class="flex-none">
                    <h2 class="text-xl font-semibold mb-2 w-48 h-10 truncate">{blog_preview.title}</h2>

                    <p class="dark:text-gray-200 mb-4 w-32 h-18 truncate">{blog_preview.text}</p>

                    <div class="flex justify-between">
                        <span class="dark:text-gray-200">{dt}</span>
                    </div>
                </div>
            </div>
        </a>
    }
}
