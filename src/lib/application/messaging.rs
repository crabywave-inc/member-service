use std::sync::Arc;

use anyhow::Result;
use tracing::info;

use crate::domain::member::{events::GuildCreateEvent, ports::MemberService};

use super::ports::messaging_ports::{MessagingPort, MessagingTypeImpl};

pub async fn subscribe_to_guild_created<M>(
    messaging: Arc<MessagingTypeImpl>,
    member_service: Arc<M>,
) -> Result<()>
where
    M: MemberService,
{
    let messaging = Arc::clone(&messaging);

    messaging
        .subscribe("guild-created", "guild-created-member", {
            move |e: GuildCreateEvent| {
                let member_service = Arc::clone(&member_service);

                async move {
                    let member = member_service.create(e.owner_id, e.id).await?;

                    info!("Member created: {:?}", member);

                    Ok(())
                }
            }
        })
        .await?;

    Ok(())
}
