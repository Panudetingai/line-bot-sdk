#![deny(clippy::all)]

mod api;
mod client;
mod services;
mod types;
mod webhook;

pub use client::LineClient;
pub use webhook::{parse_webhook_body, verify_webhook_signature};

// Message types
pub use types::message::{
  AudioMessage, ImageMessage, LocationMessage, MessageQuota, MessageQuotaConsumption,
  PushMessageRequest, QuickReply, QuickReplyAction, QuickReplyItem, ReplyMessageRequest, Sender,
  StickerMessage, TextMessage, VideoMessage,
};

// Profile & Group
pub use types::group::{FollowerIds, GroupMemberProfile, GroupSummary, MemberIds};
pub use types::profile::Profile;

// Rich menu
pub use types::richmenu::{
  RichMenu, RichMenuAction, RichMenuArea, RichMenuBounds, RichMenuIdResponse, RichMenuList,
  RichMenuResponse, RichMenuSize,
};

// Webhook events
pub use types::webhook::{
  BeaconContent, ContentProvider, DeliveryContext, EventMessage, EventSource, LinkContent,
  PostbackContent, PostbackParams, WebhookBody, WebhookEvent,
};

// Insight
pub use types::insight::{FollowersInsight, MessageDeliveryOverview};
