use leptos::{component, IntoView, view};

#[component]
pub fn Index() -> impl IntoView{
    view! {
        <div class="h-full">
            <h1>"Hello, World!"</h1>
        </div>
    }
}