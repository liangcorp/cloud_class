use crate::admin::{
    control::{instructors::AdminInstructorPortal, ControlPanel, ControlPanelPortal},
    AdminLoginPage, AdminPage, AdminRedirectPage,
};
use crate::courses::{content::ContentPagePortal, CoursesPage, NoCoursePage};
// use crate::error_template::{AppError, ErrorTemplate};
use crate::home::{
    about::AboutPage, collaboration::CollaborationPage, contact::ContactPage,
    instructor_list::InstructorListPage, HomePage,
};
use crate::student::{
    account::{login::LoginPage, logout::LogoutPage, register::RegistrationPage},
    profile::ProfilePagePortal,
};
use crate::tutorials::TutorialPagePortal;

use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

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
        <Router>
            <main>
                <Routes>
                    <Route path=StaticSegment("") view=HomePage />
                    <Route path="/about" view=AboutPage />
                    <Route path="/collaboration" view=CollaborationPage />
                    <Route path="/contact" view=ContactPage />
                    <Route path="/login" view=LoginPage />
                    <Route path="/logout" view=LogoutPage />
                    <Route path="/register" view=RegistrationPage />
                    <Route path="/instructor" view=InstructorListPage />
                    <Route path="/profile" view=ProfilePagePortal />
                    <Route path="/tutorials/:course_id" view=TutorialPagePortal />
                    <Route path="/courses" view=CoursesPage>
                        <Route path=":course_id" view=ContentPagePortal />
                        <Route path="" view=NoCoursePage />
                    </Route>
                    <Route path="/admin" view=AdminPage>
                        <Route path="/control" view=ControlPanel>
                            <Route path="/instructors" view=AdminInstructorPortal />
                            <Route path="" view=ControlPanelPortal />
                        </Route>

                        <Route path="/login" view=AdminLoginPage />
                        <Route path="" view=AdminRedirectPage />
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}
