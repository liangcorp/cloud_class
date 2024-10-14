use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InstructorInfo {
    username: String,
    fullname: String,
    about: String,
    total_students: i32,
    tag_line: String,
    start_date: String,
    status: String,
    address: String,
    email: String,
    mobile: String,
    priority: i32,
    rating: i8,
    profile_image_id: String,
}

impl Default for InstructorInfo {
    fn default() -> InstructorInfo {
        InstructorInfo {
            username: "".to_string(),
            fullname: "".to_string(),
            about: "".to_string(),
            total_students: 0,
            tag_line: "".to_string(),
            start_date: "".to_string(),
            status: "".to_string(),
            address: "".to_string(),
            email: "".to_string(),
            mobile: "".to_string(),
            priority: 0,
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
            username: String,
            fullname: String,
            about: String,
            total_students: i32,
            tag_line: String,
            start_date: String,
            status: String,
            address: String,
            email: String,
            mobile: String,
            priority: i32,
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
        "SELECT *
        FROM instructors
        ORDER BY priority;",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(ok_instr_info) => ok_instr_info
            .iter()
            .map(|ok_instr_info| InstructorInfo {
                username: ok_instr_info.username.clone(),
                fullname: ok_instr_info.fullname.clone(),
                about: ok_instr_info.about.clone(),
                tag_line: ok_instr_info.tag_line.clone(),
                total_students: ok_instr_info.total_students,
                start_date: ok_instr_info.start_date.clone(),
                status: ok_instr_info.status.clone(),
                address: ok_instr_info.address.clone(),
                email: ok_instr_info.email.clone(),
                mobile: ok_instr_info.mobile.clone(),
                priority: ok_instr_info.priority,
                rating: ok_instr_info.rating,
                profile_image_id: ok_instr_info.profile_image_id.clone(),
            })
            .collect(),
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    };

    Ok(instructor_list)
}

#[server]
pub async fn update_instructor_info() -> Result<(), ServerFnError> {
    todo!()
}

/// Renders the admin login check panel
#[component]
pub fn AdminInstructorPortal() -> impl IntoView {
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
                                <AdminInstructorPage />
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

/// Rendering control panel for instructors
#[component]
fn AdminInstructorPage() -> impl IntoView {
    view! {
        <table class="control-instructor">
            <tr>
                <th class="control-instructor">"用户名"</th>
                <th class="control-instructor">"全名"</th>
                <th class="control-instructor">"about"</th>
                <th class="control-instructor">"tag_line"</th>
                <th class="control-instructor">"total_students"</th>
                <th class="control-instructor">"start_date"</th>
                <th class="control-instructor">"status"</th>
                <th class="control-instructor">"address"</th>
                <th class="control-instructor">"email"</th>
                <th class="control-instructor">"mobile"</th>
                <th class="control-instructor">"priority"</th>
                <th class="control-instructor">"rating"</th>
                <th class="control-instructor">"profile_image_id"</th>
                <th class="control-instructor">"保存"</th>
                <th class="control-instructor">"删除"</th>
            </tr>
            <DisplayInstructors />
        </table>
    }
}

/// Rendering iterator of instructors
#[component]
fn DisplayInstructors() -> impl IntoView {
    let (instructor_list, set_instructor_list) = create_signal(Vec::new());

    let input_fullname: NodeRef<html::Input> = create_node_ref();
    let input_about: NodeRef<html::Input> = create_node_ref();
    let input_tag_line: NodeRef<html::Input> = create_node_ref();
    let input_total_students: NodeRef<html::Input> = create_node_ref();
    let input_start_date: NodeRef<html::Input> = create_node_ref();
    let input_status: NodeRef<html::Input> = create_node_ref();
    let input_address: NodeRef<html::Input> = create_node_ref();
    let input_email: NodeRef<html::Input> = create_node_ref();
    let input_mobile: NodeRef<html::Input> = create_node_ref();
    let input_priority: NodeRef<html::Input> = create_node_ref();
    let input_rating: NodeRef<html::Input> = create_node_ref();

    spawn_local(async move {
        match get_instructors().await {
            Ok(data) => set_instructor_list.set(data),
            Err(e) => {
                set_instructor_list.set(Vec::new());
                logging::log!("ERROR<home/instructor_list.rs>: {}", e.to_string());
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let fullname_value = input_fullname
            .get()
            .expect("<input> should be mounted")
            .value();

        let about_value = input_about
            .get()
            .expect("<input> should be mounted")
            .value();

        let tag_line_value = input_tag_line
            .get()
            .expect("<input> should be mounted")
            .value();

        let total_students_value = input_total_students
            .get()
            .expect("<input> should be mounted")
            .value();

        let start_date_value = input_start_date
            .get()
            .expect("<input> should be mounted")
            .value();

        let status_value = input_status
            .get()
            .expect("<input> should be mounted")
            .value();

        let address_value = input_address
            .get()
            .expect("<input> should be mounted")
            .value();

        let email_value = input_email
            .get()
            .expect("<input> should be mounted")
            .value();

        let mobile_value = input_mobile
            .get()
            .expect("<input> should be mounted")
            .value();

        let priority_value = input_priority
            .get()
            .expect("<input> should be mounted")
            .value();

        let rating_value = input_rating
            .get()
            .expect("<input> should be mounted")
            .value();
    };

    view! {
        <For each=move || instructor_list.get() key=|_| () let:instructor_info>
            <tr>
                <td class="control-instructor">
                    {instructor_info.username}
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.fullname
                        node_ref=input_fullname
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.about
                        node_ref=input_about
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.tag_line
                        node_ref=input_tag_line
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.total_students
                        node_ref=input_total_students
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.start_date
                        node_ref=input_start_date
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.status
                        node_ref=input_status
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.address
                        node_ref=input_address
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.email
                        node_ref=input_email
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.mobile
                        node_ref=input_mobile
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.priority
                        node_ref=input_priority
                    />
                </td>
                <td class="control-instructor">
                    <input
                        type="text"
                        value=instructor_info.rating
                        node_ref=input_rating
                    />
                </td>
                <td class="control-instructor">
                    <img
                        src=format!(
                            "images/users/instructors/{}",
                            { instructor_info.profile_image_id },
                        )
                        style="width:100px"
                    />
                </td>
                <td class="control-instructor">
                    <button>"保存"</button>
                </td>
                <td class="control-instructor">
                    <button>"删除"</button>
                </td>
                </form>
            </tr>
        </For>
    }
}
