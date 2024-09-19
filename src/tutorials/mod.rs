pub mod editor;
pub mod output;
// pub mod console;

use leptos::*;
use leptos_router::*;
use server_fn::ServerFnError;
use cfg_if::cfg_if;

#[derive(Params, PartialEq)]
struct CourseParams {
    id: Option<String>
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct UserChapterQuery {
            username: String,
            course_id: String
        }

        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct CourseTitleQuery {
            title: String,
        }
    }
}

#[server]
pub async fn is_subscribed(user: String, course_id: String) -> Result<bool, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(false),
    };

    //  取得数据库信息
    let pool = state.pool;

    // let user_courses: (String, String);

    match sqlx::query_as::<_, UserChapterQuery>(
        "SELECT DISTINCT username, course_id
        FROM student_course
        WHERE username = $1 AND course_id = $2"
    )
    .bind(&user)
    .bind(&course_id)
    .fetch_one(&pool)
    .await {
        Ok(_) => Ok(true),
        Err(_) => Ok(false), // Course not found
    }
}

#[server]
pub async fn get_user_course_title(user: String, course_id: String) -> Result<Option<String>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(None),
    };

    //  取得数据库信息
    let pool = state.pool;

    // let user_courses: (String, String);

    match sqlx::query_as::<_, CourseTitleQuery>(
        "SELECT c.*
        FROM student_course sc
        INNER JOIN courses c ON sc.course_id = c.course_id
        WHERE sc.username = $1 AND c.course_id = $2;"
    )
    .bind(&user)
    .bind(&course_id)
    .fetch_one(&pool)
    .await {
        Ok(t) => Ok(Some(t.title)),
        Err(e) => {
            logging::log!("ERROR <tutorials/mod.rs:88>: {}", e.to_string());
            return Ok(None)
        },
    }
}

#[component]
pub fn TutorialPage() -> impl IntoView {
    use leptos_router::Redirect;
    use crate::session::extract_session_user;

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=extract_session_user

            // the data is bound to whatever variable name you provide
            let:session_user
        >
            {match session_user {
                Ok(ok_username) => {
                    match ok_username {
                        Some(some_username) => {
                            view! {
                                <div class="tutorial">
                                    <TutorialContentGate username=some_username.to_string() />
                                </div>
                            }
                                .into_view()
                        }
                        None => view! { <Redirect path="/profile" /> },
                    }
                }
                Err(_) => view! { <Redirect path="/profile" /> },
            }}
        </Await>
    }
}

#[component]
fn TutorialContentGate(username: String) -> impl IntoView {
    let params = use_params_map();

    // id: || -> Option<String>
    let course_id = move || params.with_untracked(|params| params.get("course_id").cloned());

    let (display_tutorial, set_display_tutorial) = create_signal(true);

    let username_clone = username.clone();
    let course_id_clone = course_id().unwrap().clone();

    view! {
        {
            spawn_local(async move {
                match is_subscribed(username_clone, course_id_clone).await {
                    Ok(result_bool) => set_display_tutorial.set(result_bool),
                    Err(_) => set_display_tutorial.set(true),
                }
            });

        }

        <div class:display=move || display_tutorial.get()>
            <div style="margin-left:40%">
                <h2>
                    <p style="color:red">"您不能访问这节课程的实验室"</p>
                </h2>
            </div>
        </div>
        <div class:display=move || !display_tutorial.get()>
            <TutorialContent username=username course_id=course_id().unwrap() />
        </div>
    }
}

#[component]
fn TutorialContent(username: String, course_id: String) -> impl IntoView {
    use editor::TutorialEditorArea;
    use output::TutorialOutputArea;
    // use console::TutorialConsoleArea;

    let _ = course_id;
    let (code, set_code) = create_signal("".to_string());
    let (value, set_value) = create_signal(1_u32);

    let (course_title, set_course_title) = create_signal("".to_string());

    let username_clone = username.clone();
    let course_id_clone = course_id.clone();

    view! {
        {
            spawn_local(async move {
                match get_user_course_title(username_clone, course_id_clone).await {
                    Ok(title) => match title {
                        Some(t) => set_course_title.set(t),
                        None => set_course_title.set("".to_string()),
                    }
                    Err(_) => set_course_title.set("".to_string()),
                }
            });
        }

        <div style="float:left; font-weight:bold; padding-top:10px">
            <table>
                <tr>
                    <td style="padding-right: 50px">
                        "用户: "{username}
                    </td>
                    <td style="padding-right: 50px">
                        "课程: "{course_title}
                    </td>
                    <td style="padding-right: 50px">
                        "章节: "<select
                            on:change=move |ev| {
                                let new_value = event_target_value(&ev);
                                set_value.set(new_value.parse().unwrap());
                            }
                            prop:value=move || value.get().to_string()
                        >
                            <option value="1">"1"</option>
                            <option value="2">"2"</option>
                        </select>
                    </td>
                </tr>
            </table>
        </div>
        <div>
            <TutorialEditorArea code=code set_code=set_code />
            <TutorialOutputArea code=code />
        </div>
    }
}
