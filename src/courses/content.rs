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
        use comrak::{markdown_to_html, Options};

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
        pub struct ChapterContentQuery {
            chapter_id: String,
            title: String,
            content: String,
            chapter_number: u32,
        }

        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct UserChapterQuery {
            username: String,
            course_id: String
        }
    }
}

#[server]
pub async fn check_user_courses(user: String, course_id: String) -> Result<bool, ServerFnError> {
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
        WHERE username = $1 AND course_id = $2",
    )
    .bind(&user)
    .bind(&course_id)
    .fetch_one(&pool)
    .await
    {
        Ok(_) => Ok(true),
        Err(_) => Ok(false), // Course not found
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

    /*---   提取用户数据    ---*/
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
pub async fn get_chapter_content(chapter_id: String) -> Result<String, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(some_state) => some_state,
        None => return Ok("".to_string()),
    };

    //  取得数据库信息
    let pool = state.pool;

    /*---   提取用户数据    ---*/
    let chapter_content = match sqlx::query_as::<_, ChapterContentQuery>(
        "SELECT * FROM chapters WHERE chapter_id = $1;",
    )
    .bind(&chapter_id)
    .fetch_one(&pool)
    .await
    {
        Ok(ok_chapter_content) => ok_chapter_content,
        Err(e) => {
            return Err(ServerFnError::Args(format!(
                "ERROR:<courses/content.rs:get_chapter_content>: {}",
                e
            )))
        }
    };

    // logging::log!("transform content to raw HTML");
    let result_html = markdown_to_html(chapter_content.content.as_str(), &Options::default());

    Ok(result_html)
}

#[component]
pub fn ContentPageGate() -> impl IntoView {
    use crate::session::*;

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
                            view! { <CourseContentGate username=some_username.to_string() /> }
                        }
                        None => view! { <Redirect path="/courses" /> },
                    }
                }
                Err(_) => view! { <Redirect path="/courses" /> },
            }}
        </Await>
    }
}

#[component]
fn CourseContentGate(username: String) -> impl IntoView {
    let params = use_params_map();

    // id: || -> Option<String>
    let course_id = move || params.with_untracked(|params| params.get("course_id").cloned());

    let (disable, set_disable) = create_signal(true);

    let username_clone = username.clone();

    provide_context(disable);

    view! {
        {match course_id() {
            Some(c_id) => {
                if c_id.is_empty() {
                    return vec![view! { <Redirect path="/courses" /> }];
                }
                spawn_local(async move {
                    match check_user_courses(username_clone, c_id).await {
                        Ok(result_bool) => set_disable.set(result_bool),
                        Err(_) => set_disable.set(true),
                    }
                })
            }
            None => return vec![view! { <Redirect path="/courses" /> }],
        }}

        <div class:display=move || disable.get()>
            <div>
                <a class="header" href="/courses">
                    "回到个人主页"
                </a>
            </div>
            <div style="margin-left:40%">
                <h2>
                    <p style="color:red">"您不能访问这节课程"</p>
                </h2>
            </div>
        </div>
        <div class:cover-up-chapter=move || !disable.get() class:isDisabled=move || !disable.get()>
            <CourseContent username=username course_id=course_id().unwrap() />
        </div>
    }
}

#[component]
fn CourseContent(username: String, course_id: String) -> impl IntoView {
    view! {
        <CourseContentHeader username=username course_id=course_id.clone() />
        <CourseContentBody course_id=course_id />
    }
}

#[component]
fn CourseContentHeader(username: String, course_id: String) -> impl IntoView {
    view! {
        <div align="right" style="height:30px">
            <table>
                <tr>
                    <td style="padding-right:20px">
                        <a
                            target="_blank"
                            rel="noopener noreferrer"
                            href=format!("/tutorials/{}", course_id)
                            class="tutorial-link"
                        >
                            "⚒ 实验室"
                        </a>
                    </td>
                    <td class="header-login">
                        <a class="header" href="/courses">
                            {username}
                        </a>
                    </td>
                    <td class="header-login">
                        <a href="/logout" class="home-login">
                            "退出"
                        </a>
                    </td>
                </tr>
            </table>
        </div>
    }
}

#[component]
fn CourseContentBody(course_id: String) -> impl IntoView {
    // @TODO: collapsible side navigation panel
    // let (show_navbar, set_show_navbar) = create_signal(true);

    let (chapter_id, set_chapter_id) = create_signal("welcome-0000".to_string());

    provide_context(chapter_id);
    provide_context(set_chapter_id);

    view! {
        <div class="sidenav">
            <div style="padding-left:10px;padding-bottom:20px;">
                <a class="header" href="/courses">
                    "返回个人主页"
                </a>
            </div>
            <div>
                <ul style="list-style-type:none">
                    <ChapterList course_id=course_id />
                </ul>
            </div>
        </div>
        <div class="chapter-content">
            <ChapterContent />
        </div>
    }
}

#[component]
fn ChapterList(course_id: String) -> impl IntoView {
    let (show_chapters, set_show_chapters) = create_signal(Vec::new());

    let set_chapter_id = expect_context::<WriteSignal<String>>();

    let disable = expect_context::<ReadSignal<bool>>();

    view! {
        {
            spawn_local(async move {
                match get_course_chapters(course_id).await {
                    Ok(ok_course_chapters) => set_show_chapters.set(ok_course_chapters),
                    Err(_) => set_show_chapters.set(Vec::new()),
                }
            });
        }

        <For each=move || show_chapters.get() key=|state| (state.chapter_id.clone()) let:chapter>
            <li>
                <p>
                    <a
                        on:click=move |_| {
                            set_chapter_id.set(chapter.chapter_id.clone());
                        }
                        class:isDisabled=move || !disable.get()
                        href="#"
                    >
                        <div style="float: left;" class:display=move || chapter.chapter_number == 0>
                            <b style="padding-right:5px;">{chapter.chapter_number}"."</b>
                        </div>
                        {chapter.title}
                    </a>
                </p>
            </li>
        </For>
    }
}

#[component]
fn ChapterContent() -> impl IntoView {
    let chapter_id = expect_context::<ReadSignal<String>>();

    // create_resource takes two arguments after its scope
    let async_chapter_content = create_resource(
        // the first is the "source signal"
        move || chapter_id.get(),
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { get_chapter_content(value).await },
    );

    view! {
        <Transition fallback=move || {
            view! { <p>"正在下载课程章节..."</p> }
        }>
            <div inner_html=move || {
                async_chapter_content.get().map(|value| value.unwrap().to_string())
            } />
        </Transition>
    }
}
