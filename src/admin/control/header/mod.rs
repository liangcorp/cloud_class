use leptos::prelude::*;

/// Renders the header menu of control panel
#[component]
pub fn HeaderSection(username: String) -> impl IntoView {
    view! {
        <div class="contents">
            <table>
                <tr>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "课程管理"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "教材管理"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "练习管理"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "赞助商管理"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "学员管理"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="/admin/control/instructors" class="header">
                            "教师管理"
                        </a>
                    </td>
                    <td class="header-menu">
                        <a href="#" class="header">
                            "管理员中心"
                        </a>
                    </td>
                    <td class="header-menu"></td>
                    <td class="header-login">
                        <b>{username}</b>
                    </td>
                    <td class="header-login">
                        <a href="/logout" class="home-login">
                            "退出"
                        </a>
                    </td>
                </tr>
            </table>
        </div>
        <div style="padding-bottom:30px">
            <hr class="page-divider" />
        </div>
    }
}
