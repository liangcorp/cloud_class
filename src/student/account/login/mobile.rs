use leptos::*;

#[component]
pub fn MobileLoginLayer() -> impl IntoView {
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