use crate::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(NavBar)]
pub fn navbar() -> Html {
    let classes = vec!["navbar"];
    html! {
        <nav class={classes!(classes)}>
            <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            <Link<Route> to={Route::Tracker}>{"Tracker"}</Link<Route>>
        </nav>
    }
}
