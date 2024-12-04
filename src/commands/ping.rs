use poise::CreateReply;

use crate::{Context, Error};

/// It works :+1:
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let msg = ctx.reply("Pinging...").await?;
    msg.edit(
        ctx,
        CreateReply {
            content: Some(format!("{:.1?}", ctx.ping().await)),
            ..Default::default()
        },
    )
    .await?;
    Ok(())
}
