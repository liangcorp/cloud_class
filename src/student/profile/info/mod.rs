use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalContent {
    username: String,
    fullname: String,
    start_date: String,
    status: String,
    address: String,
    email: String,
    mobile: String,
}

impl Default for PersonalContent {
    fn default() -> PersonalContent {
        PersonalContent {
            username: "".to_string(),
            fullname: "".to_string(),
            start_date: "".to_string(),
            status: "".to_string(),
            address: "".to_string(),
            email: "".to_string(),
            mobile: "".to_string(),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct PersonalContentQuery {
            username: String,
            fullname: String,
            start_date: String,
            status: String,
            address: String,
            email: String,
            mobile: String,
        }
    }
}

#[server]
pub async fn get_personal_profile(user: String) -> Result<PersonalContent, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(PersonalContent::default()),
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let personal_content = match sqlx::query_as::<_, PersonalContentQuery>(
        "SELECT * FROM students WHERE username = $1;",
    )
    .bind(&user)
    .fetch_one(&pool)
    .await
    {
        Ok(ok_personal_content) => PersonalContent {
            username: ok_personal_content.username.clone(),
            fullname: ok_personal_content
                .fullname
                .chars()
                .map(|c| if c == '_' { ' ' } else { c })
                .collect::<String>(),
            start_date: ok_personal_content.start_date.clone(),
            status: ok_personal_content.status.clone(),
            address: ok_personal_content.address.clone(),
            email: ok_personal_content.email.clone(),
            mobile: ok_personal_content.mobile.clone(),
        },
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    };

    Ok(personal_content)
}

#[component]
pub fn PersonalContentPage(user: String) -> impl IntoView {
    view! {
        <Await future=move || get_personal_profile(user.clone()) let:data>
            {
                let content = match data.as_ref() {
                    Ok(d) => (*d).clone(),
                    Err(_) => PersonalContent::default(),
                };

                view! {
                    <PersonalContentPanel personal_content=content />
                }
            }
        </Await>
    }
}

#[component]
pub fn PersonalContentPanel(personal_content: PersonalContent) -> impl IntoView {
    view! {
        <table style="width:100%">
            <tr>
                <td>
                    <h1>{personal_content.fullname}</h1>
                    <table>
                        <tr>
                            <td>
                                <b>"注册日:"</b>
                            </td>
                            <td>{personal_content.start_date}</td>
                        </tr>
                        <tr>
                            <td>
                                <b>"邮件地址:"</b>
                            </td>
                            <td>{personal_content.email}</td>
                        </tr>
                        <tr>
                            <td>
                                <b>"手机号:"</b>
                            </td>
                            <td>{personal_content.mobile}</td>
                        </tr>
                        <tr>
                            <td>
                                <b>"地址:"</b>
                            </td>
                            <td>{personal_content.address}</td>
                        </tr>
                    </table>
                </td>
                <td>
                    <img src="images/users/default_profile.png" style="width:250px;height:250px" />
                </td>
            </tr>
        </table>
    }
}
