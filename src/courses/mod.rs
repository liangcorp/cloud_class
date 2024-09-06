use leptos::*;
use leptos_router::*;


#[derive(Params, PartialEq)]
struct CourseParams {
    id: Option<String>
}

#[component]
pub fn CoursePage() -> impl IntoView {

    // let params = use_params::<CourseParams>();
    //
    // // id: || -> String
    // let id = move || {
    //     params.with(|params| {
    //         params.as_ref()
    //             .map(|params| params.id.clone())
    //             .unwrap_or_default()
    //     })
    // };

    let params = use_params_map();
    // id: || -> Option<String>
    let id = move || {
        params.with(|params| params.get("id").cloned())
    };

    view! {
        <p> { id } </p>
    }
}
