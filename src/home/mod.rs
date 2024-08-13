use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="contents">
            <div class="header">
                <table class="header-menu">
                <tr>
                    <td class="header">
                        <img src="images/logo.png"/>
                    </td>
                    <td class="header_menu">
                        <a href="/" class="header">首页</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">走进学校</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">课程中心</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">继续教育</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">师资力量</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">新闻中心</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">在线学习</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">就业招聘</a>
                    </td>

                    <td class="header_login"><a href="/login" class="header" style="padding-top:10px;padding-bottom:10px;padding-left:20px;padding-right:20px;color:#FAFAFA;background-color: #333333;">登陆</a></td>
                    <td class="header_login"><a href="#" class="header" >注册</a></td>
                </tr>
                </table>
            </div>
        </div>
        <div><img src="images/banners/3.财务会计banner.jpg" class="banner"/></div>
    }
}
