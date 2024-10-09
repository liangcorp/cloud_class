mod login;

use leptos::*;
use leptos_meta::*;

/// 提供登陆页
#[component]
pub fn AdminPage() -> impl IntoView {
    use login::LoginPanel;

    view! {
        <Title text="数智化教学辅助系统" />

        <div align="center" style="margin-top:100px">
            <LoginPanel />
            <ControlPanel />
        </div>
    }
}

#[component]
pub fn ControlPanel() -> impl IntoView {
    view! {}
}
