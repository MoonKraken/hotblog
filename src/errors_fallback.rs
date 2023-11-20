use leptos::*;
pub fn error_fallback() -> Box<dyn Fn(RwSignal<Errors>) -> View> {
    Box::new(|errors: RwSignal<Errors>| {
        view! {
            <div class="bg-red-100 border-l-4 border-red-500 text-red-700 p-4 rounded">
                <ul>
                {
                    move || {errors.with(|errors| {
                        errors.iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    })}
                }
                </ul>
            </div>
        }.into_view()
    })
}
