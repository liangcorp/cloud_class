use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::Title;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstructorInfo {
    fullname: String,
    tag_line: String,
    start_date: String,
    rating: i8,
    profile_image_id: String,
}

impl Default for InstructorInfo {
    fn default() -> InstructorInfo {
        InstructorInfo {
            fullname: "".to_string(),
            tag_line: "".to_string(),
            start_date: "".to_string(),
            rating: 5,
            profile_image_id: "default_profile.png".to_string(),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct InstructorInfoQuery {
            fullname: String,
            tag_line: String,
            start_date: String,
            rating: i8,
            profile_image_id: String,
        }
    }
}

#[server]
pub async fn get_instructors() -> Result<Vec<InstructorInfo>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(vec![InstructorInfo::default()]),
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let instructor_list = match sqlx::query_as::<_, InstructorInfoQuery>(
        "SELECT fullname, tag_line, start_date, rating, profile_image_id
        FROM instructors
        WHERE status = 'active'
        ORDER BY priority;",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(ok_instr_info) => ok_instr_info
            .iter()
            .map(|ok_instr_info| InstructorInfo {
                fullname: ok_instr_info.fullname.clone(),
                tag_line: ok_instr_info.tag_line.clone(),
                start_date: ok_instr_info.start_date.clone(),
                rating: ok_instr_info.rating,
                profile_image_id: ok_instr_info.profile_image_id.clone(),
            })
            .collect(),
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    };

    Ok(instructor_list)
}

/// Renders the home page of your application.
#[component]
pub fn InstructorListPage() -> impl IntoView {
    use crate::header::HeaderSection;

    view! {
        <Title text="教师中心" />

        <HeaderSection />

        <div class="contents">
            <div class="instructor-panel">
                <InstructorListPanel />
            </div>
        </div>
    }
}

#[component]
pub fn InstructorListPanel() -> impl IntoView {
    let (instructor_list, set_instructor_list) = create_signal(Vec::new());

    spawn_local(async move {
        match get_instructors().await {
            Ok(data) => set_instructor_list.set(data),
            Err(e) => {
                set_instructor_list.set(Vec::new());
                logging::log!("ERROR<home/instructor_list.rs>: {}", e.to_string());
            }
        }
    });

    view! {
        <For each=move || instructor_list.get() key=|_| () let:instructor_info>
            <table>
                <tr>
                    <td>
                        <img
                            src=format!(
                                "images/users/instructors/{}",
                                instructor_info.profile_image_id,
                            )
                            style="width:200px"
                        />
                    </td>
                    <td style="width:200px;word-wrap:break-word;padding-left:5px">
                        <p>
                            <b>{instructor_info.fullname}</b>
                        </p>
                        <p>{instructor_info.tag_line}</p>
                        <p>"加入日: "{instructor_info.start_date}</p>
                        <p>
                            <span>
                                {(0..instructor_info.rating)
                                    .map(|_| view! { <span style="color:red;">"★"</span> })
                                    .collect_view()}
                            </span>
                            <span>
                                {(instructor_info.rating..5)
                                    .map(|_| view! { <span style="color:gray;">"★"</span> })
                                    .collect_view()}
                            </span>
                        </p>
                    </td>
                </tr>
                <tr></tr>
            </table>
        </For>
    }
}
