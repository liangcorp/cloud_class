pub mod control;
mod login;

use leptos::*;
use leptos_meta::Title;
use leptos_router::*;

#[component]
pub fn AdminPage() -> impl IntoView {
    view! { <Outlet /> }
}

#[component]
pub fn AdminLoginPage() -> impl IntoView {
    use login::LoginPanel;

    view! {
        <Title text="数智化教学辅助系统" />

        <div align="center" style="margin-top:100px">
            <LoginPanel />
        </div>
    }
}
