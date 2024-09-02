pub mod class;
pub mod personal;

use leptos::*;
use leptos_router::Redirect;
use crate::user::profile::class::ClassPage;
use crate::user::profile::personal::PersonalPage;

// use serde::Deserialize;
/// Renders the profile page of your application.
#[component]
pub fn ProfilePage() -> impl IntoView {
    use crate::session::*;

    let (username, set_username) = create_signal("".to_string());
    let (login_button, set_login_button) = create_signal("inline".to_string());
    let (logout_button, set_logout_button) = create_signal("none".to_string());

    let (display_class, set_display_class) = create_signal("inline".to_string());
    let (display_personal, set_display_personal) = create_signal("none".to_string());

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=extract_session_user

            // the data is bound to whatever variable name you provide
            let:session_user
        >
            <p>
                {match session_user {
                    Ok(s) => {
                        if s == "" {
                            set_login_button.set("inline".to_string());
                            set_logout_button.set("none".to_string());
                            //  用户没有登陆回到主页
                            let _ = view!{
                                <Redirect path="/" />
                            };
                        } else {
                            set_login_button.set("none".to_string());
                            set_logout_button.set("inline".to_string());
                        }
                        set_username.set((*s).clone());
                    }
                    Err(_) => {
                        set_username.set("".to_string());
                    }
                }}
            </p>
        </Await>

        <div class="contents">
            <div class="header">
                <table class="header-menu">
                    <tr>
                        <td class="header">
                            <img src="images/logo.png" />
                        </td>
                        <td class="header_menu">
                            <a href="/" class="header">
                                首页
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="/profile" class="header" on:click=move |_| {
                                set_display_class.set("inline".to_string());
                                set_display_personal.set("none".to_string());
                            }>
                                我的课程
                            </a>
                        </td>
                        <td class="header_menu">
                            <a href="/profile" class="header" on:click=move |_| {
                                set_display_class.set("none".to_string());
                                set_display_personal.set("inline".to_string());
                            }>
                                个人资料
                            </a>
                        </td>
                        <td class="header_menu">
                            <input class="course_search_box_profile" style="text" placeholder="搜索" />
                        </td>
                        <td class="header_login">
                            <a
                                href="/login"
                                class="home_login"
                                style:display=move || login_button.get()
                            >
                                登陆
                            </a>
                            <a class="header" href="/profile">
                                {move || username.get()}
                            </a>
                        </td>
                        <td class="header_login">
                            <a
                                href="/register"
                                class="header"
                                style:display=move || login_button.get()
                            >
                                注册
                            </a>
                            <a
                                href="/logout"
                                class="home_login"
                                style:display=move || logout_button.get()
                            >
                                退出
                            </a>
                        </td>
                    </tr>
                </table>
            </div>
        </div>
        <div>
            <hr class="page_divider"></hr>
        </div>

        <div style:display=move || display_class.get()>
            <ClassPage username=username/>
        </div>

        <div style:display=move || display_personal.get()>
            <PersonalPage username=username/>
        </div>
    }
}
