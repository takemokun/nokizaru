//! Slack API クライアントモジュール
//!
//! このモジュールは、Slack Web APIとの通信を担当する独立したレイヤーです。
//!
//! ## アーキテクチャ
//!
//! ```text
//! ┌─────────────────────────────────────────┐
//! │ infrastructure/client.rs                │
//! │ - ドメイン層とのブリッジ                │
//! │ - エラー型の変換                        │
//! └──────────────┬──────────────────────────┘
//!                │
//!                ▼
//! ┌─────────────────────────────────────────┐
//! │ slack_api/api.rs                        │
//! │ - Slack APIメソッド層                   │
//! │ - 型推論による自動レスポンス変換        │
//! └──────────────┬──────────────────────────┘
//!                │
//!                ▼
//! ┌─────────────────────────────────────────┐
//! │ slack_api/client.rs                     │
//! │ - 汎用HTTPクライアント                  │
//! │ - ジェネリック型パラメータ              │
//! └─────────────────────────────────────────┘
//! ```
//!
//! ## 型推論の仕組み
//!
//! このモジュールの設計では、Rustの型推論を活用して
//! 簡潔で型安全なAPIを提供しています。
//!
//! ```rust,ignore
//! // API層での使用例
//! pub async fn get_channel_history(...) -> ClientResult<Vec<SlackHistoryMessage>> {
//!     // 型注釈により RS = ConversationsHistoryResponse と推論される
//!     let response: ConversationsHistoryResponse = self
//!         .client
//!         .http_get("conversations.history", &params)
//!         .await?;
//!
//!     Ok(response.messages)
//! }
//! ```
//!
//! ## 独立性
//!
//! このモジュールは `domain` や `infrastructure` とは独立しており、
//! 将来的に他のプロジェクトへの移行や再利用が容易な設計になっています。

pub mod client;
pub mod api;
pub mod error;

pub use api::*;
