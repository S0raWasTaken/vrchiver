use crate::Error;

mod help;
mod ping;
mod post;

pub fn commands() -> Vec<poise::Command<(), Error>> {
    vec![ping::ping(), help::help(), post::post()]
}
