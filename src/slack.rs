// Custom Slack functionality.
use serde::{Deserialize, Serialize};
use std::env;

// Calls to users_info return the following.
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct UserWrapper {
    ok: bool,
    user: Option<User>,
    error: Option<String>,
}

// All available user info, see https://api.slack.com/methods/users.info.
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct User {
    pub(crate) id: String,
    pub(crate) team_id: String,
    pub(crate) name: String,
    pub(crate) real_name: String,
    pub(crate) tz: Option<String>,
    pub(crate) tz_label: Option<String>,
    pub(crate) tz_offset: Option<i32>,
    pub(crate) profile: Profile,
    pub(crate) is_admin: bool,
    pub(crate) is_owner: bool,
    pub(crate) is_restricted: bool,
    pub(crate) is_ultra_restricted: bool,
    pub(crate) is_bot: bool,
    pub(crate) updated: u32,
    pub(crate) is_app_user: bool,
    pub(crate) has_2fa: Option<bool>,
}

// Profile information included about user.
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Profile {
    pub(crate) status_text: String,
    pub(crate) status_emoji: String,
    pub(crate) real_name: String,
    pub(crate) display_name: String,
    pub(crate) real_name_normalized: String,
    pub(crate) display_name_normalized: String,
    pub(crate) email: Option<String>,
    pub(crate) team: String,
}

// Calls to channel_info return the following.
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct ChannelWrapper {
    ok: bool,
    channel: Option<Channel>,
    error: Option<String>,
}

// All available user info, see https://api.slack.com/methods/conversations.info
#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Channel {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) is_channel: Option<bool>,
    pub(crate) is_group: Option<bool>,
    pub(crate) is_im: Option<bool>,
    pub(crate) created: u32,
    pub(crate) creator: String,
    pub(crate) is_archived: bool,
    pub(crate) is_general: bool,
    pub(crate) name_normalized: String,
    pub(crate) is_read_only: Option<bool>,
    pub(crate) is_shared: bool,
    pub(crate) is_ext_shared: bool,
    pub(crate) is_org_shared: bool,
    pub(crate) is_member: bool,
    pub(crate) is_private: bool,
    pub(crate) is_mpim: bool,
    pub(crate) last_read: String,
    pub(crate) locale: Option<String>,
    pub(crate) topic: ChannelDetail,
    pub(crate) purpose: ChannelDetail,
    pub(crate) previous_names: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct ChannelDetail {
    value: String,
    creator: String,
    last_set: u32,
}

// Get full details about a user by id.
pub(crate) async fn users_info(user_id: &str) -> Result<User, String> {
    let slack_bot_token = env::var("SLACK_BOT_TOKEN")
        .unwrap_or_else(|_| panic!("slack bot token is not set (starts with 'xoxb')."));

    let user_wrapper: UserWrapper =
        match surf::post(format!("https://slack.com/api/users.info?user={}", user_id))
            .header("Authorization", format!("Bearer {}", slack_bot_token))
            .recv_json()
            .await
        {
            Ok(user_wrapper) => user_wrapper,
            Err(e) => return Err(e.to_string()),
        };

    // No need to check `ok`, just check if the user exists.
    if let Some(user) = user_wrapper.user {
        Ok(user)
    // Otherwise we got an error.
    } else if let Some(error) = user_wrapper.error {
        Err(error)
    } else {
        // Debug output if somehow this happened:
        log::error!("user_wrapper: {:#?}", user_wrapper);
        unreachable!("No user and no error!?");
    }
}

// Get full details about a channel by id.
pub(crate) async fn channels_info(channel_id: &str) -> Result<Channel, String> {
    let slack_bot_token = env::var("SLACK_BOT_TOKEN")
        .unwrap_or_else(|_| panic!("slack bot token is not set (starts with 'xoxb')."));

    let channel_wrapper: ChannelWrapper = match surf::post(format!(
        "https://slack.com/api/conversations.info?channel={}",
        channel_id
    ))
    .header("Authorization", format!("Bearer {}", slack_bot_token))
    .recv_json()
    .await
    {
        Ok(channel_wrapper) => channel_wrapper,
        Err(e) => return Err(e.to_string()),
    };

    // No need to check `ok`, just check if the channel exists.
    if let Some(channel) = channel_wrapper.channel {
        Ok(channel)
    // Otherwise we got an error.
    } else if let Some(error) = channel_wrapper.error {
        Err(error)
    } else {
        // Debug output if somehow this happened:
        log::error!("channel_wrapper: {:#?}", channel_wrapper);
        unreachable!("No channel and no error!?");
    }
}
