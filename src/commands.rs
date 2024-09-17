use std::env::var;
use std::time::Duration;
use crate::{Context, Error};
use mcping;
use poise::serenity_prelude::Color;
use poise::serenity_prelude::colours::roles::{BLUE};

#[poise::command(prefix_command, slash_command)]
pub async fn status(
    ctx: Context<'_>,
) -> Result<(), Error> {

    let server_ip = var("MC_SERVER_IP").expect("Server IP missing");

    let embed = match tokio::task::spawn_blocking(move || {
        mcping::get_status(&server_ip, Duration::from_secs(10))
    }).await? {
        Ok((ping, server_information)) => {
            let (player_count, player_names): (usize, String) = match server_information.players.sample {
                Some(s) => {
                    let player_vec = s.into_iter().map(|x| x.name).collect::<Vec<_>>();
                    (player_vec.len(), player_vec.join(", "))
                },
                None => (0, String::from("None"))
            };
            poise::serenity_prelude::CreateEmbed::default().color(Color::new(6553467))
                .title("Sexybabeycraft Status")
                .thumbnail("https://cdn.vaughn.sh/icon-zHLcOQop.png")
                .description(server_information.description.text().to_string())
                .field("**Ping:**", format!("`{ping}`"), false)
                .field(format!("**Active Players ({player_count}/10):**"), format!("`{player_names}`"), false)
        }
        Err(_) => {
            poise::serenity_prelude::CreateEmbed::default().color(BLUE)
                .title("Sexybabeycraft Status")
                .description("cc: <@173232081575346178>")
                .field("", "Uh oh! Looks like the server is down.", false)
        }
    };

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
