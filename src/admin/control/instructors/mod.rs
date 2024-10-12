use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;
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
/// Renders the control panel for manaing instructors
#[component]
pub fn AdminInstructorPage() -> impl IntoView {
    use super::header::HeaderSection;
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
                                <HeaderSection username=some_username.to_string() />
                                <AdminInstructorPanel />
                            }
                                .into_view()
                        }
                        None => view! { <Redirect path="/admin/login" /> }.into_view(),
                    }
                }
                Err(_) => view! { <Redirect path="/admin/login" /> }.into_view(),
            }}
        </Await>
    }
}

#[component]
fn AdminInstructorPanel() -> impl IntoView {
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
            <table class="control-instructor">
                <For each=move || instructor_list.get() key=|_| () let:instructor_info>
                    <tr>
                        <td>
                            {instructor_info.fullname}
                        </td>
                        <td>
                            {instructor_info.tag_line}
                        </td>
                        <td>
                            "加入日: "{instructor_info.start_date}
                        </td>
                    </tr>
                </For>
            </table>
    }
}
