use leptos::*;
use leptos_meta::*;

#[component]
pub fn InstructorPage() -> impl IntoView {

    view! {
        <Title text="教师中心" />

        <div class="contents">
            <div>
                <a class="header" href="/">
                    "回到主页"
                </a>
            </div>
        </div>
    }
}
