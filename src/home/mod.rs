use leptos::*;
// use serde::Deserialize;

// #[cfg(feature = "ssr", derive(serde::Deserialize))]
// #[derive(Deserialize, Debug)]
// struct MyQuery {
//     response: ResponseOptions,
// }

#[server]
pub async fn axum_extract() -> Result<String, ServerFnError> {
    // use axum::{extract::Query, http::{Method, header::{HeaderMap, HeaderValue}}};
    use axum::http::header::{HeaderMap, HeaderValue};
    use leptos_axum::extract;

    // let (method, query): (Method, Query<MyQuery>);
    let header: HeaderMap<HeaderValue>;

    match extract().await {
        Ok(h) => {
            header = h;
            logging::log!("{:?}", header.get("cookie"));
        },
        Err(e) => {
            logging::log!("Error: {}", e.to_string());
        },
    }


    logging::log!("layer loading is working");
    Ok("test".to_string())
}

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let (username, set_username) = create_signal("".to_string());

    // spawn_local(
    //     async {
    //         set_username.update(axum_extract().await.unwrap());
    // });

    // let mut content = "".to_string();

    // set_username.update(content.unwrap());

    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future = axum_extract

            // the data is bound to whatever variable name you provide
            let:data
        >
            <p>{
                match data {
                    Ok(s) => set_username.set((*s).clone()),
                    Err(_) => set_username.set("".to_string()),
                }
            }</p>
        </Await>
        // <body on:load=move |_| {
        //     set_username.set("user".to_string());
        //     spawn_local(
        //         async move {
        //             axum_extract().await;
        //         });
        // }>

        <div class="contents">
            <div class="header">
                <table class="header-menu">
                <tr>
                    <td class="header">
                        <img src="images/logo.png"/>
                    </td>
                    <td class="header_menu">
                        <a href="/" class="header">首页</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">走进学校</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">课程中心</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">继续教育</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">师资力量</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">新闻中心</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">在线学习</a>
                    </td>
                    <td class="header_menu">
                        <a href="#" class="header">就业招聘</a>
                    </td>

                    <td class="header_login"><a href="/login" class="header" style="padding-top:10px;padding-bottom:10px;padding-left:20px;padding-right:20px;color:#FAFAFA;background-color: #333333;">登陆</a><b>{move || username.get()}</b></td>
                    <td class="header_login"><a href="/register" class="header" >注册</a></td>
                </tr>
                </table>
            </div>
        </div>
        <div><img src="images/banners/3.财务会计banner.jpg" class="banner"/></div>
        // </body>
    }
}
