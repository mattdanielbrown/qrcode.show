use axum_server;
use tower::Service;
use worker::*;

mod utils;

#[event(fetch)]
async fn fetch(
    req: worker::HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::response::Response> {
    utils::set_panic_hook();

    let mut router = axum_server::router();
    let response = router.call(req).await?;

    Ok(response)
}
