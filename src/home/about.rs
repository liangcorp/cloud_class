use leptos::*;
#[component]
pub fn AboutPage() -> impl IntoView {
    use super::header::*;

    view! {
        <HeaderMenu />

        <div class="contents">
            <img class="about" src="images/banners/marvin-meyer-SYTO3xs06fU.jpg" />
            <h1>"学校简介"</h1>
        </div>
    }
}
