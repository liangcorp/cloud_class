use leptos::*;
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
                        <h1>"学校简介"</h1>
                        <br />
                        <a href="/contact" class="contact">
                            "联系我们"
                        </a>
                    </td>
                </tr>
            </table>
        </div>
    }
}
