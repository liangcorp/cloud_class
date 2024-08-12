use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/cloud-class.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=LoginPage/>
                </Routes>
            </main>
        </Router>
    }
}

/// 提供登陆页
#[component]
fn LoginPage() -> impl IntoView {
    // 制作一个reactive值去更新提交按钮
    let (username, set_username) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());

    let input_username: NodeRef<html::Input> = create_node_ref();
    let input_password: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let username_value = input_username
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_username.set(username_value);

        let password_value = input_password
            .get()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_password.set(password_value);
    };

    view! {
        <form on:submit=on_submit> // on_submit defined below
            <table class="login">
                // <tr><td><p>"用户名是: " {username}</p></td></tr>
                <tr><td style="padding:25px"><h2 style="color:white">浩天数智化教学辅助系统</h2></td></tr>
                <tr style="display:none;color:#FF1E47">
                <td>
                <h4>用户名或者密码不正确</h4>
                </td>
                </tr>
                <tr>
                <td style="padding:10px"><b style="padding:10px;border:0px;background-color:#40BFE3">
                    账号<input placeholder="请输入账号" style="border:0px;background-color:#40BFE3;color:white" type="text"
                        value=username
                        node_ref=input_username
                    />

                </b></td>
                </tr>
                <tr><td></td></tr>
                <tr>
                <td style="padding:10px"><b style="padding:10px;border:0px;background-color:#40BFE3">
                    密码<input placeholder="请输入密码" style="border:0px;background-color:#40BFE3;color:white" type="password"
                        value=password
                        node_ref=input_password
                        />
                </b></td>
                </tr>
                <tr>
                <td style="padding:10px">
                    <input style="border:0px;background-color:#0383AA;color:white;font-size:20px" type="submit" value="登陆"/>
                </td>
                </tr>
            </table>
        </form>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
