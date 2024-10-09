use leptos::*;

#[component]
pub fn LoginPage() -> impl IntoView {
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
            .expect("<input> should be mounted")
            .value();
        set_username.set(username_value);

        let password_value = input_password
            .get()
            .expect("<input> should be mounted")
            .value();
        set_password.set(password_value);
    };

    view! {
        <form on:submit=on_submit>
            <table class="login">
                // <tr><td><p>"用户名是: " {username}</p></td></tr>
                <tr>
                    <td style="padding:25px">
                        <h2 style="color:#000000">数智化教学辅助系统</h2>
                    </td>
                </tr>
                <tr style="display:none;color:red">
                    <td>
                        <h4>用户名或者密码不正确</h4>
                    </td>
                </tr>
                <tr>
                    <td>
                        <b style="padding:10px;border:0px;font-size:20px">
                            账号
                            <input
                                placeholder="请输入账号"
                                style="padding:10px;border:0px;font-size:20px"
                                type="text"
                                value=username
                                node_ref=input_username
                            />

                        </b>
                    </td>
                </tr>
                <tr>
                    <td></td>
                </tr>
                <tr>
                    <td style="padding:10px">
                        <b style="padding:10px;border:0px;font-size:20px">
                            密码
                            <input
                                placeholder="请输入密码"
                                style="padding:10px;border:0px;font-size:20px"
                                type="password"
                                value=password
                                node_ref=input_password
                            />
                        </b>
                    </td>
                </tr>
                <tr>
                    <td style="padding:10px">
                        <input
                            style="border:0px;background-color:#333333;color:white;font-size:20px"
                            type="submit"
                            value="登陆"
                        />
                    </td>
                </tr>
            </table>
        </form>
    }
}
