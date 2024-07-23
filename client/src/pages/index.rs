use leptos::{component, IntoView, view};
use log::debug;
use web_sys::{self, js_sys};


pub fn t(){
    debug!("Hello, Worl!d!");
    let func = js_sys::Function::new_no_args("window.location.href = '/movie/1';");
    let x = web_sys::window().unwrap().document().unwrap().start_view_transition_with_update_callback(Some(&func)).unwrap();
    debug!("{:?}", x);
    // let navigate = leptos_router::use_navigate();
    // navigate("/movie/1", Default::default());
}

#[component]
pub fn Index() -> impl IntoView{

    view! {
        <div class="h-full">
            <h1>"Hello, World!"</h1>
            <div
                class="flex justify-center items-center bg-[red] w-[200px] h-[200px] box"
            >box</div>
            <button on:click=move |_| {t()}>click me</button>
        </div>
    }
}