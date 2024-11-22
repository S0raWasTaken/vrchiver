use poise::{serenity_prelude::CreateEmbed, CreateReply};

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

    ctx.send(CreateReply {
        embeds: vec![CreateEmbed::new()
            .title("[teste](https://youtube.com)")
            .description("[**>Source<**](https://github.com)\n[>Download<](https://youtube.com)")
            .field("name", "Velle", true)],
        ..Default::default()
    })
    .await?;
    Ok(())
}
