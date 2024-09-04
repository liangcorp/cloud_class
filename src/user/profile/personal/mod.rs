use leptos::*;
use server_fn::ServerFnError;
use serde::{Serialize, Deserialize};
use cfg_if::cfg_if;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalContent {
    username: String,
    full_name: String,
    start_date: String,
    address: String,
    role: String,
    email: String,
    mobile: String,
}

impl Default for PersonalContent {
    fn default() -> PersonalContent {
        PersonalContent {
            username: "".to_string(),
            full_name: "".to_string(),
            start_date: "".to_string(),
            address: "".to_string(),
            role: "".to_string(),
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
            address: String,
            role: String,
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
        None => return Err(ServerFnError::Args("ERROR<user/profile/personal/mod.rs>: during application state retrieval".to_string())),
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
        address: personal_content.address.clone(),
        role: personal_content.role.clone(),
        email: personal_content.address.clone(),
        mobile: personal_content.address.clone(),
    };

    Ok(result_content)
}
#[component]
pub fn PersonalPage(user: String) -> impl IntoView {
    let (content, set_content) = create_signal(PersonalContent::default());

    if user != "".to_string() {
        spawn_local(
            async move {
                match get_personal_profile(user.clone()).await {
                    Ok(data) => {
                        set_content.set(data)
                    },
                    Err(_) => set_content.set(PersonalContent::default()),
                }
           }
        )
    }

    view! {
        <div class="profile_contents">
            <div style="display:inline-block;width:60%">
                <p style="color:gray; weight:bold;"> { move || content.get().role} </p>
                <h1>{ move || content.get().full_name}</h1>
                <p>
                    <b>"注册日:"</b>{ move || content.get().start_date}
                </p>
                <p>
                    <b>"邮件地址:"</b>{ move || content.get().email}
                </p>
                <p>
                    <b>"手机号:"</b>{ move || content.get().mobile}
                </p>
                <p>
                    <b>"地址:"</b>{ move || content.get().address}
                </p>
            </div>
            <div style="display:inline-block;width:40%">
                <img src="images/users/default_profile.png" style="width:250px;height:250px"/>
            </div>
        </div>
    }
}
