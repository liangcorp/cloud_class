use leptos::*;
use leptos_router::*;


#[derive(Params, PartialEq)]
struct CourseParams {
    id: Option<String>
}

#[component]
pub fn Courses() -> impl IntoView {
    view! {
        <Redirect path="/profile" />
    }
}

#[component]
pub fn CoursePage() -> impl IntoView {

    let params = use_params_map();
    // id: || -> Option<String>
    let id = move || {
        params.with(|params| params.get("id").cloned())
    };

    view! {
        <a href="/courses">回到个人资料</a>
        <p> { id } </p>
    }
}
