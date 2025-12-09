use axum::{
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use nokizaru_core::module::slack::verify_slack_signature;

/// Slack署名検証ミドルウェア
pub async fn verify_signature_middleware(
    headers: HeaderMap,
    signing_secret: String,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // ヘッダーから署名情報取得
    let timestamp = headers
        .get("x-slack-request-timestamp")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let signature = headers
        .get("x-slack-signature")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // リクエストボディを取得
    let (parts, body) = request.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let body_str = std::str::from_utf8(&bytes)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // 署名検証
    if !verify_slack_signature(&signing_secret, timestamp, body_str, signature) {
        tracing::warn!("Invalid Slack signature");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // リクエストを再構築
    let request = Request::from_parts(parts, Body::from(bytes));
    Ok(next.run(request).await)
}
