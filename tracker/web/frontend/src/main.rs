use yew::prelude::*;
use yew_router::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod components;
pub mod pages;

use components::*;
use pages::*;

#[derive(Clone, Routable, PartialEq, Eq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/tracker")]
    Tracker,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Tracker => html! {<Tracker />},
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}
