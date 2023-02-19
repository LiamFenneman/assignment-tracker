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
    let course = create_tmp_course().unwrap();

    view! {
        cx,
        <main class="my-0 mx-auto max-w-3xl">
            // TOOD: display table for each course
            <CourseTable course />
        </main>
    }
}

fn create_tmp_course() -> anyhow::Result<Course> {
    let mut course = Course::new("Example");
    course.assignments.push_back(Assignment::new("Assignment 1"))?;
    course.assignments.push_back(Assignment::new("Assignment 2"))?;
    course.assignments.push_back(Assignment::new("Assignment 3"))?;
    course.assignments.push_back(Assignment::new("Exam"))?;

    course.assignments.get_mut(0).unwrap().set_mark(100)?;
    course.assignments.get_mut(0).unwrap().set_weight(25)?;
    course.assignments.get_mut(1).unwrap().set_mark(75)?;
    course.assignments.get_mut(1).unwrap().set_weight(25)?;
    course.assignments.get_mut(2).unwrap().set_mark(50)?;
    course.assignments.get_mut(2).unwrap().set_weight(25)?;
    course.assignments.get_mut(3).unwrap().set_weight(25)?;

    return Ok(course);
}
