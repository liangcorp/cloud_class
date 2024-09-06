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
    content: String,
    chapter_number: u32,
    course_id: String
}

impl Default for Chapter {
    fn default() -> Chapter {
        Chapter {
            chapter_id: "".to_string(),
            title: "".to_string(),
            content: "".to_string(),
            chapter_number: 0,
            course_id: "".to_string()
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct ChapterContentQuery {
            chapter_id: String,
            title: String,
            content: String,
            chapter_number: u32,
            course_id: String
        }
    }
}

#[server]
pub async fn get_course_content(course_id: String) -> Result<Vec<Chapter>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state;
    match use_context::<AppState>() {
        Some(s) => state = s,
        None => return Ok(vec![Chapter::default()]),
    }

    //  取得数据库信息
    let pool = state.pool;

    /*---   提取用户数据    ---*/
    let chapter_content;

    match sqlx::query_as::<_, ChapterContentQuery>(
        "SELECT * FROM chapters WHERE course_id = $1 ORDER BY chapter_number;",
    )
    .bind(&course_id)
    .fetch_all(&pool)
    .await {
        Ok(cc) => chapter_content = cc,
        Err(e) => {
            return Err(ServerFnError::Args(e.to_string()))
        },
    }

    let result_content = chapter_content
        .iter()
        .map(|cc| Chapter {
                chapter_id: cc.chapter_id.clone(),
                title: cc.title.clone(),
                content: cc.content.clone(),
                chapter_number: cc.chapter_number.clone(),
                course_id: cc.course_id.clone(),
        })
        .collect();

    Ok(result_content)
}
#[component]
pub fn ContentPage() -> impl IntoView {
    let (content, set_content) = create_signal(Vec::new());

    let params = use_params_map();

    // id: || -> Option<String>
    let id = move || {
        params.with(|params| params.get("id").cloned())
    };

    if id() != None {
        spawn_local(
            async move {
                match get_course_content(id().unwrap().clone()).await {
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
        <a href="/courses">回到个人资料</a>
        // <p> { move || content.get() } </p>
        <div class="contents">
            <For
                each=move || content.get()
                key=|state| (state.chapter_id.clone())
                let:chapter_content
            >
                <b>{chapter_content.chapter_number}". "{chapter_content.title}</b>
                <br/>
                {chapter_content.content}
                <br />
            </For>
        </div>
    }
}
