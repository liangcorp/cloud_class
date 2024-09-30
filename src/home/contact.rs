
use leptos::*;

/// Render the collaboration page
#[component]
pub fn ContactPage() -> impl IntoView {
    use super::header::HeaderSection;

    view! {
        <HeaderSection />

        <div class="contents">
            <table>
                <tr>
                    <td class="content">
                        <h1>"准备好带领您的团队应对市场变化了吗？"</h1>
                        <h3>"让我们一起讨论如何为您提供帮助:"</h3>
                        <ul>
                            <li>"加速数字化转型"</li>
                            <li>"提高公司的敏捷性"</li>
                            <li>"提高员工工作效率和创新能力"</li>
                            <li>"让员工有能力推动增长"</li>
                        </ul>
                    </td>
                    <td class="content" style="background-color:white">
                        <form>
                            <table style="padding-top:30px;padding-left:50px;border-spacing:5px;">
                                <tr>
                                    <td>"全名:"</td>
                                    <td>
                                        <input class="login-form" type="text" placeholder="请输入全名" />
                                    </td>
                                </tr>
                                <tr>
                                    <td>"邮件:"</td>
                                    <td>
                                        <input class="login-form" type="text" placeholder="请输入邮件" />
                                    </td>
                                </tr>
                                <tr>
                                    <td>"电话号码:"</td>
                                    <td>
                                        <input class="login-form" type="text" placeholder="请输入电话号码" />
                                    </td>
                                </tr>
                                <tr>
                                    <td colspan="2" style="padding-top:10px">
                                        <input class="submit-button" type="button" value="提交"/>
                                    </td>
                                </tr>
                            </table>
                        </form>
                    </td>
                </tr>
            </table>
        </div>
    }
}
