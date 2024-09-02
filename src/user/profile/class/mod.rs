use leptos::*;
use server_fn::ServerFnError;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct User {
            username: String,
            salt: String,
            pw_hash: String,
        }
    }
}

#[server(Login, "/api")]
pub async fn user_auth(user: String, password: String, remember_user: String) -> Result<(), ServerFnError> {
    use crate::state::AppState;
    use crate::session::cookie::Cookie;
    use crate::session::cache::Cache;
    use crate::utils::{ crypto::*, uuid::* };

    //  取得软件状态
    let state;
    match use_context::<AppState>() {
        Some(s) => state = s,
        None => return Err(ServerFnError::Args("ERROR<user/profile/class/mod.rs>: during application state retrieval".to_string())),
    }

    //  取得数据库信息
    let pool = state.pool;

    /*---   提取用户数据    ---*/
    let account = sqlx::query_as::<_, User>(
        "SELECT * FROM student_accounts WHERE username==$1;",
    )
    .bind(&user)
    .fetch_one(&pool)
    .await?;

    /*---   Salt Hash 用户输入密码    ---*/
    let parsed_hash = get_parsed_hash(&password, account.salt.as_str())?;
    /*---   认证密码一致    ---*/
    // if Argon2::default().verify_password(&b_password, &parsed_hash).is_ok() {
    if parsed_hash == account.pw_hash {
        let session_token = get_session_token();

        if remember_user == "true" {
            Cookie::set_cookie(&session_token, true)?;
        } else {
            Cookie::set_cookie(&session_token, false)?;
        }
        Cache::set_cache(&session_token, &account.username)?;

        //  改变网址到学生资料
        leptos_axum::redirect("/profile");
    } else {
        return Err(ServerFnError::Args("failed".to_string()));
    }

    Ok(())
}
#[component]
pub fn ClassPage(username: ReadSignal<String>) -> impl IntoView {

    view!{
        <h1> { move || username.get() } Classes: </h1>
    }
}
