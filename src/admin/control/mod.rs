use leptos::*;

#[component]
pub fn ControlPagePortal() -> impl IntoView {
    view! {
        <HeaderSection />

        <div class="contents">
            "you got it!"
        </div>
    }
}

/// Renders the header menu of control panel.
#[component]
pub fn HeaderSection() -> impl IntoView {
    view! {
        <div class="contents">
            <table>
                <tr>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "课程管理"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "学员管理"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "教师管理"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "管理员中心"
                        </a>
                    </td>
                    <td class="header-menu"></td>
                    <td class="header-menu"></td>
                    <td class="header-menu"></td>
                    <td class="header-menu"></td>

                    <LoginLogoutSection />
                </tr>
            </table>
        </div>
        <div style="padding-bottom:30px">
            <hr class="page-divider" />
        </div>
    }
}

#[component]
fn LoginLogoutSection() -> impl IntoView {
    use crate::session::*;
    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=extract_session_user

            // the data is bound to whatever variable name you provide
            let:session_user
        >
            {match session_user {
                Ok(ok_username) => {
                    match ok_username {
                        Some(some_username) => {
                            view! {
                                <td class="header-login">
                                    <a class="header" href="#">
                                        {some_username}
                                    </a>
                                </td>
                                <td class="header-login">
                                    <a href="/logout" class="home-login">
                                        "退出"
                                    </a>
                                </td>
                            }
                                .into_view()
                        }
                        None => {
                            view! {
                            }
                                .into_view()
                        }
                    }
                }
                Err(_) => {
                    view! {
                    }
                        .into_view()
                }
            }}
        </Await>
    }
}
