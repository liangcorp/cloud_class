use leptos::*;
use server_fn::ServerFnError;
use serde::{Serialize, Deserialize};
use cfg_if::cfg_if;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalContent {
    username: String,
    full_name: String,
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
            full_name: "".to_string(),
            start_date: "".to_string(),
            status: "".to_string(),
            address: "".to_string(),
            email: "".to_string(),
            mobile: "".to_string()
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct PersonalContentQuery {
            username: String,
            full_name: String,
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
    let state;
    match use_context::<AppState>() {
        Some(s) => state = s,
        None => return Ok(PersonalContent::default()),
    }

    //  取得数据库信息
    let pool = state.pool;

    /*---   提取用户数据    ---*/
    let personal_content;

    match sqlx::query_as::<_, PersonalContentQuery>(
        "SELECT * FROM students WHERE username = $1;",
    )
    .bind(&user)
    .fetch_one(&pool)
    .await {
        Ok(pc) => personal_content = pc,
        Err(e) => {
            return Err(ServerFnError::Args(e.to_string()))
        },
    }

    let result_content = PersonalContent {
        username: personal_content.username.clone(),
        full_name: personal_content.full_name.clone(),
        start_date: personal_content.start_date.clone(),
        status: personal_content.status.clone(),
        address: personal_content.address.clone(),
        email: personal_content.email.clone(),
        mobile: personal_content.mobile.clone(),
    };

    Ok(result_content)
}
#[component]
pub fn PersonalPage(user: Option<String>) -> impl IntoView {
    let (content, set_content) = create_signal(PersonalContent::default());

    if user != None {
        spawn_local(
            async move {
                match get_personal_profile(user.unwrap().clone()).await {
                    Ok(data) => {
                        set_content.set(data)
                    },
                    Err(e) => {
                        set_content.set(PersonalContent::default());
                        logging::log!("ERROR<user/profile/info/mod.rs>: {}", e.to_string());
                    },
                }
           }
        )
    }

    view! {
        <div class="profile_contents">
            <p style="color:gray; font-weight:bold;">学生</p>
            <table style="width:100%">
                <tr>
                    <td>
                        <h1>{move || content.get().full_name}</h1>
                        <table>
                            <tr>
                                <td>
                                    <b>"注册日:"</b>
                                </td>
                                <td>{move || content.get().start_date}</td>
                            </tr>
                            <tr>
                                <td>
                                    <b>"邮件地址:"</b>
                                </td>
                                <td>{move || content.get().email}</td>
                            </tr>
                            <tr>
                                <td>
                                    <b>"手机号:"</b>
                                </td>
                                <td>{move || content.get().mobile}</td>
                            </tr>
                            <tr>
                                <td>
                                    <b>"地址:"</b>
                                </td>
                                <td>{move || content.get().address}</td>
                            </tr>
                        </table>
                    </td>
                    <td>
                        <img
                            src="images/users/default_profile.png"
                            style="width:250px;height:250px"
                        />
                    </td>
                </tr>
            </table>
        </div>
    }
}
