use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::component::edit_post::EditPost;
use crate::component::blog_previews::BlogPreviews;
use crate::component::toast::Toast;
use crate::component::view_post::ViewPost;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="dark:bg-gray-800 text-white p-4">
            <div class="container mx-auto flex justify-between items-center">
                // title on the left
                <a href="/" class="text-2xl font-bold">The Roaming Crab</a>

                // nav bar
                <nav>
                    <ul class="flex space-x-4">
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
        <Title text="The Roaming Crab"/>
        <Toast/>

        <Navbar/>
        // content for this welcome page

        <Router>
            <main class="dark:bg-gray-700 dark:text-gray-200 p-8 h-screen">
                <Routes>
                    <Route path="" view=BlogPreviews/>
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
