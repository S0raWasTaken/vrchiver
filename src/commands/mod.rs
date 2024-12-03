use std::future::Future;

use poise::{serenity_prelude::CreateEmbed, CreateReply};

use crate::{Context, Error};

mod counter;
mod help;
mod ping;
mod post;

pub fn commands() -> Vec<poise::Command<(), Error>> {
    vec![
        ping::ping(),
        help::help(),
        post::post(),
        post::submit(),
        counter::count_messages(),
    ]
}

#[inline]
pub fn send_reply<'a>(
    ctx: Context<'a>,
    reply: &str,
) -> impl Future<Output = Result<poise::ReplyHandle<'a>, poise::serenity_prelude::Error>> {
    ctx.send(CreateReply {
        embeds: vec![CreateEmbed::new().description(reply)],
        ..Default::default()
    })
}
