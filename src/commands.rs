use std::env::var;
use std::time::Duration;
use crate::{Context, Error};
use mcping;
use poise::serenity_prelude::colours::roles::{BLUE};

#[poise::command(prefix_command, slash_command)]
pub async fn status(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let server_ip = var("MC_SERVER_IP").expect("Server IP missing");

    let (ping, server_information) = tokio::task::spawn_blocking(move || {
        mcping::get_status(&server_ip, Duration::from_secs(10)).expect("Could not ping the server.")
    }).await.expect("Task panicked");


    let player_names = server_information
        .players
        .sample
        .expect("Coudln't find players")
        .into_iter()
        .map(|x| x.name)
        .collect::<Vec<_>>()
        .join(", ");


    let embed = poise::serenity_prelude::CreateEmbed::default().color(BLUE)
        .title("Sexybabeycraft Status")
        .description(server_information.description.text().to_string())
        .field("", format!("**Ping**: `{ping}`"), false)
        .field("", format!("**Players**: `{player_names}`"), false);

    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn ip(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let server_ip = var("MC_SERVER_IP").expect("Server IP missing");

    let embed = poise::serenity_prelude::CreateEmbed::default().color(BLUE)
        .title("Sexybabeycraft IP")
        .field("", format!("**IP**: `{server_ip}`"), false);

    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn maps(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let maps_address = var("MAPS_ADDRESS").expect("Server maps address missing");

    let embed = poise::serenity_prelude::CreateEmbed::default().color(BLUE)
        .title("Sexybabeycraft Maps")
        .field("", format!("**Server Map**: {maps_address}"), false);

    let reply = poise::CreateReply::default().embed(embed);

    ctx.send(reply).await?;
    Ok(())
}
