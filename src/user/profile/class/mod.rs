use leptos::*;
use server_fn::ServerFnError;
use serde::{Serialize, Deserialize};
use cfg_if::cfg_if;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CourseContent {
    course_id: String,
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

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct UserCourses {
            course_id: String,
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
pub async fn get_user_courses(user: String) -> Result<Vec<CourseContent>, ServerFnError> {
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
    let courses_contents = user_courses
        .iter()
        .map(|uc| CourseContent {
                course_id: uc.course_id.clone(),
                title: uc.title.clone(),
                price: uc.price.clone(),
                language: uc.language.clone(),
                instructor: uc.instructor.clone(),
                rating: uc.rating.clone(),
                level: uc.level.clone(),
                requirement: uc.requirement.clone(),
                duration_minutes: uc.duration_minutes.clone(),
                about: uc.about.clone(),
                description: uc.description.clone(),
                tag_line: uc.tag_line.clone(),
                update_date: uc.update_date.clone()})
        .collect();

    Ok(courses_contents)
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
                    Err(_) => set_content.set(Vec::new()),
                }
           }
        )
    }

    view!{
        <ul>
            <For
                each=move || content.get()
                key=|state| (state.course_id.clone())
                let:child
            >
                <p>{child.title}</p>
            </For>
        </ul>
    }
}
