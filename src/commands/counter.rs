use poise::serenity_prelude::futures::StreamExt;

use super::send_reply;
use crate::{Context, Error};

/// Counts messages in a channel
#[poise::command(slash_command)]
pub async fn count_messages(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let count = ctx.channel_id().messages_iter(ctx.http()).count().await;

    send_reply(ctx, &format!("```{count} messages```")).await?;
    Ok(())
}
