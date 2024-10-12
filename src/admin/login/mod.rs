use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::Title;
use server_fn::ServerFnError;

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
pub async fn admin_auth(user: String, password: String) -> Result<(), ServerFnError> {
    use crate::session::cache::Cache;
    use crate::session::cookie::Cookie;
    use crate::state::AppState;
    use crate::utils::{crypto, uuid};

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => {
            return Err(ServerFnError::Args(
                "ERROR<admin/login/mod.rs>: during application state retrieval".to_string(),
            ))
        }
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let account = sqlx::query_as::<_, User>("SELECT * FROM administrators WHERE username==$1;")
        .bind(&user)
        .fetch_one(&pool)
        .await?;

    //  Salt Hash 用户输入密码
    let parsed_hash = crypto::get_parsed_hash(&password, account.salt.as_str())?;
    //  认证密码一致
    if parsed_hash == account.pw_hash {
        let session_token = uuid::get_session_token();

        Cookie::set_cookie(&session_token, false)?;
        Cache::set_cache(&session_token, &account.username)?;

        //  改变网址到学生资料
        leptos_axum::redirect("/admin/control");
    } else {
        return Err(ServerFnError::Args("failed".to_string()));
    }

    Ok(())
}

#[component]
pub fn LoginPanel() -> impl IntoView {
    // 制作一个reactive值去更新提交按钮
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
    let (auth_success, set_auth_success) = create_signal("none");

    let input_username: NodeRef<html::Input> = create_node_ref();
    let input_password: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let username_value = input_username
            .get()
            .expect("<input> should be mounted")
            .value();

        let password_value = input_password
            .get()
            .expect("<input> should be mounted")
            .value();

        spawn_local(async move {
            match admin_auth(username_value.clone(), password_value.clone()).await {
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
        <Title text="数智化教学辅助系统" />

        <div align="center" style="margin-top:100px">
            <form on:submit=on_submit>
                <table class="admin-login">
                    <tr>
                        <td>
                            <h3>"数字化教辅平台-控制端"</h3>
                        </td>
                    </tr>
                    <tr style:display=move || auth_success.get() style="color:red">
                        <td>
                            <h4>"用户名或者密码不正确"</h4>
                        </td>
                    </tr>
                    <tr>
                        <td style="padding-bottom:10px">
                            "用户名:"<br />
                            <input
                                placeholder="请输入用户名"
                                class="login-form"
                                type="text"
                                value=username
                                node_ref=input_username
                            />
                        </td>
                    </tr>
                    <tr>
                        <td style="padding-bottom:10px">
                            "密码:"<br />
                            <input
                                placeholder="请输入密码"
                                class="login-form"
                                type="password"
                                value=password
                                node_ref=input_password
                            />
                        </td>
                    </tr>
                    <tr>
                        <td style="padding-bottom:10px">
                            <input
                                class="submit-button"
                                style="width:100%;padding-top:10px;padding-bottom:10px"
                                type="submit"
                                value="登陆"
                            />
                        </td>
                    </tr>
                </table>
            </form>
        </div>
    }
}
