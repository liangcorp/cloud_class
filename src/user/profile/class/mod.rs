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
pub async fn get_user_courses(user: String) -> Result<Vec<String>, ServerFnError> {
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
    .fetch_all(&pool)
    .await {
        Ok(uc) => user_courses = uc,
        Err(e) => {
            logging::log!("ERROR<user/profile/class/mod.rs:52>: {}", e.to_string());
            return Err(ServerFnError::Args(e.to_string()))
        },
    }

    logging::log!("DEBUG: <user/profile/class/mod.rs:58> {:?}", user_courses);
    let unpacked_courses = user_courses
        .iter()
        .map(|uc| format!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                uc.title,
                uc.price,
                uc.language,
                uc.instructor,
                uc.rating,
                uc.level,
                uc.requirement,
                uc.duration_minutes,
                uc.about,
                uc.description,
                uc.tag_line,
                uc.update_date))
        .collect();

    Ok(unpacked_courses)
}
#[component]
pub fn ClassPage(user: String) -> impl IntoView {

    let (content, set_content) = create_signal(Vec::new());

    if user != "".to_string() {
        logging::log!("spawning get {} courses", &user);
        spawn_local(
            async move {
                match get_user_courses(user.clone()).await {
                    Ok(data) => {
                        set_content.set(data)
                    },
                    Err(_) => set_content.set(vec!["".to_string()]),
                }
           }
        )
    }
    view!{
        <h1> Classes: </h1>

        <p> "show: " {content} </p>
    }
}
