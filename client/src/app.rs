use leptos::{component, IntoView, view};
use leptos_router::{Route, Router, Routes};
use crate::pages::{Index, MovieDetail};

#[component]
pub fn App() -> impl IntoView{
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Index/>
                <Route path="/movie/:id" view=MovieDetail/>
            </Routes>
        </Router>
    }
}