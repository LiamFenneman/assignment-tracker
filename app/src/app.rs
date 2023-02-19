use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use tracker_core::*;

pub use crate::components::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Title text="Assignment Tracker" />
        <Router>
            <Routes>
                <Route path="" view=move |cx| view! { cx, <HomePage /> } />
            </Routes>
        </Router>
    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    // TODO: get this from database
    let mut course = Course::new("Example");
    course.assignments = vec![
        Assignment::new("Assignment 1").mark(100).weight(25),
        Assignment::new("Assignment 2").mark(75).weight(25),
        Assignment::new("Assignment 3").weight(25),
        Assignment::new("Exam").weight(25),
    ];

    view! {
        cx,
        <main class="my-0 mx-auto max-w-3xl">
            // TOOD: display table for each course
            <CourseTable course />
        </main>
    }
}
