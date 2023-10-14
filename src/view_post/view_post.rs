use leptos::*;
use leptos_meta::*;

use chrono::DateTime;
use chrono::Utc;

#[component]
pub fn BlogPost(date: ReadSignal<DateTime<Utc>>, title: ReadSignal<String>, text: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="block">
            <div class="text-xl">{date}</div>
            <div class="text-4xl">{title}</div>
            <div>{text}</div>
        </div>
    }
}
