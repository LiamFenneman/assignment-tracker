use leptos::*;

#[component]
pub fn NavBar(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <nav class="w-full flex justify-evenly justify-items-stretch items-stretch h-20">
            <a href="/">"Home"</a>
            <a href="/assignments">"Assignments"</a>
            <a href="/courses">"Courses"</a>
            <a href="/profile">"Profile"</a>
        </nav>
    }
}
