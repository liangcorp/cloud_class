use leptos::*;

#[component]
pub fn QRLayer() -> impl IntoView {
    view! {
        <p>微信扫描二维码登陆</p>
        <img src="images/winxinlogo.png" />
        <img src="images/QR/showQrCode.png" />
    }
}
