use leptos::*;
use leptos::prelude::*;

#[component]
pub fn MobileLoginLayer() -> impl IntoView {
    // 制作一个reactive值去更新提交按钮
    let (mobile_no, set_mobile_no) = signal("".to_string());
    let (sms, set_sms) = signal("".to_string());

    let input_mobile_no: NodeRef<html::Input> = NodeRef::new();
    let input_sms: NodeRef<html::Input> = NodeRef::new();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let mobile_no_value = input_mobile_no
            .get()
            .expect("<input> should be mounted")
            .value();
        set_mobile_no.set(mobile_no_value);

        let sms_value = input_sms.get().expect("<input> should be mounted").value();
        set_sms.set(sms_value);
    };

    view! {
        <form on:submit=on_submit>
            <table class="login-form">
                <tr style="display:none;color:red">
                    <td>
                        <h4>手机号或验证码不正确</h4>
                    </td>
                </tr>
                <tr>
                    <td colspan="2" style="padding-left:10px">
                        <input
                            placeholder="请输入手机号"
                            class="login-form"
                            style="width:100%"
                            type="text"
                            value=mobile_no
                            node_ref=input_mobile_no
                        />
                    </td>
                </tr>
                <tr>
                    <td style="padding-left:10px;padding-right:10px">
                        <input
                            placeholder="验证密码"
                            class="login-form"
                            style="width:90%"
                            type="text"
                            value=sms
                            node_ref=input_sms
                        />
                    </td>
                    <td>
                        <button class="mobile_verify">获取验证码</button>
                    </td>
                </tr>
                <tr>
                    <td colspan="2" style="padding-left:10px">
                        <input class="submit-button" type="submit" value="登陆" />
                    </td>
                </tr>
                <tr>
                    <td colspan="2" style="padding-bottom:65px"></td>
                </tr>
            </table>
        </form>
    }
}
