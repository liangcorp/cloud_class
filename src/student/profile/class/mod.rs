use cfg_if::cfg_if;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

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
    update_date: String,
    image_id: String,
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
            update_date: "".to_string(),
            image_id: "class_default.png".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CourseInstructor {
    username: String,
    fullname: String,
}

impl Default for CourseInstructor {
    fn default() -> CourseInstructor {
        CourseInstructor {
            username: "".to_string(),
            fullname: "".to_string(),
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
            image_id: String,
        }

        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct CourseInstructorQuery {
            username: String,
            fullname: String,
        }
    }
}

#[server]
pub async fn get_user_courses(user: String) -> Result<Vec<CourseContent>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(vec![CourseContent::default()]),
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let user_courses = match sqlx::query_as::<_, CourseContentQuery>(
        "SELECT c.*
        FROM student_course sc
        INNER JOIN courses c ON sc.course_id = c.course_id
        WHERE sc.username = $1
        ORDER BY sc.priority;",
    )
    .bind(&user)
    .fetch_all(&pool)
    .await
    {
        Ok(ok_user_courses) => ok_user_courses
            .iter()
            .map(|ok_user_courses| CourseContent {
                course_id: ok_user_courses.course_id.clone(),
                title: ok_user_courses.title.clone(),
                price: ok_user_courses.price,
                course_language: ok_user_courses.course_language.clone(),
                rating: ok_user_courses.rating,
                target_level: ok_user_courses.target_level.clone(),
                requirement: ok_user_courses.requirement.clone(),
                duration_minutes: ok_user_courses.duration_minutes,
                about: ok_user_courses.about.clone(),
                description: ok_user_courses.description.clone(),
                tag_line: ok_user_courses.tag_line.clone(),
                update_date: ok_user_courses.update_date.clone(),
                image_id: ok_user_courses.image_id.clone(),
            })
            .collect(),
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    };

    Ok(user_courses)
}

#[server]
pub async fn get_instructor(course_id: String) -> Result<Vec<CourseInstructor>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(vec![CourseInstructor::default()]),
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let course_instructors = match sqlx::query_as::<_, CourseInstructorQuery>(
        "SELECT username, fullname
        FROM course_instructor
        WHERE course_id = $1
        ORDER BY priority;",
    )
    .bind(&course_id)
    .fetch_all(&pool)
    .await
    {
        Ok(ok_courses_instructor) => ok_courses_instructor
            .iter()
            .map(|ok_courses_instructor| CourseInstructor {
                username: ok_courses_instructor.username.clone(),
                fullname: ok_courses_instructor.fullname.clone(),
            })
            .collect(),
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    };

    Ok(course_instructors)
}

#[component]
pub fn CourseContentPage(user: String) -> impl IntoView {
    view! {
        <Await future=get_user_courses(user.clone()) let:data>
            {
                let course_contents = match data.as_ref() {
                    Ok(d) => (*d).clone(),
                    Err(_) => Vec::new(),
                };
                view! { <CourseContentPanel course_contents=course_contents /> }
            }
        </Await>
    }
}

/// Rendring Course Contents
#[component]
fn CourseContentPanel(course_contents: Vec<CourseContent>) -> impl IntoView {
    view! {
        <For each=move || { course_contents.clone() } key=|_| () let:single_content>
            <div class="each-class">
                <a
                    href=format!("/courses/{}", &single_content.course_id)
                    style="text-decoration-line: none;color: #333333;"
                >
                    <div style="display: inline-block; width:40%">
                        <img
                            src=format!("images/courses/{}", single_content.image_id)
                            style="width:350px;height:250px"
                        />
                    </div>
                    <div style="display: inline-block; width:60%">
                        <table style="width:100%">
                            <tr>
                                <td style="align:left">
                                    <h3>{single_content.title}</h3>
                                </td>
                                <td style="padding-left:300px;align:right">
                                    <b>"¥" {single_content.price}" (CNY)"</b>
                                </td>
                            </tr>
                            <tr>
                                <td style="align:left">
                                    <p>{single_content.tag_line}</p>
                                </td>
                                <td style="align:right"></td>
                            </tr>
                            <tr>
                                <td style="align:left;color:gray;">
                                    "教师: "
                                    <InstructorsNamePanel course_id=single_content
                                        .course_id
                                        .clone() />
                                </td>
                                <td style="align:right"></td>
                            </tr>
                            <tr>
                                <td style="align:left">"面对: "{single_content.target_level}</td>
                                <td style="align:right"></td>
                            </tr>
                            <tr>
                                <td style="align:left">"语言: "{single_content.course_language}</td>
                                <td style="align:right"></td>
                            </tr>
                            <tr>
                                <td style="align:left">
                                    <span>
                                        {(0..single_content.rating)
                                            .map(|_| view! { <span style="color:red;">"★"</span> })
                                            .collect_view()}
                                    </span>
                                    <span>
                                        {(single_content.rating..10)
                                            .map(|_| view! { <span style="color:gray;">"★"</span> })
                                            .collect_view()}
                                    </span>
                                </td>
                                <td style="align:right"></td>
                            </tr>
                            <tr>
                                <td style="align:left">
                                    "时间: "{single_content.duration_minutes} "分钟"
                                </td>
                                <td style="align:right"></td>
                            </tr>
                            <tr>
                                <td style="align:left;color:#1e6055;">
                                    "更新日: "
                                    <b>{single_content.update_date}</b>
                                </td>
                                <td style="align:right"></td>
                            </tr>
                        </table>
                    </div>
                </a>
            </div>
            <div style="display:inline-block; padding-top:10px;padding-bottom:10px;margin-left:80%;">
                <a
                    target="_blank"
                    rel="noopener noreferrer"
                    href=format!("/tutorials/{}", &single_content.course_id)
                    class="tutorial-link"
                >
                    "⚒ 实验室"
                </a>
            </div>
            <hr />
        </For>
    }
}

/// Render instructors names and append comma if necesary
#[component]
fn InstructorsNamePanel(course_id: String) -> impl IntoView {
    let (instructors, set_instructors) = signal(Vec::new());

    spawn_local(async move {
        match get_instructor(course_id).await {
            Ok(data) => set_instructors.set(data),
            Err(_) => set_instructors.set(Vec::new()),
        };
    });

    view! {
        <For each=move || instructors.get() key=|_| () let:single_instructor>
            {single_instructor.fullname.clone()}
            {
                let empty_vec = CourseInstructor::default();
                let i_get = move || instructors.get();
                if single_instructor != *i_get().last().unwrap_or(&empty_vec) {
                    view! { ", " }
                } else {
                    view! { "" }
                }
            }
        </For>
    }
}
