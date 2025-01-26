use cfg_if::cfg_if;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::*;
use leptos_router::components::Redirect;
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
            .map(|ok_instructor| ok_instructor.username.clone())
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
pub async fn update_instructor_info(info: InstructorInfo) -> Result<Option<String>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Err(ServerFnError::Args("Error in database pool".to_string())),
    };

    //  取得数据库信息
    let pool = state.pool;

    match sqlx::query("UPDATE instructors
        SET fullname = $1, about = $2, total_students = $3, tag_line = $4, start_date = $5, status = $6, address = $7, email = $8, mobile = $9, priority = $10, rating = $11
        WHERE username = $12;")
        .bind(&info.fullname)
        .bind(&info.about)
        .bind(info.total_students)
        .bind(&info.tag_line)
        .bind(&info.start_date)
        .bind(&info.status)
        .bind(&info.address)
        .bind(&info.email)
        .bind(&info.mobile)
        .bind(info.priority)
        .bind(info.rating)
        .bind(&info.username)
        .execute(&pool)
        .await {
            Ok(_query_result) => {
                // logging::log!("DEBUG<admin/control/instructors/mod.rs>: {:?}", query_result);
               Ok(None)
            },
            Err(e) => {
                Ok(Some(e.to_string()))
            },
        }
}

/// Renders the admin login check panel
#[component]
pub fn AdminInstructorPortal() -> impl IntoView {
    use super::header::HeaderSection;
    use crate::session::extract_session_user;

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=extract_session_user()

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
                            }.into_any()

                        }
                        None => view! { <Redirect path="/admin/login" /> }.into_any(),
                    }
                }
                Err(_) => view! { <Redirect path="/admin/login" /> }.into_any(),
            }}
        </Await>
    }
}

/// Rendering control panel for instructors
#[component]
fn AdminInstructorPage() -> impl IntoView {
    let (instructor_list, set_instructor_list) = signal(Vec::new());
    let (show_editor, set_show_editor) = signal(false);
    let (username, set_username) = signal("".to_string());
    let (instructor_info, set_instructor_info) = signal(InstructorInfo::default());
    let (show_db_update_error, set_show_db_update_error) = signal(false);
    let (db_update_error, set_db_update_error) = signal("".to_string());

    let input_username: NodeRef<html::Input> = NodeRef::new();
    let input_fullname: NodeRef<html::Input> = NodeRef::new();
    let input_about: NodeRef<html::Input> = NodeRef::new();
    let input_tag_line: NodeRef<html::Input> = NodeRef::new();
    let input_total_students: NodeRef<html::Input> = NodeRef::new();
    let input_start_date: NodeRef<html::Input> = NodeRef::new();
    let input_status: NodeRef<html::Input> = NodeRef::new();
    let input_address: NodeRef<html::Input> = NodeRef::new();
    let input_email: NodeRef<html::Input> = NodeRef::new();
    let input_mobile: NodeRef<html::Input> = NodeRef::new();
    let input_priority: NodeRef<html::Input> = NodeRef::new();
    let input_rating: NodeRef<html::Input> = NodeRef::new();

    spawn_local(async move {
        match get_all_instructors().await {
            Ok(data) => {
                set_username.set(data[0].clone());
                set_instructor_list.set(data)
            }
            Err(_e) => {
                set_instructor_list.set(Vec::new());
                // logging::log!("ERROR<home/instructor_list.rs>: {}", e.to_string())
            }
        }
    });

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let username_value = input_username
            .get()
            .expect("<input> should be mounted")
            .value();

        let username_value_clone = username_value.clone();

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

        spawn_local(async move {
            match update_instructor_info(InstructorInfo {
                username: username_value,
                fullname: fullname_value,
                about: about_value,
                tag_line: tag_line_value,
                total_students: total_students_value.parse().unwrap(),
                start_date: start_date_value,
                status: status_value,
                address: address_value,
                email: email_value,
                mobile: mobile_value,
                priority: priority_value.parse().unwrap(),
                rating: rating_value.parse().unwrap(),
                ..Default::default()
            })
            .await
            {
                Ok(ok_query_error) => match ok_query_error {
                    None => {
                        set_show_db_update_error.set(true);
                        set_show_editor.set(false)
                    }
                    Some(error) => {
                        set_show_db_update_error.set(false);
                        // logging::log!("ERROR<admin/control/instructors/mod.rs>: {}", &error);
                        set_db_update_error.set(error)
                    }
                },
                Err(e) => {
                    logging::log!("ERROR<admin/control/instructors/mod.rs>: {}", e.to_string());
                }
            };
        });

        spawn_local(async move {
            match get_single_instructor(username_value_clone).await {
                Ok(data) => set_instructor_info.set(data),
                Err(_e) => {
                    set_instructor_info.set(InstructorInfo::default());
                    // logging::log!("ERROR<admin/control/instructors/mod.rs>: {}", e.to_string());
                }
            }
        });
    };

    let on_username_select = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let user = username.get();

        spawn_local(async move {
            match get_single_instructor(user).await {
                Ok(data) => set_instructor_info.set(data),
                Err(_e) => {
                    set_instructor_info.set(InstructorInfo::default());
                    // logging::log!("ERROR<admin/control/instructors/mod.rs>: {}", e.to_string());
                }
            }
        });

        set_show_editor.set(true);
        set_show_db_update_error.set(true);
    };

    let on_username_change = move |ev| {
        let new_username_value = event_target_value(&ev);

        set_username.set(new_username_value);

        let user = username.get();

        spawn_local(async move {
            match get_single_instructor(user).await {
                Ok(data) => set_instructor_info.set(data),
                Err(_e) => {
                    set_instructor_info.set(InstructorInfo::default());
                    // logging::log!("ERROR<admin/control/instructors/mod.rs>: {}", e.to_string());
                }
            }
        });
    };

    view! {
        <Await future=get_all_instructors() let:data>
            {
                let instructors = (data.as_ref().unwrap_or(&Vec::new())).to_vec();
                let first_instructor = instructors[0].clone();
                spawn_local(async move {
                    match get_single_instructor(first_instructor).await {
                        Ok(data) => set_instructor_info.set(data),
                        Err(_e) => {
                            set_instructor_info.set(InstructorInfo::default());
                            // logging::log!(
                            //     "ERROR<admin/control/instructors/mod.rs>: {}", e.to_string()
                            // );
                        }
                    }
                });
                set_username.set(instructors[0].clone());
                set_instructor_list.set(instructors);
                view! {
                    // Display panel
                    <div class="contents" class:display=move || show_editor.get()>
                        // Select instructor panel
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
                                                <For
                                                    each=move || instructor_list.get()
                                                    key=|_| ()
                                                    let:instructor_username
                                                >
                                                    <option username=instructor_username
                                                        .clone()>{instructor_username.clone()}</option>
                                                </For>
                                            </select>
                                        </td>
                                        <td style="padding:10px">
                                            <input type="submit" value="更改" />
                                        </td>
                                    </tr>
                                </table>
                            </form>
                        // End of select instructor panel
                        </div>

                        // Display content
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
                                    <td>
                                        <img src=move || {
                                            format!(
                                                "images/users/instructors/{}",
                                                instructor_info.get().profile_image_id,
                                            )
                                        } />
                                    </td>
                                </tr>
                            </table>
                        // End of display content
                        </div>
                    // End of display panel
                    </div>

                    // Edit panel
                    <div class="contents" class:display=move || !show_editor.get()>
                        <form on:submit=on_submit>
                            // Show control buttons
                            <div>
                                <table>
                                    <tr>
                                        <td style="padding:10px">
                                            <label for="instructors">"教师用户名: "</label>
                                        </td>
                                        <td style="padding:10px">{move || username.get()}</td>
                                        <td style="padding:10px">
                                            <input type="submit" value="保存" />
                                            <span style="padding-left:5px"></span>
                                            <button on:click=move |ev| {
                                                ev.prevent_default();
                                                set_show_editor.set(false);
                                            }>"取消"</button>
                                        </td>
                                    </tr>
                                </table>
                            // End of show control buttons
                            </div>

                            // Display input boxes
                            <div>
                                <div class:display=move || show_db_update_error.get()>
                                    <h3 style="color:red">
                                        "数据库保存问题: "{move || db_update_error.get()}
                                    </h3>
                                </div>
                                <table>
                                    <tr>
                                        <td>"用户名"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().username
                                                node_ref=input_username
                                                disabled
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"全名:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().fullname
                                                node_ref=input_fullname
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"介绍:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().about
                                                node_ref=input_about
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"学生数:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().total_students
                                                node_ref=input_total_students
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"简介:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().tag_line
                                                node_ref=input_tag_line
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"加入日:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().start_date
                                                node_ref=input_start_date
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"状态:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().status
                                                node_ref=input_status
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"地址:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().address
                                                node_ref=input_address
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"邮件:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().email
                                                node_ref=input_email
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"电话号码:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().mobile
                                                node_ref=input_mobile
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"优先权:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().priority
                                                node_ref=input_priority
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"评价:"</td>
                                        <td>
                                            <input
                                                type="text"
                                                value=move || instructor_info.get().rating
                                                node_ref=input_rating
                                            />
                                        </td>
                                    </tr>
                                    <tr>
                                        <td>"照片:"</td>
                                        <td>
                                            <input
                                                type="file"
                                                value=move || {
                                                    format!(
                                                        "images/users/instructors/{}",
                                                        instructor_info.get().profile_image_id,
                                                    )
                                                }
                                            />
                                        </td>
                                    </tr>
                                </table>
                            // End of display input boxes
                            </div>
                        </form>
                    // End of edit panel
                    </div>

                    // Adding new instructor page
                    <div>
                    </div>
                }
            }
        </Await>
    }
}
