use leptos::*;

/// Render the collaboration page
#[component]
pub fn CollaborationPage() -> impl IntoView {
    use super::header::HeaderSection;

    view! {
        <HeaderSection />

        <div class="contents">
            <table>
                <tr>
                    <td class="content">
                        <h1>"赋予您的人才权力，推动您的业务向前发展"</h1>
                        <ul>
                            <li>"与受人尊敬的行业专家和顶尖大学一起培训团队"</li>
                            <li>"使用量身定制的路径和 AI 工具丰富学习解决方案"</li>
                            <li>"使用全球公认的证书提高员工参与度"</li>
                            <li>"高效定制可扩展的学习解决方案"</li>
                        </ul>
                        <br />
                        <button class="contact">"联系我们"</button>
                    </td>
                    <td class="content">
                        <img class="content" src="images/banners/asian-school-photo.jpeg" />
                    </td>
                </tr>
            </table>
        </div>
    }
}
