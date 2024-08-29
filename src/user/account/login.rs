use leptos::*;
use leptos_meta::*;
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
pub async fn user_auth(user: String, password: String) -> Result<(), ServerFnError> {
    use crate::state::AppState;
    use crate::session::cookie::*;
    use crate::session::cache::*;
    use crate::utils::{ crypto::*,
                        uuid::*,
                        redis::* };

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
        "SELECT * FROM student_accounts WHERE username==$1;",
    )
    .bind(&user)
    .fetch_one(&pool)
    .await?;

    /*---   Salt Hash 用户输入密码    ---*/
    let parsed_hash = get_parsed_hash(password, account.salt.as_str())?;
    /*---   认证密码一致    ---*/
    // if Argon2::default().verify_password(&b_password, &parsed_hash).is_ok() {
    if parsed_hash == account.pw_hash {
        let session_id = get_session_id();

        Cookie::set_cookie(&session_id)?;

        CustomeCache::set_cache(session_id, account.username);
        // let redis = Redis::default();
        // match redis.fetch_an_integer() {
        //     Ok(result) => logging::log!("{}", result),
        //     Err(e) => logging::log!("ERROR<user/account/login.rs>: {}", e.to_string()),
        // }
        // let _ = CustomCache::new(user, session_id);
        //  改变网址到学生资料
        leptos_axum::redirect("/");
    } else {
        return Err(ServerFnError::Args("failed".to_string()));
    }

    Ok(())
}

#[component]
fn UsernameLoginLayer() -> impl IntoView {
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

        // if username_value != "user".to_string() || password_value != "password".to_string() {
        //     set_auth_success.set("inline");
        // } else {
        //     set_auth_success.set("none");
        //     set_username.set(username_value.clone());
        //     set_password.set(password_value.clone());
        // }

        // spawn_local(async {
        //     match user_auth(username_value, password_value).await {
        //         Ok(()) => set_auth_success.set("none"),
        //         Err(_e) => set_auth_success.set("inline"),
        //     };
        // });

        spawn_local(async move {
            match user_auth(username_value.clone(), password_value.clone()).await {
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
                // <tr><td><p>"用户名是: " {username}</p></td></tr>
                // <tr><td><p>"密码名是: " {password}</p></td></tr>
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
                        <input type="checkbox" />
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

#[component]
fn MobileLoginLayer() -> impl IntoView {
    // 制作一个reactive值去更新提交按钮
    let (mobile_no, set_mobile_no) = create_signal("".to_string());
    let (sms, set_sms) = create_signal("".to_string());

    let input_mobile_no: NodeRef<html::Input> = create_node_ref();
    let input_sms: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let mobile_no_value = input_mobile_no
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_mobile_no.set(mobile_no_value);

        let sms_value = input_sms
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_sms.set(sms_value);
    };

    view! {
        <form on:submit=on_submit>
            <table style="padding-left:10px">
                // <tr><td><p>"用户名是: " {username}</p></td></tr>
                // padding 用来装饰
                <tr>
                    <td style="padding-top:60px"></td>
                    <td></td>
                </tr>
                <tr style="display:none;color:red">
                    <td>
                        <h4>手机号或验证码不正确</h4>
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        <input
                            placeholder="请输入手机号"
                            style="width:94%"
                            class="login_form"
                            type="text"
                            value=mobile_no
                            node_ref=input_mobile_no
                        />
                    </td>
                </tr>

                <tr>
                    <td>
                        <input
                            placeholder="验证密码"
                            class="login_form"
                            type="text"
                            value=sms
                            node_ref=input_sms
                        />
                    </td>
                    <td>
                        <button>获取验证码</button>
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        <input
                            class="submit_button"
                            style="width:100%; padding-top:10px; padding-bottom:10px"
                            type="submit"
                            value="登陆"
                        />
                    </td>
                </tr>
            </table>
        </form>
    }
}

#[component]
fn QRLayer() -> impl IntoView {
    view! {
        <p>微信扫描二维码登陆</p>
        <img src="images/winxinlogo.png" />
        <img src="images/QR/showQrCode.png" />
    }
}

/// 提供登陆页
#[component]
pub fn LoginPage() -> impl IntoView {
    let (username_login, set_username_login) = create_signal("".to_string());
    let (mobile_login, set_mobile_login) = create_signal("none".to_string());

    view! {
        <Title text="浩天数智化教学" />

        <div class="full-height">
            <div class="login_div" align="center">
                <table>
                    <tr>
                        <td style="padding: 20px">
                            <hr width="350px" size="1" color="#BFBFBF" noshade />
                        </td>
                        <td>
                            <img src="images/logo1.png" />
                        </td>
                        <td style="padding: 20px">
                            <hr width="350px" size="1" color="#BFBFBF" noshade />
                        </td>
                    </tr>

                    <tr>
                        <td>
                            <div style="padding:20px;top:0px">
                                <table>
                                    <tr>
                                        <td>
                                            <a
                                                href="#"
                                                class="login_switch"
                                                on:click=move |_| {
                                                    set_username_login.update(|n| *n = String::from("inline"));
                                                    set_mobile_login.update(|n| *n = String::from("none"));
                                                }
                                            >
                                                密码登录
                                            </a>
                                        </td>
                                        <td>
                                            <a
                                                href="#"
                                                class="login_switch"
                                                on:click=move |_| {
                                                    set_username_login.update(|n| *n = String::from("none"));
                                                    set_mobile_login.update(|n| *n = String::from("inline"));
                                                }
                                            >
                                                短信登录
                                            </a>
                                        </td>
                                    </tr>
                                </table>
                            </div>

                            <div style:display=move || username_login.get()>
                                <UsernameLoginLayer />
                            </div>
                            <b>{move || username_login.get()}</b>

                            <div style:display=move || mobile_login.get()>
                                <MobileLoginLayer />
                            </div>
                        </td>
                        <td></td>
                        <td align="center">
                            <QRLayer />
                        </td>
                    </tr>
                </table>
                <br />
                <br />
                <br />
                <a href="/">返回主页</a>
            </div>
        </div>
    }
}
