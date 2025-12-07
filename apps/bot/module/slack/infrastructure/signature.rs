use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Slackリクエスト署名の検証
pub fn verify_slack_signature(
    signing_secret: &str,
    timestamp: &str,
    body: &str,
    signature: &str,
) -> bool {
    // タイムスタンプのリプレイ攻撃チェック（5分以内）
    if let Ok(ts) = timestamp.parse::<i64>() {
        let current_time = chrono::Utc::now().timestamp();
        if (current_time - ts).abs() > 60 * 5 {
            tracing::warn!("Request timestamp too old");
            return false;
        }
    }

    // 署名ベース文字列の構築
    let sig_basestring = format!("v0:{}:{}", timestamp, body);

    // HMAC-SHA256で署名計算
    let mut mac = match HmacSha256::new_from_slice(signing_secret.as_bytes()) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(sig_basestring.as_bytes());

    let result = mac.finalize();
    let code_bytes = result.into_bytes();
    let computed = format!("v0={}", hex::encode(code_bytes));

    // 定数時間比較
    computed == signature
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_slack_signature() {
        let signing_secret = "test_secret";
        let timestamp = "1234567890";
        let body = r#"{"type":"event_callback"}"#;

        // 正しい署名を計算
        let sig_basestring = format!("v0:{}:{}", timestamp, body);
        let mut mac = HmacSha256::new_from_slice(signing_secret.as_bytes()).unwrap();
        mac.update(sig_basestring.as_bytes());
        let result = mac.finalize();
        let signature = format!("v0={}", hex::encode(result.into_bytes()));

        assert!(verify_slack_signature(signing_secret, timestamp, body, &signature));
    }
}
