pub mod editor;
pub mod execution;
// pub mod console;

use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    chapter_id: String,
    title: String,
    chapter_number: u32,
    course_id: String,
}

impl Default for Chapter {
    fn default() -> Chapter {
        Chapter {
            chapter_id: "".to_string(),
            title: "".to_string(),
            chapter_number: 0,
            course_id: "".to_string(),
        }
    }
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

        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct CourseChapterQuery {
            chapter_id: String,
            title: String,
            chapter_number: u32,
            course_id: String
        }

        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct TutorialQuery {
            code_content: String
        }
    }
}

#[server]
pub async fn get_user_course_title(
    user: String,
    course_id: String,
) -> Result<Option<String>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(None),
    };

    //  取得数据库信息
    let pool = state.pool;

    match sqlx::query_as::<_, CourseTitleQuery>(
        "SELECT c.*
        FROM student_course sc
        INNER JOIN courses c ON sc.course_id = c.course_id
        WHERE sc.username = $1 AND c.course_id = $2;",
    )
    .bind(&user)
    .bind(&course_id)
    .fetch_one(&pool)
    .await
    {
        Ok(t) => Ok(Some(t.title)),
        Err(_) => Ok(None),
    }
}

#[server]
pub async fn get_course_chapters(course_id: String) -> Result<Vec<Chapter>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(some_state) => some_state,
        None => return Ok(vec![Chapter::default()]),
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let chapters = match sqlx::query_as::<_, CourseChapterQuery>(
        "SELECT *
        FROM chapters
        WHERE course_id = $1
        ORDER BY chapter_number;",
    )
    .bind(&course_id)
    .fetch_all(&pool)
    .await
    {
        Ok(ok_chapters) => ok_chapters
            .iter()
            .map(|cc| Chapter {
                chapter_id: cc.chapter_id.clone(),
                title: cc.title.clone(),
                chapter_number: cc.chapter_number,
                course_id: cc.course_id.clone(),
            })
            .collect(),
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    };

    Ok(chapters)
}

#[server]
pub async fn get_tutorial_chapter(
    course_id: String,
    chapter_number: u32,
) -> Result<Option<String>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(None),
    };

    //  取得数据库信息
    let pool = state.pool;

    match sqlx::query_as::<_, TutorialQuery>(
        "SELECT *
        FROM tutorials
        WHERE course_id = $1 AND chapter_number = $2;",
    )
    .bind(&course_id)
    .bind(chapter_number)
    .fetch_one(&pool)
    .await
    {
        Ok(code) => Ok(Some(code.code_content)),
        Err(_) => Ok(None),
    }
}

#[component]
pub fn TutorialPagePortal() -> impl IntoView {
    use crate::session::extract_session_user;
    use leptos_router::Redirect;

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
                                    <TutorialContentPortal username=some_username.to_string() />
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
fn TutorialContentPortal(username: String) -> impl IntoView {
    let params = use_params_map();

    // id: || -> Option<String>
    let course_id = move || params.with_untracked(|params| params.get("course_id").cloned());

    let (display_tutorial, set_display_tutorial) = create_signal(true);
    let (course_title, set_course_title) = create_signal("".to_string());

    let username_clone = username.clone();
    let course_id_clone = course_id().unwrap().clone();

    provide_context(course_title);

    view! {
        {
            spawn_local(async move {
                match get_user_course_title(username_clone, course_id_clone).await {
                    Ok(title) => {
                        match title {
                            Some(t) => {
                                set_display_tutorial.set(true);
                                set_course_title.set(t)
                            }
                            None => {
                                set_display_tutorial.set(false);
                            }
                        }
                    }
                    Err(_) => set_display_tutorial.set(false),
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
    use execution::TutorialExecutionArea;
    // use console::TutorialConsoleArea;

    let course_title = expect_context::<ReadSignal<String>>();

    let (chapter_number, set_chapter_number) = create_signal(1_u32);

    provide_context(chapter_number);
    provide_context(set_chapter_number);

    view! {
        <div style="float:left; font-weight:bold; padding-top:10px">
            <table>
                <tr>
                    <td style="padding-right: 50px">"用户: "{username.to_string()}</td>
                    <td style="padding-right: 50px">"课程: "{move || course_title.get()}</td>
                    <td style="padding-right: 50px">
                        "章节: " <ChapterSelectionBox course_id=course_id.clone() />
                    </td>
                </tr>
            </table>
        </div>
        <div>
            <TutorialEditor course_id=course_id.clone() username=username.to_string() />
            <TutorialExecutionArea username=username.to_string() />
        </div>
    }
}

#[component]
fn ChapterSelectionBox(course_id: String) -> impl IntoView {
    let (chapter_list, set_chapter_list) = create_signal(vec![Chapter::default()]);

    let chapter_number = expect_context::<ReadSignal<u32>>();

    let set_chapter_number = expect_context::<WriteSignal<u32>>();

    view! {
        {
            spawn_local(async move {
                match get_course_chapters(course_id).await {
                    Ok(chapters) => set_chapter_list.set(chapters),
                    Err(_) => set_chapter_list.set(Vec::new()),
                }
            });
        }

        <select
            on:change=move |ev| {
                let new_value = event_target_value(&ev);
                set_chapter_number
                    .set(new_value.split(".").collect::<Vec<&str>>()[0].parse().unwrap());
            }
            prop:chapter_number=move || chapter_number.get().to_string()
        >
            <For each=move || chapter_list.get() key=|state| (state.chapter_id.clone()) let:chapter>
                <option chapter_number=chapter
                    .chapter_number>{chapter.chapter_number}". "{chapter.title}</option>
            </For>
        </select>
    }
}

#[component]
fn TutorialEditor(course_id: String, username: String) -> impl IntoView {
    use editor::TutorialEditorArea;

    let (initial_code, set_initial_code) = create_signal("".to_string());

    let chapter_number = expect_context::<ReadSignal<u32>>();

    // our resource
    let code_content = create_resource(
        move || chapter_number.get(),
        // every time `chapter_number` changes, this will run
        move |value| {
            let course_id_clone = course_id.clone();
            async move {
                // logging::log!("loading course initial code from tutorial");
                get_tutorial_chapter(course_id_clone, value).await
            }
        },
    );

    view! {
        <Transition fallback=move || {
            view! { <p>"下载课程代码..."</p> }
        }>
            {move || match code_content.get() {
                Some(some_code_data) => {
                    match some_code_data {
                        Ok(ok_code_data) => {
                            match ok_code_data {
                                Some(code_data) => set_initial_code.set(code_data),
                                None => set_initial_code.set("".to_string()),
                            }
                        }
                        Err(_) => set_initial_code.set("".to_string()),
                    }
                }
                None => set_initial_code.set("".to_string()),
            }} <TutorialEditorArea initial_code=initial_code username=username.clone() />
        </Transition>
    }
}
