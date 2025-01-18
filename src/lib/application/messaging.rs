use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::domain::member::events::GuildCreateEvent;

use super::ports::messaging_ports::{MessagingPort, MessagingTypeImpl};

pub async fn subscribe_to_guild_created(messaging: Arc<MessagingTypeImpl>) -> Result<()> {
    let messaging = Arc::clone(&messaging);

    messaging
        .subscribe("guild-created", "guild-created-member", {
            move |e: GuildCreateEvent| {
                info!("Received guild created event: {:?}", e);
                async move { Ok(()) }
            }
        })
        .await?;

    Ok(())
}
