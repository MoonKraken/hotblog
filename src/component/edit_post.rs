use super::blog_post::BlogPost;
use super::errors_fallback::error_fallback;
use super::toast::ToastMessage;
use super::toast::ToastType;
use chrono::Duration;
use chrono::DurationRound;
use chrono::Local;
use chrono::NaiveDateTime;
use leptos::logging::log;
use leptos::*;
use leptos_router::*;

use crate::model::blog_post::Post;
use crate::repository::blog_repository::get_post;
use crate::repository::blog_repository::DeletePost;
use crate::repository::blog_repository::UpsertPost;
use serde::{Deserialize, Serialize};

#[derive(Params, Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
struct EditPostParams {
    post_id: Option<String>,
}

fn format_dt(datetime: NaiveDateTime) -> String {
    datetime.format("%Y-%m-%dT%H:%M").to_string()
}

#[component]
pub fn EditPost() -> impl IntoView {
    let params: Memo<Result<_, _>> = use_params::<EditPostParams>();
    let post_resource: Resource<_, Result<Post, ServerFnError>> = create_resource(
        move || params.get(),
        |params| async move {
            match params {
                Ok(EditPostParams { post_id: Some(s) }) => get_post(s).await,
                Ok(EditPostParams { post_id: None }) => Ok(Post::new_empty()), // if no id is in the URL path parameter, assume we are making a new post
                _ => Err(ServerFnError::Args("issue getting params".to_string())),
            }
        },
    );

    let upsert_post = create_server_action::<UpsertPost>();
    let delete_post = create_server_action::<DeletePost>();

    let set_toast: WriteSignal<ToastMessage> = expect_context();
    // take them to the new or updated post once they create or edit it
    create_effect(move |_| {
        let id = upsert_post.value().get();
        if let Some(Ok(id)) = id {
            set_toast.set(ToastMessage {
                message: String::from("Post submitted."),
                toast_type: ToastType::Success,
                visible: true,
            });
            let navigate = use_navigate();
            navigate(format!("/view/{}", id).as_str(), Default::default());
        }
    });

    // take them to the home page if they delete a post
    create_effect(move |_| {
        let id = delete_post.value().get();
        if let Some(Ok(_)) = id {
            log!("set toast set");
            set_toast.set(ToastMessage {
                message: String::from("Post deleted."),
                toast_type: ToastType::Success,
                visible: true,
            });

            let navigate = use_navigate();
            navigate("/", Default::default());
        }
    });

    view! {
        <Transition fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback={error_fallback()}>
                <div class="flex h-screen">
                <div class="min-w-[50%] max-h-[90%] text-gray-200 dark:bg-gray-800 bg-gray-100 p-10 rounded-md">
                <ActionForm action=upsert_post>
                    <input type="hidden" name="id" prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.id).ok())}/>
                    <label class="block mb-4">
                        <span>Date</span>
                        <input class="mt-1 p-2 w-full" type="datetime-local" id="datetime" name="dt"
                            on:input=move |ev| {
                                let dt: String = event_target_value(&ev);
                                let chrono_dt = NaiveDateTime::parse_from_str(&dt, "%Y-%m-%dT%H:%M");
                                let utc_dt = match chrono_dt {
                                    Ok(dt) => dt,
                                    _ => Local::now().naive_local()
                                };
                                post_resource.update(|curr| {
                                    if let Some(Ok(post)) = curr {
                                        post.dt = utc_dt;
                                    }
                                });
                            }
                            prop:value={move || {
                                post_resource
                                    .get()
                                    .and_then(|res| res.map(|post| format_dt(post.dt)).ok())
                            }}
                    />
                    </label>
                    <label class="block mb-4">
                    <span>Image URL</span>
                    <input class="mt-1 p-2 w-full" type="text" id="image_url" name="image_url"
                        on:input=move |ev| {
                            post_resource.update(|curr| {
                                if let Some(Ok(post)) = curr {
                                   post.image_url = event_target_value(&ev);
                                }
                            });
                        }
                        prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.image_url).ok())}/>
                    </label>
                    <label class="block mb-4">
                    <span>Title</span>
                    <input class="mt-1 p-2 w-full" type="text" id="title" name="title"
                        on:input=move |ev| {
                            post_resource.update(|curr| {
                                if let Some(Ok(post)) = curr {
                                   post.title = event_target_value(&ev);
                                }
                            });
                        }
                            prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.title).ok())}/>
                    </label>
                    <label class="block mb-4">
                    <span>Entry</span>
                    <textarea class="mt-1 p-2 w-full" id="text" name="text"
                        on:input=move |ev| {
                            post_resource.update(|curr| {
                                if let Some(Ok(post)) = curr {
                                   post.text = event_target_value(&ev);
                                }
                            });
                        }
                        prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.text).ok())}
                    />
                    </label>
                <div class="flex justify-center pb-4">
                    <input type="submit" value="Submit" class="mx-auto w-1/3 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded cursor-pointer"/>
                </div>
                </ActionForm>
                <ActionForm action=delete_post>
                    <input type="hidden" name="id"
                        prop:value={move || post_resource.get().and_then(|res| res.map(|post| post.id).ok())}/>
                <div class="flex justify-center pb-4">
                    <input type="submit" value="Delete Post" class="mx-auto w-1/3 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded cursor-pointer"/>
                </div>
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
