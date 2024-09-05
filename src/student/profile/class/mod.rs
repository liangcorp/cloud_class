use leptos::*;
use server_fn::ServerFnError;
use serde::{Serialize, Deserialize};
use cfg_if::cfg_if;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CourseContent {
    course_id: String,
    title: String,
    price: f32,
    course_language: String,
    rating: i32,
    target_level: String,
    requirement: String,
    duration_minutes: i32,
    about: String,
    description: String,
    tag_line: String,
    update_date: String
}

impl Default for CourseContent {
    fn default() -> CourseContent {
        CourseContent {
            course_id: "".to_string(),
            title: "".to_string(),
            price: 0.0,
            course_language: "".to_string(),
            rating: 0,
            target_level: "".to_string(),
            requirement: "".to_string(),
            duration_minutes: 0,
            about: "".to_string(),
            description: "".to_string(),
            tag_line: "".to_string(),
            update_date: "".to_string()
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct CourseContentQuery {
            course_id: String,
            title: String,
            price: f32,
            course_language: String,
            rating: i32,
            target_level: String,
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
        None => return Ok(vec![CourseContent::default()]),
    }

    //  取得数据库信息
    let pool = state.pool;

    /*---   提取用户数据    ---*/
    let user_courses;

    match sqlx::query_as::<_, CourseContentQuery>(
        "SELECT c.* FROM student_course sc INNER JOIN courses c ON sc.course_id = c.course_id WHERE sc.username = $1 ORDER BY sc.priority;",
    )
    .bind(&user)
    .fetch_all(&pool)
    .await {
        Ok(uc) => user_courses = uc,
        Err(e) => {
            return Err(ServerFnError::Args(e.to_string()))
        },
    }

    let result_content = user_courses
        .iter()
        .map(|uc| CourseContent {
                course_id: uc.course_id.clone(),
                title: uc.title.clone(),
                price: uc.price.clone(),
                course_language: uc.course_language.clone(),
                rating: uc.rating.clone(),
                target_level: uc.target_level.clone(),
                requirement: uc.requirement.clone(),
                duration_minutes: uc.duration_minutes.clone(),
                about: uc.about.clone(),
                description: uc.description.clone(),
                tag_line: uc.tag_line.clone(),
                update_date: uc.update_date.clone()})
        .collect();

    Ok(result_content)
}

#[component]
pub fn ClassPage(user: String) -> impl IntoView {

    let (content, set_content) = create_signal(Vec::new());

    if user != "".to_string() {
        spawn_local(
            async move {
                match get_user_courses(user.clone()).await {
                    Ok(data) => {
                        set_content.set(data)
                    },
                    Err(e) => {
                        set_content.set(Vec::new());
                        logging::log!("{}", e.to_string());
                    },
                }
           }
        )
    }

    view! {
        <div class="contents">
            <For
                each=move || content.get()
                key=|state| (state.course_id.clone())
                let:course_content
            >
                <a href="/course" style="text-decoration-line: none;color: #333333;">
                    <div class="each_class">
                        <div style="display: inline-block; width:40%">
                            <img
                                src="images/classes/class_default.png"
                                style="width:350px;height:250px"
                            />
                        </div>
                        <div style="display: inline-block; width:60%">
                            <table width="100%">
                                <tr>
                                    <td align="left">
                                        <h3>{course_content.title}</h3>
                                    </td>
                                    <td stype="padding-left:300px" align="right">
                                        <b>"¥" {course_content.price}" (CNY)"</b>
                                    </td>
                                </tr>
                                <tr>
                                    <td align="left">
                                        <p>{course_content.tag_line}</p>
                                    </td>
                                    <td align="right"></td>
                                </tr>
                                <tr>
                                    <td align="left" style="color:gray;">
                                        "教师: "
                                        // {course_content.instructor}
                                    </td>
                                    <td align="right"></td>
                                </tr>
                                <tr>
                                    <td align="left">"面对: "{course_content.target_level}</td>
                                    <td align="right"></td>
                                </tr>
                                <tr>
                                    <td align="left">"语言: "{course_content.course_language}</td>
                                    <td align="right"></td>
                                </tr>
                                <tr>
                                    <td align="left">
                                        <span>
                                            {(0..course_content.rating)
                                                .into_iter()
                                                .map(|_| view! { <span style="color:red;">"★"</span> })
                                                .collect_view()}
                                        </span>
                                        <span>
                                            {(course_content.rating..10)
                                                .into_iter()
                                                .map(|_| view! { <span style="color:gray;">"★"</span> })
                                                .collect_view()}
                                        </span>
                                    </td>
                                    <td align="right"></td>
                                </tr>
                                <tr>
                                    <td align="left">
                                        "时间: "{course_content.duration_minutes} "分钟"
                                    </td>
                                    <td align="right"></td>
                                </tr>
                                <tr>
                                    <td align="left" style="color:#1e6055;">
                                        "更新日: "
                                        <b>{course_content.update_date}</b>
                                    </td>
                                    <td align="right"></td>
                                </tr>
                            </table>
                        </div>
                    </div>
                </a>
                <hr />
            </For>
        </div>
    }
}
