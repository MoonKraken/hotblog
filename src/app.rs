use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::edit_post::edit_post::EditPost;
use crate::blog_previews::blog_previews::BlogPreviews;
use crate::about::about::About;
use crate::view_post::view_post::ViewPost;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="bg-stone-800 text-white p-4">
            <div class="container mx-auto flex justify-between items-center">
                // title on the left
                <a href="/" class="text-2xl font-bold">Blog</a>

                // nav bar
                <nav>
                    <ul class="flex space-x-4">
                        <li><a href="/about" class="hover:text-blue-400">About</a></li>
                        <li><a href="/" class="hover:text-blue-400">Blog</a></li>
                        <li><a href="/edit" class="hover:text-blue-400">Create</a></li>
                    </ul>
                </nav>
            </div>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        <Navbar/>
        // content for this welcome page

        <Router>
            <main>
                <Routes>
                    <Route path="" view=BlogPreviews/>
                    <Route path="/about" view=About/>
                    <Route path="/edit/:post_id?" view=EditPost/>
                    <Route path="/view/:post_id?" view=ViewPost/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
