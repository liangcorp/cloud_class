use leptos::*;
use leptos::prelude::*;
use leptos_meta::Title;

/// Render the about page
#[component]
pub fn AboutPage() -> impl IntoView {
    use crate::header::HeaderSection;

    view! {
        <Title text="学校简介" />

        <HeaderSection />

        <div class="contents">
            <table>
                <tr>
                    <td class="content">
                        <img class="content" src="images/banners/marvin-meyer-SYTO3xs06fU.jpg" />
                    </td>
                    <td class="content">
                        <h1>"技能是释放潜力的关键"</h1>
                        <p>
                            "无论您是想学习新技能、培训团队还是与世界分享您的知识，您都来对地方了。作为在线学习领域的领导者，我们随时准备帮助您实现目标并改变您的生活。"
                        </p>
                        <a href="/contact" class="contact">
                            "联系我们"
                        </a>
                    </td>
                </tr>
            </table>
        </div>
    }
}
