pub mod about;
pub mod collaboration;
pub mod contact;
pub mod instructor_list;

use cfg_if::cfg_if;
use leptos::prelude::*;
use leptos_meta::Title;
use serde::{Deserialize, Serialize};
use server_fn::ServerFnError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SponsorInfo {
    uuid: String,
}

impl Default for SponsorInfo {
    fn default() -> SponsorInfo {
        SponsorInfo {
            uuid: "".to_string(),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        #[derive(Clone, Debug, PartialEq)]
        #[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
        pub struct SponsorInfoQuery {
            uuid: String,
        }
    }
}

#[server]
pub async fn get_sponsor_icons() -> Result<Vec<SponsorInfo>, ServerFnError> {
    use crate::state::AppState;

    //  取得软件状态
    let state = match use_context::<AppState>() {
        Some(s) => s,
        None => return Ok(vec![SponsorInfo::default()]),
    };

    //  取得数据库信息
    let pool = state.pool;

    //  提取用户数据
    let sponsor_list = match sqlx::query_as::<_, SponsorInfoQuery>(
        "SELECT uuid
        FROM sponsors
        ORDER BY priority;",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(ok_sponsor_info) => ok_sponsor_info
            .iter()
            .map(|ok_sponsor_info| SponsorInfo {
                uuid: ok_sponsor_info.uuid.clone(),
            })
            .collect(),
        Err(e) => return Err(ServerFnError::Args(e.to_string())),
    };

    Ok(sponsor_list)
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    use crate::header::HeaderSection;

    view! {
        <Title text="首页" />

        <HeaderSection />

        <img src="images/banners/default_home.jpg" style="width:100%" />

        <div class="contents">
            <div style="align:center">
                <h3>"受到全球 16,000 多家公司和数百万学习者的信赖"</h3>

                <SponsorsPanel />

            </div>
        </div>
    }
}

/// Render the sponsors icons
#[component]
pub fn SponsorsPanel() -> impl IntoView {
    view! {
        <Await future=get_sponsor_icons() let:data>
            {
                let content = match data.as_ref() {
                    Ok(d) => (*d).clone(),
                    Err(_) => Vec::new(),
                };
                view! {
                    <table>
                        <tr>
                            <For each=move || { content.clone() } key=|_| () let:image_entry>
                                <td>
                                    <img
                                        src=format!(
                                            "images/sponsors/{}.png",
                                            image_entry.uuid.to_string(),
                                        )
                                        // style="width:150px;filter: grayscale(100%);-webkit-filter: grayscale(100%);"
                                        // To grayscale or not to grayscale? That is the question.
                                        style="width:150px;"
                                    />
                                </td>
                            </For>
                        </tr>
                    </table>
                }
            }
        </Await>
    }
}
