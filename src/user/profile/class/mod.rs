use leptos::*;
use server_fn::ServerFnError;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct UserCourses {
            title: String,
            price: f32,
            language: String,
            instructor: String,
            rating: i32,
            level: String,
            requirement: String,
            duration_minutes: i32,
            about: String,
            description: String,
            tag_line: String,
            update_date: String,
        }
    }
}

#[server]
pub async fn get_user_courses(user: String) -> Result<(), ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state;
    match use_context::<AppState>() {
        Some(s) => state = s,
        None => return Err(ServerFnError::Args("ERROR<user/profile/class/mod.rs>: during application state retrieval".to_string())),
    }

    //  取得数据库信息
    let pool = state.pool;

    logging::log!("DEBUG: <user/profile/class/mod.rs:41> getting {}'s courses", &user);

    /*---   提取用户数据    ---*/
    let user_courses;

    match sqlx::query_as::<_, UserCourses>(
        "SELECT c.* FROM student_course sc INNER JOIN courses c ON sc.course_id = c.course_id WHERE sc.username = $1;",
    )
    .bind(&user)
    .fetch_one(&pool)
    .await {
        Ok(uc) => user_courses = uc,
        Err(e) => {
            logging::log!("ERROR<user/profile/class/mod.rs:52>: {}", e.to_string());
            return Err(ServerFnError::Args(e.to_string()))
        },
    }

    logging::log!("DEBUG: <user/profile/class/mod.rs:54> {:?}", user_courses);
    Ok(())
}
#[component]
pub fn ClassPage(user: ReadSignal<String>) -> impl IntoView {

    // let mut result = Ok(());

    // our resource
    let async_data = create_resource(
        move || user.get(),
        // every time `count` changes, this will run
        move |value| async move {
            logging::log!("DEBUG<user/profile/class/mod.rs:57>: {:?}", &value);
            get_user_courses(user.get()).await
        },
    );

    logging::log!("ERROR<user/profile/class/mod.rs:68> {:?}", async_data);
    view!{
        <h1> Classes: </h1>
    }
}
