pub mod bot_config;
pub mod cdclient;
mod commands;
pub mod custom;
pub mod interaction_command;
pub mod locale;
pub mod queries;
pub mod repeat;

use std::env;
use std::path::Path;
use std::time::Instant;

use bot_config::BotConfig;
use cdclient::CdClient;
use commands::achievement::AchievementCommand;
use commands::activity::ActivityCommand;
use commands::brick::BrickCommand;
use commands::buy::BuyCommand;
use commands::cooldowngroup::CooldownGroupCommand;
use commands::drop::DropCommand;
use commands::earn::EarnCommand;
use commands::enemy::EnemyCommand;
use commands::get::GetCommand;
use commands::item::ItemCommand;
use commands::level::LevelCommand;
use commands::loottable::LootTableCommand;
use commands::mission::MissionCommand;
use commands::npc::NpcCommand;
use commands::package::PackageCommand;
use commands::preconditions::PreconditionsCommand;
use commands::reward::RewardCommand;
use commands::skill::SkillCommand;
use commands::skillitems::SkillItemsCommand;
use commands::skills::SkillsCommand;
use commands::smash::SmashCommand;
use commands::unpack::UnpackCommand;
use commands::vendor::VendorCommand;
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
                AchievementCommand::NAME => AchievementCommand::handle_autocomplete(option),
                ActivityCommand::NAME => ActivityCommand::handle_autocomplete(option),
                BrickCommand::NAME => BrickCommand::handle_autocomplete(option),
                BuyCommand::NAME => BuyCommand::handle_autocomplete(option),
                CooldownGroupCommand::NAME => CooldownGroupCommand::handle_autocomplete(option),
                DropCommand::NAME => DropCommand::handle_autocomplete(option),
                EarnCommand::NAME => EarnCommand::handle_autocomplete(option),
                EnemyCommand::NAME => EnemyCommand::handle_autocomplete(option),
                GetCommand::NAME => GetCommand::handle_autocomplete(option),
                ItemCommand::NAME => ItemCommand::handle_autocomplete(option),
                LevelCommand::NAME => LevelCommand::handle_autocomplete(option),
                LootTableCommand::NAME => LootTableCommand::handle_autocomplete(option),
                MissionCommand::NAME => MissionCommand::handle_autocomplete(option),
                NpcCommand::NAME => NpcCommand::handle_autocomplete(option),
                PackageCommand::NAME => PackageCommand::handle_autocomplete(option),
                PreconditionsCommand::NAME => PreconditionsCommand::handle_autocomplete(option),
                RewardCommand::NAME => RewardCommand::handle_autocomplete(option),
                SkillCommand::NAME => SkillCommand::handle_autocomplete(option),
                SkillItemsCommand::NAME => SkillItemsCommand::handle_autocomplete(option),
                SkillsCommand::NAME => SkillsCommand::handle_autocomplete(option),
                SmashCommand::NAME => SmashCommand::handle_autocomplete(option),
                UnpackCommand::NAME => UnpackCommand::handle_autocomplete(option),
                VendorCommand::NAME => VendorCommand::handle_autocomplete(option),
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
                AchievementCommand::NAME => Some(AchievementCommand::handle_slash_command(command)),
                ActivityCommand::NAME => Some(ActivityCommand::handle_slash_command(command)),
                BrickCommand::NAME => Some(BrickCommand::handle_slash_command(command)),
                BuyCommand::NAME => Some(BuyCommand::handle_slash_command(command)),
                CooldownGroupCommand::NAME => {
                    Some(CooldownGroupCommand::handle_slash_command(command))
                }
                DropCommand::NAME => Some(DropCommand::handle_slash_command(command)),
                EarnCommand::NAME => Some(EarnCommand::handle_slash_command(command)),
                EnemyCommand::NAME => Some(EnemyCommand::handle_slash_command(command)),
                GetCommand::NAME => Some(GetCommand::handle_slash_command(command)),
                ItemCommand::NAME => Some(ItemCommand::handle_slash_command(command)),
                LevelCommand::NAME => Some(LevelCommand::handle_slash_command(command)),
                LootTableCommand::NAME => Some(LootTableCommand::handle_slash_command(command)),
                MissionCommand::NAME => Some(MissionCommand::handle_slash_command(command)),
                NpcCommand::NAME => Some(NpcCommand::handle_slash_command(command)),
                PackageCommand::NAME => Some(PackageCommand::handle_slash_command(command)),
                PreconditionsCommand::NAME => {
                    Some(PreconditionsCommand::handle_slash_command(command))
                }
                RewardCommand::NAME => Some(RewardCommand::handle_slash_command(command)),
                SkillCommand::NAME => Some(SkillCommand::handle_slash_command(command)),
                SkillItemsCommand::NAME => Some(SkillItemsCommand::handle_slash_command(command)),
                SkillsCommand::NAME => Some(SkillsCommand::handle_slash_command(command)),
                SmashCommand::NAME => Some(SmashCommand::handle_slash_command(command)),
                UnpackCommand::NAME => Some(UnpackCommand::handle_slash_command(command)),
                VendorCommand::NAME => Some(VendorCommand::handle_slash_command(command)),
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
                AchievementCommand::NAME => Some(AchievementCommand::handle_component_interaction(
                    interaction,
                )),
                ActivityCommand::NAME => {
                    Some(ActivityCommand::handle_component_interaction(interaction))
                }
                BrickCommand::NAME => Some(BrickCommand::handle_component_interaction(interaction)),
                BuyCommand::NAME => Some(BuyCommand::handle_component_interaction(interaction)),
                CooldownGroupCommand::NAME => Some(
                    CooldownGroupCommand::handle_component_interaction(interaction),
                ),
                DropCommand::NAME => Some(DropCommand::handle_component_interaction(interaction)),
                EarnCommand::NAME => Some(EarnCommand::handle_component_interaction(interaction)),
                EnemyCommand::NAME => Some(EnemyCommand::handle_component_interaction(interaction)),
                GetCommand::NAME => Some(GetCommand::handle_component_interaction(interaction)),
                ItemCommand::NAME => Some(ItemCommand::handle_component_interaction(interaction)),
                LevelCommand::NAME => Some(LevelCommand::handle_component_interaction(interaction)),
                LootTableCommand::NAME => {
                    Some(LootTableCommand::handle_component_interaction(interaction))
                }
                MissionCommand::NAME => {
                    Some(MissionCommand::handle_component_interaction(interaction))
                }
                NpcCommand::NAME => Some(NpcCommand::handle_component_interaction(interaction)),
                PackageCommand::NAME => {
                    Some(PackageCommand::handle_component_interaction(interaction))
                }
                PreconditionsCommand::NAME => Some(
                    PreconditionsCommand::handle_component_interaction(interaction),
                ),
                RewardCommand::NAME => {
                    Some(RewardCommand::handle_component_interaction(interaction))
                }
                SkillCommand::NAME => Some(SkillCommand::handle_component_interaction(interaction)),
                SkillItemsCommand::NAME => {
                    Some(SkillItemsCommand::handle_component_interaction(interaction))
                }
                SkillsCommand::NAME => {
                    Some(SkillsCommand::handle_component_interaction(interaction))
                }
                SmashCommand::NAME => Some(SmashCommand::handle_component_interaction(interaction)),
                UnpackCommand::NAME => {
                    Some(UnpackCommand::handle_component_interaction(interaction))
                }
                VendorCommand::NAME => {
                    Some(VendorCommand::handle_component_interaction(interaction))
                }
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
                    AchievementCommand::register(),
                    ActivityCommand::register(),
                    BrickCommand::register(),
                    BuyCommand::register(),
                    CooldownGroupCommand::register(),
                    DropCommand::register(),
                    EarnCommand::register(),
                    EnemyCommand::register(),
                    GetCommand::register(),
                    ItemCommand::register(),
                    LevelCommand::register(),
                    LootTableCommand::register(),
                    MissionCommand::register(),
                    NpcCommand::register(),
                    PackageCommand::register(),
                    PreconditionsCommand::register(),
                    RewardCommand::register(),
                    SkillCommand::register(),
                    SkillItemsCommand::register(),
                    SkillsCommand::register(),
                    SmashCommand::register(),
                    UnpackCommand::register(),
                    VendorCommand::register(),
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
