use poise::samples::HelpConfiguration;

use crate::{Context, Error};

/// It... helps!
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Help command"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(ctx, command.as_deref(), HelpConfiguration::default()).await?;
    Ok(())
}
