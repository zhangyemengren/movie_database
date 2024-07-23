use leptos::{component, IntoView, view};

#[component]
pub fn MovieDetail() -> impl IntoView{
    view! {
        <div class="h-full">
            <h1>Movie Detail</h1>
            <div
                class="flex justify-center items-center bg-[red] w-[200px] h-[200px] scale-50 rotate-45 box"
            >box</div>
            <p>123</p>
        </div>
    }
}