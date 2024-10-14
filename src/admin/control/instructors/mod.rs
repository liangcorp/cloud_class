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
        pub struct InstructorUsernameQuery {
            username: String,
        }

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
pub async fn get_all_instructors() -> Result<Vec<String>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(vec!["".to_string()]),
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let instructor_username = match sqlx::query_as::<_, InstructorUsernameQuery>(
        "SELECT username
        FROM instructors
        ORDER BY priority;",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(ok_instructor) => ok_instructor
            .iter()
            .map(|ok_instructor| ok_instructor.username.clone()
            )
            .collect(),
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    };

    Ok(instructor_username)
}

#[server]
pub async fn get_single_instructor(username: String) -> Result<InstructorInfo, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(InstructorInfo::default()),
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let instructor_info = match sqlx::query_as::<_, InstructorInfoQuery>(
        "SELECT *
        FROM instructors
        WHERE username = $1;",
    )
    .bind(&username)
    .fetch_one(&pool)
    .await
    {
        Ok(ok_instr_info) => InstructorInfo {
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
        },
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    };

    Ok(instructor_info)
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
    let (instructor_list, set_instructor_list) = create_signal(Vec::new());
    let (show_editor, set_show_editor) = create_signal(false);
    let (username, set_username) = create_signal("".to_string());
    let (instructor_info, set_instructor_info) = create_signal(InstructorInfo::default());

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
        match get_all_instructors().await {
            Ok(data) => {
                set_username.set(data[0].clone());
                set_instructor_list.set(data)
            },
            Err(e) => {
                set_instructor_list.set(Vec::new());
                logging::log!("ERROR<home/instructor_list.rs>: {}", e.to_string())
            },
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

    let on_username_select = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let user = (move || username.get())();

        spawn_local(async move {
            match get_single_instructor(user).await {
                Ok(data) => set_instructor_info.set(data),
                Err(e) => {
                    set_instructor_info.set(InstructorInfo::default());
                    logging::log!("ERROR<admin/control/instructors/mod.rs>: {}", e.to_string());
                }
            }
        });

        set_show_editor.set(true);
    };

    let on_username_change = move |ev| {
        let new_username_value = event_target_value(&ev);

        set_username.set(new_username_value);

        let user = (move || username.get())();

        spawn_local(async move {
            match get_single_instructor(user).await {
                Ok(data) => set_instructor_info.set(data),
                Err(e) => {
                    set_instructor_info.set(InstructorInfo::default());
                    logging::log!("ERROR<admin/control/instructors/mod.rs>: {}", e.to_string());
                }
            }
        });

    };

    let on_username_update = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        set_show_editor.set(false);
    };

    view! {
        <Await
            future=|| get_all_instructors()
            let:data
        >
            {
                let instructors = (data.as_ref().unwrap_or(&Vec::new())).to_vec();

                let first_instructor = instructors[0].clone();

                spawn_local(async move {
                    match get_single_instructor(first_instructor).await {
                        Ok(data) => set_instructor_info.set(data),
                        Err(e) => {
                            set_instructor_info.set(InstructorInfo::default());
                            logging::log!("ERROR<admin/control/instructors/mod.rs>: {}", e.to_string());
                        }
                    }
                });

                set_username.set(instructors[0].clone());
                set_instructor_list.set(instructors);


                view! {
                    <div class="contents" class:display=move || show_editor.get() >
                        <div>
                            <form on:submit=on_username_select>
                                <table>
                                    <tr>
                                        <td style="padding:10px">
                                            <label for="instructors">"教师用户名: "</label>
                                        </td>
                                        <td style="padding:10px">
                                            <select
                                                on:change=on_username_change
                                                prop:username=move || username.get()
                                            >
                                                <For each=move || instructor_list.get() key=|_| () let:instructor_username>
                                                    <option username=instructor_username.clone()>{instructor_username.clone()}</option>
                                                </For>
                                            </select>
                                        </td>
                                        <td style="padding:10px">
                                            <input type="submit" value="更改" />
                                        </td>
                                    </tr>
                                </table>
                            </form>
                        </div>
                        <div>
                            <table>
                                <tr>
                                    <td>"全名:"</td>
                                    <td>{move || instructor_info.get().fullname}</td>
                                </tr>
                                <tr>
                                    <td>"介绍:"</td>
                                    <td>{move || instructor_info.get().about}</td>
                                </tr>
                                <tr>
                                    <td>"学生数:"</td>
                                    <td>{move || instructor_info.get().total_students}</td>
                                </tr>
                                <tr>
                                    <td>"简介:"</td>
                                    <td>{move || instructor_info.get().tag_line}</td>
                                </tr>
                                <tr>
                                    <td>"加入日:"</td>
                                    <td>{move || instructor_info.get().start_date}</td>
                                </tr>
                                <tr>
                                    <td>"状态:"</td>
                                    <td>{move || instructor_info.get().status}</td>
                                </tr>
                                <tr>
                                    <td>"地址:"</td>
                                    <td>{move || instructor_info.get().address}</td>
                                </tr>
                                <tr>
                                    <td>"邮件:"</td>
                                    <td>{move || instructor_info.get().email}</td>
                                </tr>
                                <tr>
                                    <td>"电话号码:"</td>
                                    <td>{move || instructor_info.get().mobile}</td>
                                </tr>
                                <tr>
                                    <td>"优先权:"</td>
                                    <td>{move || instructor_info.get().priority}</td>
                                </tr>
                                <tr>
                                    <td>"评价:"</td>
                                    <td>{move || instructor_info.get().rating}</td>
                                </tr>
                                <tr>
                                    <td>"照片:"</td>
                                    <td><a href=format!("images/users/instructors/{}", instructor_info.get().profile_image_id)>{move || instructor_info.get().profile_image_id}</a></td>
                                </tr>
                            </table>
                        </div>
                    </div>

                    <div class="contents" class:display=move || !show_editor.get()>
                        <div>
                            <form on:submit=on_username_update>
                                <table>
                                    <tr>
                                        <td style="padding:10px">
                                            <label for="instructors">"教师用户名: "</label>
                                        </td>
                                        <td style="padding:10px">
                                            { move || username.get() }
                                        </td>
                                        <td style="padding:10px">
                                            <input type="submit" value="保存" />
                                        </td>
                                    </tr>
                                </table>
                            </form>
                        </div>
                        <div>
                            "editing panel"
                            {move || username.get()}
                        </div>
                    </div>
                }
            }
        </Await>

    }
}
