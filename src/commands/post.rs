use super::send_reply;
use crate::{Context, Error};
use poise::{
    serenity_prelude::{
        json::{from_str, Value},
        Attachment, ChannelId, CreateEmbed, CreateEmbedFooter, CreateMessage,
    },
    CreateReply,
};
use tokio::time::Instant;
/*[
    {
        "name": "Hu Tao",
        "img": "https://booth.pximg.net/a81c9bc6-f795-48fa-8e85-87ea0f567656/i/4175623/addcfc8c-e460-4def-9d1f-ebc666f4e7fb_base_resized.jpg",
        "source": "https://vrc-booth.com/en/product/4175623",
        "assetType": "Avatar",
        "avatarName": [],
        "comment": "",
        "id": "REDACTED"
    },
    {
        "name": "❰Hair❱ 7アバター対応 Hair_003 Hime Cut PACK",
        "img": "https://booth.pximg.net/0f0637a8-9966-43f3-8e0b-40233300656a/i/5908294/b1adc1d7-bc57-46e7-9c01-58f95ae120ea_base_resized.jpg",
        "source": "https://booth.pm/en/items/5908294",
        "download": "REDACTED",
        "avatarName": [],
        "comment":""
    }
]*/
#[allow(clippy::too_many_arguments)]
#[poise::command(slash_command)]
/// Submits an asset for posting
pub async fn submit(
    ctx: Context<'_>,
    #[description = "The asset name"] name: String,
    #[description = "A valid image URL"] image: String,
    #[description = "Original source"] source: String,
    #[description = "Which bases does this asset support?"] bases_supported: Option<String>,
    #[description = "A password, extra info, etc"] comment: Option<String>,
    #[description = "Direct download link"] download: String,
    #[description = "In which channel does this asset fit?"] channel: ChannelId,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let embed = CreateEmbed::new()
        .image(image)
        .title(name)
        .description(format!(
            "[**>Source<**]({source})\n[**>Download<**]({download})"
        ))
        .footer(CreateEmbedFooter::new(comment.unwrap_or_default()));

    let embed = if let Some(bases) = bases_supported {
        embed.field("Avatars:", format!("`{bases}`"), false)
    } else {
        embed
    };

    channel
        .send_message(ctx.http(), CreateMessage::new().add_embed(embed))
        .await?;

    send_reply(ctx, "Done!").await?;
    Ok(())
}

/// Starts posting the embeds in this channel, using the provided JSON file
#[poise::command(slash_command)]
pub async fn post(
    ctx: Context<'_>,
    #[description = "JSON file in the expected structure"] json: Attachment,
) -> Result<(), Error> {
    let time = Instant::now();
    let content = String::from_utf8(json.download().await?)?;
    let values: Value = from_str(content)?;

    let mut embeds = Vec::new();
    for value in values
        .as_array()
        .ok_or("Expected an array of json objects: [{}, {}]")?
    {
        embeds.push(build_embed(value));
    }

    for chunk in embeds.chunks(10) {
        ctx.send(CreateReply {
            embeds: chunk.to_vec(),
            reply: false,
            ..Default::default()
        })
        .await?;
    }

    send_reply(ctx, &format!("```Finished in {:.2?}```", time.elapsed())).await?;
    Ok(())
}

fn build_embed(json: &Value) -> CreateEmbed {
    let name = json["name"].as_str().unwrap_or_default();
    let img = json["img"].as_str().unwrap_or_default();
    let source = json["source"].as_str().unwrap_or_default();
    let download = json["download"].as_str().unwrap_or_default();
    let avatar_name = json["avatarName"].as_array();
    let comment = json["comment"].as_str().unwrap_or_default();
    let asset_type = json["assetType"].as_str().unwrap_or_default();
    let id = json["id"].as_str().unwrap_or_default();

    // Parse NORMAL JSON
    let avatar_names = avatar_name
        .unwrap_or(&vec![])
        .iter()
        .map(|entry| entry.as_str().unwrap_or_default())
        .collect::<Vec<&str>>()
        .join("`, `");

    let embed = if id.is_empty() {
        // NORMAL
        CreateEmbed::new()
            .image(img)
            .title(name)
            .description(format!(
                "[**>Source<**]({source})\n[**>Download<**]({download})"
            ))
            .footer(CreateEmbedFooter::new(comment))
    } else {
        // WANT
        CreateEmbed::new()
            .image(img)
            .title(name)
            .description(format!(
                "[**>Source<**]({})",
                json["source"].as_str().unwrap_or_default(),
            ))
            .field("Asset Type", format!("`{asset_type}`"), true)
            .footer(CreateEmbedFooter::new(
                json["comment"].as_str().unwrap_or_default(),
            ))
    };

    if avatar_names.is_empty() {
        embed
    } else {
        embed.field("Avatars:", format!("`{avatar_names}`"), false)
    }
}
