use leptos::*;

#[component]
pub fn QRLayer() -> impl IntoView {
    view! {
        <p>微信扫描二维码登陆</p>
        <img src="images/winxinlogo.png" />
        <img style="width:80%;height:auto" src="images/QR/showQrCode.png" />
    }
}
