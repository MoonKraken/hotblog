use leptos::*;

use crate::model::post::Post;

#[component]
pub fn BlogCard(blog_preview: Post) -> impl IntoView {
    let dt = format!("{}", blog_preview.dt.format("%b %e, %Y %I:%M%P"));
    view! {

        <a href={format!("/view/{}", blog_preview.id)}>
            <div class="bg-white p-6 rounded-lg shadow-md mb-6 mr-10 p-10 flex flex-none w-96 h-48">
                <img src={blog_preview.image_url} alt="Blog Thumbnail" class="w-32 h-24 rounded-lg object-cover mr-4"/>

                <div class="flex-none">
                    <h2 class="text-xl font-semibold mb-2">{blog_preview.title}</h2>

                    <p class="text-gray-600 mb-4">{blog_preview.text}</p>

                    <div class="flex justify-between">
                        <span class="text-gray-500">{dt}</span>
                    </div>
                </div>
            </div>
        </a>
    }
}
