use cfg_if::cfg_if;
use leptos::*;
use leptos_router::*;

cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::{
        http::header::{SET_COOKIE},
        http::{HeaderMap, HeaderValue},
    };
    use leptos_axum::{ResponseParts};

}}

#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    let logout = create_server_action::<DestroySession>(cx);

    view! { cx,
        <ActionForm action=logout>
            <input class="logout-button" type="submit" value="Logout"/>
        </ActionForm>
    }
}

#[server(DestroySession, "/api")]
pub async fn server_destroy_session(cx: Scope) -> Result<(), ServerFnError> {
    destroy_session(cx);
    Ok(())
}

#[cfg(feature = "ssr")]
fn destroy_session(cx: Scope) {
    log::trace!("user logged out");
    let response = match use_context::<leptos_axum::ResponseOptions>(cx) {
        Some(rp) => rp, // actual user request
        None => return, // no request, building routes in main.rs
    };
    let mut response_parts = ResponseParts::default();
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        HeaderValue::from_str(
            "SESSIONID=deleted; Expires=Thu, 01-Jan-1970 00:00:01 GMT; Max-Age=0; Secure; SameSite=Lax; HttpOnly; Path=/"
        )
        .expect("to create header value"),
    );
    response_parts.headers = headers;
    response.overwrite(response_parts);
}
