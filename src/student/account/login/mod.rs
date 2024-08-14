use leptos::*;
use leptos_meta::*;

#[component]
fn LoginLayer() -> impl IntoView {
    // 制作一个reactive值去更新提交按钮
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

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
        set_username.set(username_value);

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
        set_password.set(password_value);
    };

    view! {
            <form on:submit=on_submit> // on_submit defined below
                <table>
                    // <tr><td><p>"用户名是: " {username}</p></td></tr>
                    <tr style="display:none;color:red">
                    <td>
                    <h4>用户名或者密码不正确</h4>
                    </td>
                    </tr>
                    <tr>
                    <td><b class="login_box">
                        账号<input placeholder="请输入账号" class="login_form" type="text"
                            value=username
                            node_ref=input_username
                        />

                    </b></td>
                    </tr>
                    <tr><td></td></tr>
                    <tr>
                    <td><b class="login_box">
                        密码<input placeholder="请输入密码" class="login_form" type="password"
                            value=password
                            node_ref=input_password
                            />
                    </b></td>
                    </tr>
                </table>

                <table>
                    <tr>
                        <td style="padding: 10px">
                            <input type="checkbox"/>记住账号
                        </td>
                        <td style="padding: 10px">
                            忘记密码
                        </td>
                    </tr>
                </table>

                <table>
                    <tr>
                    <td style="padding:10px">
                        <input class="submit_button" type="submit" value="登陆"/>
                    </td>
                    <td style="padding:10px">
                        <input class="register_button" type="submit" value="注册"/>
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

#[component]
fn RegisterLayer() -> impl IntoView {
    view! {}
}

/// 提供登陆页
#[component]
pub fn LoginPage() -> impl IntoView {
    view! {

        // sets the document title
        <Title text="浩天数智化教学"/>

        <div class="full-height">
        <div class="login_div" align="center">
            <table>
            <tr>
                <td style="padding: 20px">
                <hr width="350px" size="1" color="#BFBFBF" noshade />
                </td>
                <td>
                <img src="images/logo1.png"/>
                </td>
                <td style="padding: 20px">
                <hr width="350px" size="1" color="#BFBFBF" noshade />
                </td>
            </tr>

            <tr>
                <td>
                    <div style="padding:20px">
                        <a href="#" class="login_switch">密码登录</a>
                        <a href="#" class="login_switch">短信登录</a>
                    </div>

                    <div>
                    <LoginLayer/>
                    </div>

                    <div>
                    <RegisterLayer/>
                    </div>
                </td>
                <td></td>
                <td align="center">
                    <QRLayer/>
                </td>
            </tr>
            </table>
        </div>
        </div>
    }
}
