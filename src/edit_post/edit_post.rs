use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use chrono::DateTime;
use chrono::Utc;
use crate::view_post::view_post::BlogPost;
#[derive(Params, Eq, PartialEq, Debug, Clone)]
struct EditPostParams {
    post_id: Option<String>
}

#[component]
pub fn EditPost() -> impl IntoView{
    let (title, set_title) = create_signal::<String>("".to_string());
    let (text, set_text) = create_signal::<String>("".to_string());
    let (date, set_date) = create_signal::<DateTime<Utc>>(Utc::now());
    let params: Memo<Result<_,_>> = use_params::<EditPostParams>();
    // let display_id = match params.get() {
    //     Ok(EditPostParams { post_id: Some(s)}) => s,
    //     _ => "".to_string()
    // };

    view! {
        <div class="flex h-screen">
            // left side input
            <div class="w-1/2 bg-gray-100 p-10">
                <label class="block mb-4">
                    <span class="text-gray-700">Date</span>
                    <input class="mt-1 p-2 w-full border rounded-md" type="datetime-local" id="datetime"
                        on:input=move |ev| {
                            let dt: String = event_target_value(&ev);
                            let chrono_dt = DateTime::parse_from_rfc3339(&dt);
                            let utc_dt = match chrono_dt {
                                Ok(dt) => dt.with_timezone(&Utc),
                                _ => Utc::now() // this means the browser gave us something wrong?
                            };
                            set_date(utc_dt);
                        }
                        prop:value=date
                    />
                </label>
                <label class="block mb-4">
                    <span class="text-gray-700">Title</span>
                    <input class="mt-1 p-2 w-full border rounded-md" type="text" id="title"
                        on:input=move |ev| {
                            set_title(event_target_value(&ev));
                        }
                        prop:value=title
                    />
                </label>
                <label class="block mb-4">
                    <span class="text-gray-700">Entry</span>
                    <textarea class="mt-1 p-2 w-full border rounded-md" id="text"
                        on:input=move |ev| {
                            set_text(event_target_value(&ev));
                        }
                    >
                        {text.get()}
                    </textarea>
                </label>
            </div>

            <BlogPost date title text/>
        </div>
   }
}
