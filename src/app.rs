use crate::admin::AdminPage;
use crate::courses::{content::ContentPageGate, CoursesPage, NoCoursePage};
use crate::error_template::{AppError, ErrorTemplate};
use crate::home::{about::AboutPage, HomePage};
use crate::student::account::{login::LoginPage, logout::LogoutPage, register::RegistrationPage};
use crate::student::profile::ProfilePageGate;
use crate::tutorials::TutorialPageGate;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/cloud-class.css" />
        <Meta charset="UTF-8" />

        // sets the document title
        <Title text="云学院" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }.into_view()
        }>
            <main>
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/about" view=AboutPage />
                    <Route path="/login" view=LoginPage />
                    <Route path="/logout" view=LogoutPage />
                    <Route path="/register" view=RegistrationPage />
                    <Route path="/admin" view=AdminPage />
                    <Route path="/profile" view=ProfilePageGate />
                    <Route path="/tutorials/:course_id" view=TutorialPageGate />
                    <Route path="/courses" view=CoursesPage>
                        <Route path=":course_id" view=ContentPageGate />
                        <Route path="" view=NoCoursePage />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
