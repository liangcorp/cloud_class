use crate::admin::AdminPage;
use crate::error_template::{AppError, ErrorTemplate};
use crate::home::HomePage;
use crate::student::account::login::LoginPage;
use crate::student::account::logout::LogoutPage;
use crate::student::account::register::RegistrationPage;
use crate::student::profile::ProfilePage;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/cloud-class.css" />

        // sets the document title
        <Title text="浩天云学院" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/login" view=LoginPage />
                    <Route path="/logout" view=LogoutPage />
                    <Route path="/register" view=RegistrationPage />
                    <Route path="/admin" view=AdminPage />
                    <Route path="/profile" view=ProfilePage />
                </Routes>
            </main>
        </Router>
    }
}
