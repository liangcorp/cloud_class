use leptos::*;
use server_fn::ServerFnError;
use serde::{Serialize, Deserialize};
use cfg_if::cfg_if;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PersonalContent {
    username: String,
    first_name: String,
    last_name: String,
    start_date: String,
    address: String,
    email: String,
    mobile: String,
}

impl Default for PersonalContent {
    fn default() -> PersonalContent {
        PersonalContent {
            username: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            start_date: "".to_string(),
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
            first_name: String,
            last_name: String,
            start_date: String,
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
        first_name: personal_content.first_name.clone(),
        last_name: personal_content.last_name.clone(),
        start_date: personal_content.start_date.clone(),
        address: personal_content.address.clone(),
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
        <div class="contents">
            <h1>个人资料</h1>
            <p>
            Name: { move || content.get().first_name}" "{move || content.get().last_name }
            </p>
        </div>
    }
}
