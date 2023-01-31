use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::pages::*;
use crate::components::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tracker_app.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <NavBar />
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/assignments" view=|cx| view! { cx, <p>"Assignments"</p> }/>
                    <Route path="/courses" view=|cx| view! { cx, <p>"Courses"</p> }/>
                    <Route path="/profile" view=|cx| view! { cx, <p>"Profile"</p> }/>
                </Routes>
            </main>
        </Router>
    }
}
