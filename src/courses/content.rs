use leptos::*;
use leptos_router::*;
use server_fn::ServerFnError;
use serde::{Serialize, Deserialize};
use cfg_if::cfg_if;

#[derive(Params, PartialEq)]
struct CourseParams {
    id: Option<String>
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    chapter_id: String,
    title: String,
    chapter_number: u32,
    course_id: String
}

impl Default for Chapter {
    fn default() -> Chapter {
        Chapter {
            chapter_id: "".to_string(),
            title: "".to_string(),
            chapter_number: 0,
            course_id: "".to_string()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChapterContent {
    chapter_id: String,
    title: String,
    content: String,
    chapter_number: u32,
}

impl Default for ChapterContent {
    fn default() -> ChapterContent {
        ChapterContent {
            chapter_id: "".to_string(),
            title: "".to_string(),
            content: "".to_string(),
            chapter_number: 0,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use comrak::{markdown_to_html, Options};

        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct ChapterQuery {
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
    }
}

#[server]
pub async fn get_course_chapters(course_id: String) -> Result<Vec<Chapter>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state;
    match use_context::<AppState>() {
        Some(some_state) => state = some_state,
        None => return Ok(vec![Chapter::default()]),
    }

    //  取得数据库信息
    let pool = state.pool;

    /*---   提取用户数据    ---*/
    let chapters;

    match sqlx::query_as::<_, ChapterQuery>(
        "SELECT *
        FROM chapters
        WHERE course_id = $1
        ORDER BY chapter_number;"
    )
    .bind(&course_id)
    .fetch_all(&pool)
    .await {
        Ok(ok_chapters) => chapters =
            ok_chapters
                .iter()
                .map(|cc| Chapter {
                        chapter_id: cc.chapter_id.clone(),
                        title: cc.title.clone(),
                        chapter_number: cc.chapter_number.clone(),
                        course_id: cc.course_id.clone(),
                })
                .collect(),
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    }

    Ok(chapters)
}

#[server]
pub async fn get_chapter_content(chapter_id: String) -> Result<String, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state;
    match use_context::<AppState>() {
        Some(some_state) => state = some_state,
        None => return Ok("".to_string()),
    }

    //  取得数据库信息
    let pool = state.pool;

    /*---   提取用户数据    ---*/
    let chapter_content;

    match sqlx::query_as::<_, ChapterContentQuery>(
        "SELECT * FROM chapters WHERE chapter_id = $1;",
    )
    .bind(&chapter_id)
    .fetch_one(&pool)
    .await {
        Ok(ok_chapter_content) => chapter_content = ok_chapter_content,
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    }
    // logging::log!("transform content to raw HTML");
    let result_html = markdown_to_html(chapter_content.content.as_str(), &Options::default());

    Ok(result_html)
}

#[component]
pub fn ContentPage() -> impl IntoView {
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
                            view! {
                                <DisplayUserCourseContent username=some_username.to_string() />
                            }
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
pub fn DisplayUserCourseContent(username: String) -> impl IntoView {
    let (chapter_id, set_chapter_id) = create_signal("welcome-0000".to_string());
    let (show_chapters, set_show_chapters) = create_signal(Vec::new());

    // @TODO: collapsible side navigation panel
    // let (show_navbar, set_show_navbar) = create_signal(true);

    let params = use_params_map();

    // id: || -> Option<String>
    let course_id = move || params.with_untracked(|params| params.get("course_id").cloned());

    // create_resource takes two arguments after its scope
    let async_data = create_resource(
        // the first is the "source signal"
        move || chapter_id.get(),
        // the second is the loader
        // it takes the source signal's value as its argument
        // and does some async work
        |value| async move { get_chapter_content(value).await },
    );

    view! {
        {if course_id().is_some() {
            spawn_local(async move {
                match get_course_chapters(course_id().unwrap().clone()).await {
                    Ok(ok_course_chapters) => set_show_chapters.set(ok_course_chapters),
                    Err(_) => {
                        set_show_chapters.set(Vec::new());
                    }
                }
            })
        }}
        <div align="right" style="height:30px">
            <table>
                <tr>
                    <td style="padding-right:20px">
                        <a
                            target="_blank"
                            rel="noopener noreferrer"
                            href=format!("/tutorials/{}", course_id().unwrap())
                            class="tutorial_link"
                        >
                            "⚒ 实验室"
                        </a>
                    </td>
                    <td class="header_login">
                        <a class="header" href="/courses">
                            {username}
                        </a>
                    </td>
                    <td class="header_login">
                        <a href="/logout" class="home_login">
                            "退出"
                        </a>
                    </td>
                </tr>
            </table>
        </div>
        <div class="sidenav">
            <ul style="list-style-type:none">
                <For
                    each=move || show_chapters.get()
                    key=|state| (state.chapter_id.clone())
                    let:chapter
                >
                    <li>
                        <p>
                            <a
                                on:click=move |_| {
                                    set_chapter_id.set(chapter.chapter_id.clone());
                                }
                                href="#"
                            >
                                <div
                                    style="float: left;"
                                    class:display=move || chapter.chapter_number == 0
                                >
                                    <b style="padding-right:5px;">{chapter.chapter_number}"."</b>
                                </div>
                                {chapter.title}
                            </a>
                        </p>
                    </li>
                </For>
            </ul>
        </div>
        <div class="main">
            <Transition fallback=move || {
                view! { <p>"正在下载课程章节..."</p> }
            }>
                <div inner_html=move || {
                    async_data.get().map(|value| value.unwrap().to_string())
                } />
            </Transition>
        </div>
    }
}
