use std::rc::Rc;

use crate::errors_fallback::error_fallback;
use leptos::logging::log;
use leptos::*;
use leptos_router::*;

use crate::model::post::Post;
use crate::repository::post::get_post;
use crate::repository::post::UpsertPost;
use crate::view_post::blog_post::BlogPost;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Params, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct EditPostParams {
    post_id: Option<String>,
}

fn format_rfc3339_without_timezone(datetime: DateTime<Utc>) -> String {
    datetime.format("%Y-%m-%dT%H:%M:%S").to_string()
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params: Memo<Result<_, _>> = use_params::<EditPostParams>();
    let navigate = use_navigate();
    let post_resource: Resource<_, Result<Post, ServerFnError>> = create_resource(
        move || params.get(),
        |params| async move {
            match params {
                Ok(EditPostParams { post_id: Some(s) }) => get_post(s).await,
                // if no id is in the URL path parameter, assume we are making a new post
                _ => Ok(Post::new_empty()),
            }
        },
    );

    let upsert_post = create_server_action::<UpsertPost>();

    create_effect(move |_| {
        let id = upsert_post.value().get();
        if let Some(Ok(id)) = id {
            navigate(format!("/view/{}", id).as_str(), Default::default());
        }
    });

    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback={error_fallback()}>
                <label class="block mb-4">
                <span class="text-gray-700">Title</span>
                    <input class="mt-1 p-2 w-full border rounded-md" type="text" id="title" name="title"
                        on:input=move |ev| {
                            post_resource.update(|curr| curr.as_mut().unwrap().as_mut().unwrap().title = event_target_value(&ev));
                        }
                    prop:value={post_resource.get().map(|res| res.ok()).flatten().map(|post| post.title)}/>
                </label>
            <h1>{move || post_resource.and_then(move |post| post.title.clone())}</h1>
                <div class="flex h-screen">
                <div class="min-w-[50%] bg-gray-100 p-10">
                <ActionForm action=upsert_post>
                    <label class="block mb-4">
                    <span class="text-gray-700">Date</span>
                    <input class="mt-1 p-2 w-full border rounded-md" type="datetime" id="datetime" name="dt"
                        on:input=move |ev| {
                            let dt: String = event_target_value(&ev);
                            log!("{:?}", dt);
                            let chrono_dt = DateTime::parse_from_rfc3339(&dt);
                            let utc_dt = match chrono_dt {
                                Ok(dt) => dt.with_timezone(&Utc),
                                _ => Utc::now() // this means the browser gave us something wrong?
                            };
                            post_resource.update(|curr| curr.as_mut().unwrap().as_mut().unwrap().dt = utc_dt);
                        }
                    prop:value={get_post_from_res(post_resource).map(|post| format_rfc3339_without_timezone(post.dt))}
                    />
                    </label>
                    <label class="block mb-4">
                    <span class="text-gray-700">Image URL</span>
                    <input class="mt-1 p-2 w-full border rounded-md" type="text" id="image_url" name="image_url"
                        on:input=move |ev| {
                            post_resource.update(|curr| curr.as_mut().unwrap().as_mut().unwrap().image_url = event_target_value(&ev));
                        }
                        prop:value={get_post_from_res(post_resource).map(|post| post.image_url)}/>
                    </label>
                    <label class="block mb-4">
                    <span class="text-gray-700">Title</span>
                    <input class="mt-1 p-2 w-full border rounded-md" type="text" id="title" name="title"
                        on:input=move |ev| {
                            post_resource.update(|curr| curr.as_mut().unwrap().as_mut().unwrap().title = event_target_value(&ev));
                        }
                        prop:value={get_post_from_res(post_resource).map(|post| post.title)}/>
                    </label>
                    <label class="block mb-4">
                    <span class="text-gray-700">Entry</span>
                    <textarea class="mt-1 p-2 w-full border rounded-md" id="text" name="text"
                        on:input=move |ev| {
                            // tricky - we update the actual post we got from `match` instead of getting it from the resource
                            post_resource.update(|curr| curr.as_mut().unwrap().as_mut().unwrap().text = event_target_value(&ev));
                        }
                >
                    {post_resource.and_then(|post| post.text.clone())}
                </textarea>
                </label>
                <input type="submit" value="Submit"/>
            </ActionForm>
                </div>
                // right side preview
                <div>
                    {move || post_resource.and_then( |post| view! {<BlogPost post=post.clone()/>})}
                </div>
                </div>
            </ErrorBoundary>
        </Transition>
    }
}

fn get_post_from_res<S: Clone>(
    post_resource: Resource<S, Result<Post, ServerFnError>>,
) -> Option<Post> {
    post_resource.get().map(|res| res.ok()).flatten()
}
