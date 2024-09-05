use leptos::*;
use server_fn::ServerFnError;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct User {
            username: String,
            salt: String,
            pw_hash: String,
        }
    }
}

#[server(Login, "/api")]
pub async fn user_auth(user: String, password: String, remember_user: String) -> Result<(), ServerFnError> {
    use crate::state::AppState;
    use crate::session::cookie::Cookie;
    use crate::session::cache::Cache;
    use crate::utils::{ crypto::*, uuid::* };

    //  取得软件状态
    let state;
    match use_context::<AppState>() {
        Some(s) => state = s,
        None => return Err(ServerFnError::Args("ERROR<user/account/login.rs>: during application state retrieval".to_string())),
    }

    //  取得数据库信息
    let pool = state.pool;

    /*---   提取用户数据    ---*/
    let account = sqlx::query_as::<_, User>(
        "SELECT * FROM students WHERE username==$1;",
    )
    .bind(&user)
    .fetch_one(&pool)
    .await?;

    /*---   Salt Hash 用户输入密码    ---*/
    let parsed_hash = get_parsed_hash(&password, account.salt.as_str())?;
    /*---   认证密码一致    ---*/
    // if Argon2::default().verify_password(&b_password, &parsed_hash).is_ok() {
    if parsed_hash == account.pw_hash {
        let session_token = get_session_token();

        if remember_user == "true" {
            Cookie::set_cookie(&session_token, true)?;
        } else {
            Cookie::set_cookie(&session_token, false)?;
        }
        Cache::set_cache(&session_token, &account.username)?;

        //  改变网址到学生资料
        leptos_axum::redirect("/profile");
    } else {
        return Err(ServerFnError::Args("failed".to_string()));
    }

    Ok(())
}

#[component]
pub fn UsernameLoginLayer() -> impl IntoView {
    // 制作一个reactive值去更新提交按钮
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (auth_success, set_auth_success) = create_signal("none");
    let (checkbox_value, set_checkbox_value) = create_signal("false");

    let input_username: NodeRef<html::Input> = create_node_ref();
    let input_password: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let username_value = input_username
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();

        let password_value = input_password
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();

        spawn_local(async move {
            match user_auth(username_value.clone(), password_value.clone(), checkbox_value.get_untracked().to_string()).await {
                Ok(()) => {
                    set_auth_success.set("none");
                    set_username.set(username_value);
                    set_password.set(password_value)
                }
                Err(_) => {
                    set_auth_success.set("inline");
                }
            }
        });
    };

    view! {
        <form on:submit=on_submit>
            <table>
                <tr style:display=move || auth_success.get() style="color:red">
                    <td>
                        <h4>用户名或者密码不正确</h4>
                    </td>
                </tr>
                <tr>
                    <td style="padding-left:10px">
                        <input
                            placeholder="请输入账号"
                            class="login_form"
                            style="width:100%"
                            type="text"
                            value=username
                            node_ref=input_username
                        />
                    </td>
                </tr>
                <tr>
                    <td></td>
                </tr>
                <tr>
                    <td style="padding-left:10px">
                        <input
                            placeholder="请输入密码"
                            class="login_form"
                            style="width:100%"
                            type="password"
                            value=password
                            node_ref=input_password
                        />
                    </td>
                </tr>
            </table>

            <table>
                <tr>
                    <td style="padding: 10px">
                        <input
                            type="checkbox"
                            on:input=move |ev| {
                                let is_checked = event_target_checked(&ev);
                                let new_value = if is_checked { "true" } else { "false" };
                                set_checkbox_value.set(new_value);
                            }
                        />
                        记住账号
                    </td>
                    <td style="padding: 10px">
                        <a href="#">忘记密码</a>
                    </td>
                </tr>
            </table>

            <table>
                <tr>
                    <td style="padding:10px">
                        <input class="submit_button" type="submit" value="登陆" />
                    </td>
                    <td style="padding:10px">
                        <a href="/register" class="login_switch">
                            注册
                        </a>
                    </td>
                </tr>
            </table>
        </form>
    }
}
