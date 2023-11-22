use crate::errors_fallback::error_fallback;
use crate::toast::ToastType;
use leptos::logging::log;
use leptos::*;
use leptos_router::*;

use crate::model::post::Post;
use crate::repository::post::get_post;
use crate::repository::post::DeletePost;
use crate::repository::post::UpsertPost;
use crate::toast::ToastMessage;
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
    let delete_post = create_server_action::<DeletePost>();

    // take them to the new or updated post once they create or edit it
    create_effect(move |_| {
        let id = upsert_post.value().get();
        if let Some(Ok(id)) = id {
            let navigate = use_navigate();
            navigate(format!("/view/{}", id).as_str(), Default::default());
        }
    });

    // take them to the home page if they delete a post
    create_effect(move |_| {
        let id = delete_post.value().get();
        log!("delete create effect");
        if let Some(Ok(_)) = id {
            let set_toast: WriteSignal<ToastMessage> =
                use_context().expect("couldn't get toast context");
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
                <div class="min-w-[50%] bg-gray-100 p-10">
                <ActionForm action=upsert_post>
                    <input type="hidden" name="id"
                        prop:value={move || get_post_from_res(post_resource).map(|post| post.id)}/>
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
                        prop:value={move || get_post_from_res(post_resource).map(|post| format_rfc3339_without_timezone(post.dt))}
                    />
                    </label>
                    <label class="block mb-4">
                    <span class="text-gray-700">Image URL</span>
                    <input class="mt-1 p-2 w-full border rounded-md" type="text" id="image_url" name="image_url"
                        on:input=move |ev| {
                            post_resource.update(|curr| curr.as_mut().unwrap().as_mut().unwrap().image_url = event_target_value(&ev));
                        }
                        prop:value={move || get_post_from_res(post_resource).map(|post| post.image_url)}/>
                    </label>
                    <label class="block mb-4">
                    <span class="text-gray-700">Title</span>
                    <input class="mt-1 p-2 w-full border rounded-md" type="text" id="title" name="title"
                        on:input=move |ev| {
                            post_resource.update(|curr| curr.as_mut().unwrap().as_mut().unwrap().title = event_target_value(&ev));
                        }
                        prop:value={move || get_post_from_res(post_resource).map(|post| post.title)}/>
                    </label>
                    <label class="block mb-4">
                    <span class="text-gray-700">Entry</span>
                    <textarea class="mt-1 p-2 w-full border rounded-md" id="text" name="text"
                        on:input=move |ev| {
                            // tricky - we update the actual post we got from `match` instead of getting it from the resource
                            post_resource.update(|curr| curr.as_mut().unwrap().as_mut().unwrap().text = event_target_value(&ev));
                        }
                >
                    {move || post_resource.and_then(|post| post.text.clone())}
                </textarea>
                </label>
                <input type="submit" value="Submit" class="mx-auto w-1/3 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded cursor-pointer"/>
                </ActionForm>
                <ActionForm action=delete_post>
                    <input type="hidden" name="id" prop:value={move || get_post_from_res(post_resource).map(|post| post.id)}/>
                    <input type="submit" value="Delete Post" class="mx-auto w-1/3 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded cursor-pointer"/>
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
