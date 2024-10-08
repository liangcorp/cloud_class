use cfg_if::cfg_if;
use leptos::*;
use server_fn::ServerFnError;
// use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct User {
            username: String,
            container_port: i32,
        }
    }
}

#[server]
pub async fn start_if_stopped_container(username: String) -> Result<String, ServerFnError> {
    use crate::state::AppState;
    use std::process::Command;


    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => {
            return Err(ServerFnError::Args(
                "ERROR<tutorials/execution.rs>: during application state retrieval".to_string(),
            ))
        }
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let sql_result = sqlx::query_as::<_, User>("SELECT username, container_port FROM students WHERE username==$1;")
        .bind(&username)
        .fetch_one(&pool)
        .await?;

    //  docker run -p <user's container port no>:8501 -d --name <username> -it streamlit
    let command_error = match Command::new("docker")
        .arg("run")
        .arg("-p")
        .arg(format!("{}:8501", sql_result.container_port))
        .arg("-q")
        .arg("-d")
        .arg("--name")
        .arg(&username)
        .arg("-it")
        .arg("streamlit")
        .output()
    {
        Ok(ok_output) => {
            ok_output.stderr
        }
        Err(e) => {
            // logging::log!("ERROR <tutorials/execution.rs:56>: {}", e.to_string());
            return Err(ServerFnError::Args(e.to_string()));
        }
    };

    if !command_error.is_empty() {
        let _ = match Command::new("docker")
            .arg("container")
            .arg("start")
            .arg(format!("{}", &username))
            .output()
        {
            Ok(_) => return Ok(sql_result.container_port.to_string()),
            Err(e) => {
                // logging::log!("ERROR <tutorials/execution.rs:56>: {}", e.to_string());
                return Err(ServerFnError::Args(e.to_string()));
            }
        };
    }

    Ok(sql_result.container_port.to_string())
}

#[component]
pub fn TutorialExecutionArea(username: String) -> impl IntoView {
    view! {

        <Await
            // `future` provides the `Future` to be resolved
            future=move || start_if_stopped_container(username.clone())
            // the data is bound to whatever variable name you provide
            let:container_port
        >
            <div class="output-area">
                // "container_port" was created during user creation.
                // So it's safe to unwrap here
                <iframe class="code-execution" src=format!("http://localhost:{}/", container_port.as_ref().unwrap()) />
            </div>
        </Await>
    }
}
