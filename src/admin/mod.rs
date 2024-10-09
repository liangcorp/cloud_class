mod login;

use leptos::*;
use leptos_meta::*;

/// 提供登陆页
#[component]
pub fn AdminPage() -> impl IntoView {

    view! {
        <Title text="数智化教学辅助系统" />

        <div>
            <AdminPage />
            <ControlPanel />
        </div>
    }
}

#[component]
pub fn ControlPanel() -> impl IntoView {
    view! {
    }
}
