pub mod cdclient;
mod commands;
pub mod custom;
pub mod interaction_command;
pub mod locale;
pub mod queries;

use std::env;
use std::path::Path;
use std::time::Instant;

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

pub struct BotConfig {
    pub locale: String,
    pub explorer_url: String,
    pub explorer_res_url: String,
    pub author_url: Option<String>,
    pub author_name: Option<String>,
    pub author_icon: Option<String>,
    pub color: Option<String>,
    pub footer_message: Option<String>,
    pub footer_icon: Option<String>,
}

fn join_paths(front: &str, back: &str) -> String {
    let front = front.strip_suffix("/").unwrap_or(&front);
    let back = back.strip_prefix("/").unwrap_or(&back);
    format!("{}/{}", front, back)
}

impl BotConfig {
    pub fn explorer_uri(&self, path: &str) -> String {
        join_paths(&self.explorer_url, path)
    }

    pub fn explorer_res_uri(&self, path: &str) -> String {
        join_paths(&self.explorer_res_url, path)
            .replace(" ", "%20")
            .to_lowercase()
            .replace(".dds", ".png")
    }
}

impl BotConfig {
    pub fn default_embed(&self) -> CreateEmbed {
        let author = self.author_name.as_ref().map(|name| {
            let mut author = CreateEmbedAuthor::new(name);
            if let Some(ref author_icon) = self.author_icon {
                author = author.icon_url(author_icon);
            }
            if let Some(ref author_url) = self.author_url {
                author = author.url(author_url);
            }
            author
        });
        let footer = self.footer_message.as_ref().map(|msg| {
            let mut footer = CreateEmbedFooter::new(msg);
            if let Some(ref icon_url) = self.footer_icon {
                footer = footer.icon_url(icon_url);
            }
            footer
        });
        let mut embed = CreateEmbed::new();
        // embed = embed.color((0x42, 0xb9, 0xf5));
        embed = embed.color((0x1a, 0x87, 0xe8));
        // if let Some(color) = self.color {
        //     embed = embed.color(color);
        // }
        if let Some(author) = author {
            embed = embed.author(author);
        }
        if let Some(footer) = footer {
            embed = embed.footer(footer);
        }
        embed
    }

    pub fn error_msg(&self, msg: impl Into<String>) -> (CreateEmbed, Option<Vec<CreateActionRow>>) {
        let embed = self
            .default_embed()
            .title("Error")
            .description(msg)
            .color((0xff, 0x00, 0x00));
        // let response = CreateInteractionResponseMessage::new().embed(embed);
        // response
        (embed, None)
    }
}

static CONFIG: Lazy<BotConfig> = Lazy::new(|| {
    BotConfig {
        locale: String::from("en_US"),
        explorer_url: String::from("https://explorer.lu/"),
        explorer_res_url: String::from("https://explorer.lu/lu-res/"),
        author_url: Some(String::from("https://github.com/MasterTemple/Crab-Force")),
        author_name: Some(String::from("Crab Force")),
        // author_icon: Some(String::from("https://explorer.lu/lu-res/textures/ui/inventory/models/amb_crab.png")),
        author_icon: Some(String::from("https://cdn.discordapp.com/avatars/1340084890342785055/4c83403b3a82920365a5007c1aa580ec.webp")),
        color: None, //(0x42, 0xb9, 0xf5),
        footer_message: Some(String::from("LEGO® is a trademark of the LEGO Group which does not sponsor, authorize, or endorse this bot. The data and assets are presented purely for informational purposes.")),
        footer_icon: Some(String::from("https://cdn.discordapp.com/attachments/813618981247516715/1339979649328877627/170px-LEGO_logo.png?ex=67b0b0cc&is=67af5f4c&hm=7e9d7b9258682dae296a525bc2fb46a7835a3b9ebefbe5cc192519c32cd66402&format=webp&quality=lossless")),
    }
});

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
                vec![
                    LevelCommand::register(),
                    PreconditionsCommand::register(),
                    // commands::ping::register(),
                    // commands::id::register(),
                    // commands::level::register(),
                    // commands::welcome::register(),
                    // commands::numberinput::register(),
                    // commands::attachmentinput::register(),
                    // commands::modal::register(),
                ],
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
