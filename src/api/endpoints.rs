pub const BASE_URL: &str = "https://api.line.me";
pub const BASE_DATA_URL: &str = "https://api-data.line.me";

// ── Bot ──────────────────────────────────────────────────────────────────────
pub fn bot_info() -> String {
  format!("{BASE_URL}/v2/bot/info")
}

// ── Messaging ────────────────────────────────────────────────────────────────
pub fn push_message() -> String {
  format!("{BASE_URL}/v2/bot/message/push")
}

pub fn reply_message() -> String {
  format!("{BASE_URL}/v2/bot/message/reply")
}

pub fn multicast_message() -> String {
  format!("{BASE_URL}/v2/bot/message/multicast")
}

pub fn broadcast_message() -> String {
  format!("{BASE_URL}/v2/bot/message/broadcast")
}

pub fn narrowcast_message() -> String {
  format!("{BASE_URL}/v2/bot/message/narrowcast")
}

pub fn message_quota() -> String {
  format!("{BASE_URL}/v2/bot/message/quota")
}

pub fn message_quota_consumption() -> String {
  format!("{BASE_URL}/v2/bot/message/quota/consumption")
}

pub fn message_content(message_id: &str) -> String {
  format!("{BASE_DATA_URL}/v2/bot/message/{message_id}/content")
}

pub fn message_content_preview(message_id: &str) -> String {
  format!("{BASE_DATA_URL}/v2/bot/message/{message_id}/content/preview")
}

// ── Profile ──────────────────────────────────────────────────────────────────
pub fn profile(user_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/profile/{user_id}")
}

pub fn followers_ids() -> String {
  format!("{BASE_URL}/v2/bot/followers/ids")
}

// ── Group ────────────────────────────────────────────────────────────────────
pub fn group_summary(group_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/group/{group_id}/summary")
}

pub fn group_members_count(group_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/group/{group_id}/members/count")
}

pub fn group_member_profile(group_id: &str, user_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/group/{group_id}/member/{user_id}")
}

pub fn group_member_ids(group_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/group/{group_id}/members/ids")
}

pub fn leave_group(group_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/group/{group_id}/leave")
}

// ── Room ─────────────────────────────────────────────────────────────────────
pub fn room_member_profile(room_id: &str, user_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/room/{room_id}/member/{user_id}")
}

pub fn room_member_ids(room_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/room/{room_id}/members/ids")
}

pub fn leave_room(room_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/room/{room_id}/leave")
}

// ── Rich Menu ────────────────────────────────────────────────────────────────
pub fn rich_menu_create() -> String {
  format!("{BASE_URL}/v2/bot/richmenu")
}

pub fn rich_menu(rich_menu_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/richmenu/{rich_menu_id}")
}

pub fn rich_menu_list() -> String {
  format!("{BASE_URL}/v2/bot/richmenu/list")
}

pub fn rich_menu_default() -> String {
  format!("{BASE_URL}/v2/bot/user/all/richmenu")
}

pub fn rich_menu_default_get() -> String {
  format!("{BASE_URL}/v2/bot/user/all/richmenu")
}

pub fn rich_menu_image_upload(rich_menu_id: &str) -> String {
  format!("{BASE_DATA_URL}/v2/bot/richmenu/{rich_menu_id}/content")
}

pub fn rich_menu_image_download(rich_menu_id: &str) -> String {
  format!("{BASE_DATA_URL}/v2/bot/richmenu/{rich_menu_id}/content")
}

pub fn user_rich_menu(user_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/user/{user_id}/richmenu")
}

pub fn link_user_rich_menu(user_id: &str, rich_menu_id: &str) -> String {
  format!("{BASE_URL}/v2/bot/user/{user_id}/richmenu/{rich_menu_id}")
}

pub fn bulk_link_rich_menu() -> String {
  format!("{BASE_URL}/v2/bot/richmenu/bulk/link")
}

pub fn bulk_unlink_rich_menu() -> String {
  format!("{BASE_URL}/v2/bot/richmenu/bulk/unlink")
}

// ── Insight ──────────────────────────────────────────────────────────────────
pub fn insight_message_delivery(date: &str) -> String {
  format!("{BASE_URL}/v2/bot/insight/message/delivery?date={date}")
}

pub fn insight_followers(date: &str) -> String {
  format!("{BASE_URL}/v2/bot/insight/followers?date={date}")
}

pub fn insight_demographic() -> String {
  format!("{BASE_URL}/v2/bot/insight/demographic")
}

// ── Webhook ──────────────────────────────────────────────────────────────────
pub fn webhook_endpoint() -> String {
  format!("{BASE_URL}/v2/bot/channel/webhook/endpoint")
}
