pub mod bot_config;
pub mod cdclient;
mod commands;
pub mod custom;
pub mod interaction_command;
pub mod locale;
pub mod queries;

use std::env;
use std::path::Path;
use std::time::Instant;

use bot_config::BotConfig;
use cdclient::CdClient;
use commands::level::LevelCommand;
use commands::preconditions::PreconditionsCommand;
use interaction_command::InteractionCommand;
use locale::LocaleXML;
use once_cell::sync::Lazy;
use serenity::all::{
    CreateActionRow, CreateAutocompleteResponse, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter,
};
use serenity::async_trait;
use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::model::application::{Command, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

static CD_CLIENT: Lazy<CdClient> =
    Lazy::new(|| CdClient::load_sqlite(Path::new("/home/dgmastertemple/cdclient.sqlite")).unwrap());

static LOCALE_XML: Lazy<LocaleXML> =
    Lazy::new(|| LocaleXML::load_xml(Path::new("/home/dgmastertemple/locale.xml")).unwrap());

static CONFIG: Lazy<BotConfig> = Lazy::new(|| BotConfig::default());

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Autocomplete(ref completion) = interaction {
            let Some(option) = completion.data.autocomplete() else {
                return;
            };
            let start = Instant::now();
            let content = match completion.data.name.as_str() {
                PreconditionsCommand::NAME => PreconditionsCommand::handle_autocomplete(option),
                _ => None,
            };
            let time = start.elapsed().as_millis();
            println!("Autocompletion query took {time}ms");
            if let Some(content) = content {
                let data = CreateAutocompleteResponse::new().set_choices(content);
                let builder = CreateInteractionResponse::Autocomplete(data);
                if let Err(why) = completion.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to auto-completion request: {why}");
                }
            }
        }

        if let Interaction::Command(ref command) = interaction {
            println!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                LevelCommand::NAME => Some(LevelCommand::handle_slash_command(command)),
                PreconditionsCommand::NAME => {
                    Some(PreconditionsCommand::handle_slash_command(command))
                }
                _ => None,
            };

            if let Some(content) = content {
                let builder = CreateInteractionResponse::Message(content);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }

        if let Interaction::Component(ref interaction) = interaction {
            println!("Received component interaction: {interaction:#?}");

            let custom_id = interaction.data.custom_id.as_str();
            let cmd = &custom_id[..custom_id.find(":").unwrap_or(0)];
            let content = match cmd {
                LevelCommand::NAME => Some(LevelCommand::handle_component_interaction(interaction)),
                PreconditionsCommand::NAME => Some(
                    PreconditionsCommand::handle_component_interaction(interaction),
                ),
                _ => None,
            };

            if let Some(content) = content {
                // let builder = CreateInteractionResponse::Message(content);
                let builder = CreateInteractionResponse::Message(content);
                if let Err(why) = interaction.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash interaction: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        {
            // force it to load
            let start = Instant::now();
            let levels = &CD_CLIENT.level_progression_lookup.len();
            println!("Total levels: {levels}");
            let end = start.elapsed().as_millis();
            println!("'cdclient.sqlite' loaded in {end}ms");
            // force it to load
            let start = Instant::now();
            let locale_count = &LOCALE_XML.locales.len();
            println!("Total locales: {locale_count}");
            let end = start.elapsed().as_millis();
            println!("'locale.xml' loaded in {end}ms");
        }

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![LevelCommand::register(), PreconditionsCommand::register()],
            )
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
