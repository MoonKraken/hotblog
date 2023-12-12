use leptos::*;

use super::errors_fallback::error_fallback;
use super::blog_preview_card::BlogPreviewCard;
use crate::model::blog_post::Post;
use crate::repository::blog_repository::get_previews;

#[component]
fn BlogDescription() -> impl IntoView {
    view! {
        <div class="p-5 flex flex-col items-center">
            <div class="mb-5 h-40 w-40 shadow-xl overflow-hidden rounded-full">
                <img src="http://cttm.io/images/CodeToTheMoonV1Square.png"/>
            </div>
            <div class="p-2 text-4xl">"The Roaming Crab"</div>
            <div class="p-2 text-xl">"It's a travel blog about fun places"</div>
        </div>
    }
}

#[component]
pub fn BlogPreviews() -> impl IntoView {
    let post_resource = create_resource(
        || {},
        |_| async move { get_previews(None, None, 20, 10).await },
    );

    let previews_view = move || -> Option<Result<View, _>>{
        post_resource.and_then(|previews: &Vec<Post>| {
            previews
                .into_iter()
                .map(|preview| {
                    view! {
                        <BlogPreviewCard blog_preview={preview.clone()}/>
                    }
                })
                .collect_view()
        })
    };

    view! {
        <BlogDescription/>
        <div class="dark:bg-gray-800 p-8 rounded-lg flex flex-wrap max-w-full">
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <ErrorBoundary fallback={error_fallback()}>
                    {previews_view}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
