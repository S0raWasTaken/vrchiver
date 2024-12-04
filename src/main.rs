#![warn(clippy::pedantic)]

use std::env::var;

use commands::commands;
use dotenv::dotenv;
use poise::{
    serenity_prelude::{ClientBuilder, CreateEmbed, GatewayIntents},
    CreateReply, Framework, FrameworkError, FrameworkOptions,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, (), Error>;

mod commands;

#[macro_export]
macro_rules! error {
    ($x:expr) => {{
        return Err($x.into());
    }};
}

/// When a command errors, it sends the full error to the command sender, with some formatting.
async fn on_error(error: FrameworkError<'_, (), Error>) {
    match error {
        FrameworkError::Command { error, ctx, .. } => {
            ctx.send(CreateReply {
                embeds: vec![CreateEmbed::new()
                    .title(format!("Error in command `/{}`", ctx.command().name))
                    .description(format!(
                        "```diff\n- {}```",
                        error.to_string().replace('\n', "\n- ").trim()
                    ))],
                ephemeral: Some(true),
                allowed_mentions: None,
                reply: true,
                ..Default::default()
            })
            .await
            .ok();
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                eprintln!("Error while... handling an error... oops\n\n{e}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().unwrap();

    let token = var("DISCORD_TOKEN").unwrap();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: commands(),
            on_error: |e| Box::pin(on_error(e)),
            ..Default::default()
        })
        .setup(move |ctx, ready, framework| {
            Box::pin(async move {
                println!("Logged as {}", ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                #[allow(clippy::unreadable_literal)]
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    1269281042737528884.into(),
                )
                .await?;
                Ok(())
            })
        })
        .build();

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
