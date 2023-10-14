use leptos::*;
use leptos_meta::*;

use super::blog_card::BlogCard;
use super::blog_preview_details::BlogPreviewDetails;

#[component]
fn BlogDescription() -> impl IntoView {
    view! {
        <div class="p-5">
            <div class="h-40 w-40 overflow-hidden rounded-full">
                <img src="http://cttm.io/images/CodeToTheMoonV1Square.png"/>
            </div>
            <div class="text-4xl">"Welcome to my awesome blog"</div>
            <div class="text-xl">"It's about cool stuff"</div>
            <div class="text-lg">"are you cool?"</div>
        </div>
    }
}

#[component]
pub fn BlogPreviews() -> impl IntoView {
    let blog_preview_list = vec!(
        BlogPreviewDetails {
            title: "Post1".to_string(),
            text: "This is my most recent blog post! Let me tell you about...".to_string()
        },
        BlogPreviewDetails {
            title: "Post2".to_string(),
            text: "This is my second post. Woohoo.".to_string()
        }
    );

    view! {
        <BlogDescription/>
        <div class="text-4xl p-4">Blog Posts</div>
        <div class="bg-gray-100 p-8">
            {
                blog_preview_list.into_iter().map(|details| {
                    view! {
                        <p>
                            <BlogCard details={details}/>
                        </p>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
    }
}
