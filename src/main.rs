pub mod bot_config;
pub mod cdclient;
mod commands;
pub mod custom;
pub mod ids;
pub mod interaction_command;
pub mod locale;
pub mod pager;
pub mod queries;
pub mod repeat;

use std::collections::BTreeMap;
use std::env;
use std::future::Future;
use std::path::Path;
use std::sync::{Arc, RwLock, RwLockReadGuard};
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
use interaction_command::{CustomIdInteractionType, CustomIdOptions, InteractionCommand};
use locale::{LocaleTranslation, LocaleXML};
use once_cell::sync::Lazy;
use queries::ObjectQueries;
use serenity::all::{
    AutocompleteChoice, CommandInteraction, ComponentInteraction, ComponentInteractionDataKind,
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

#[derive(Default)]
pub struct LocaleXML2 {
    pub locales: BTreeMap<String, Arc<RwLock<LocaleTranslation>>>,
}

static RW_LOCALE_XML: Lazy<Arc<RwLock<LocaleXML2>>> =
    Lazy::new(|| Arc::new(RwLock::new(LocaleXML2::default())));

static RW_LOCALE: Lazy<Arc<RwLock<LocaleTranslation>>> = Lazy::new(|| {
    let locale = &CONFIG.locale;
    // path should be in config
    let translation = LocaleXML::load_xml(Path::new("/home/dgmastertemple/locale.xml"))
        .unwrap()
        .locales
        .remove(locale)
        .expect(format!("Could not find locale {:?}", locale).as_str());
    Arc::new(RwLock::new(translation))
});

static RW_CD_CLIENT: Lazy<Arc<RwLock<CdClient>>> = Lazy::new(|| {
    Arc::new(RwLock::new(
        CdClient::load_sqlite(Path::new("/home/dgmastertemple/cdclient.sqlite")).unwrap(),
    ))
});

static RW_CONFIG: Lazy<Arc<RwLock<BotConfig>>> =
    Lazy::new(|| Arc::new(RwLock::new(BotConfig::default())));

// ALSO I CAN IMPL THIS ON MY COMMANDS
pub trait Api {
    fn config(&self) -> RwLockReadGuard<'_, BotConfig> {
        RW_CONFIG.read().unwrap()
    }

    fn locale(&self) -> RwLockReadGuard<'_, LocaleTranslation> {
        RW_LOCALE.read().unwrap()
    }

    fn cdclient(&self) -> RwLockReadGuard<'_, CdClient> {
        RW_CD_CLIENT.read().unwrap()
    }
}

pub fn handle_autocomplete(completion: &CommandInteraction) -> Option<Vec<AutocompleteChoice>> {
    let option = completion.data.autocomplete()?;
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
    content
}

pub fn handle_slash_command(
    command: &CommandInteraction,
) -> Option<CreateInteractionResponseMessage> {
    match command.data.name.as_str() {
        AchievementCommand::NAME => Some(AchievementCommand::handle_slash_command(command)),
        ActivityCommand::NAME => Some(ActivityCommand::handle_slash_command(command)),
        BrickCommand::NAME => Some(BrickCommand::handle_slash_command(command)),
        BuyCommand::NAME => Some(BuyCommand::handle_slash_command(command)),
        CooldownGroupCommand::NAME => Some(CooldownGroupCommand::handle_slash_command(command)),
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
        PreconditionsCommand::NAME => Some(PreconditionsCommand::handle_slash_command(command)),
        RewardCommand::NAME => Some(RewardCommand::handle_slash_command(command)),
        SkillCommand::NAME => Some(SkillCommand::handle_slash_command(command)),
        SkillItemsCommand::NAME => Some(SkillItemsCommand::handle_slash_command(command)),
        SkillsCommand::NAME => Some(SkillsCommand::handle_slash_command(command)),
        SmashCommand::NAME => Some(SmashCommand::handle_slash_command(command)),
        UnpackCommand::NAME => Some(UnpackCommand::handle_slash_command(command)),
        VendorCommand::NAME => Some(VendorCommand::handle_slash_command(command)),
        _ => None,
    }
}

pub fn handle_component_interaction(
    interaction: &ComponentInteraction,
    options: &CustomIdOptions,
) -> Option<CreateInteractionResponseMessage> {
    match options.cmd.as_str() {
        AchievementCommand::NAME => Some(AchievementCommand::handle_component_interaction(
            interaction,
            options,
        )),
        ActivityCommand::NAME => Some(ActivityCommand::handle_component_interaction(
            interaction,
            options,
        )),
        BrickCommand::NAME => Some(BrickCommand::handle_component_interaction(
            interaction,
            options,
        )),
        BuyCommand::NAME => Some(BuyCommand::handle_component_interaction(
            interaction,
            options,
        )),
        CooldownGroupCommand::NAME => Some(CooldownGroupCommand::handle_component_interaction(
            interaction,
            options,
        )),
        DropCommand::NAME => Some(DropCommand::handle_component_interaction(
            interaction,
            options,
        )),
        EarnCommand::NAME => Some(EarnCommand::handle_component_interaction(
            interaction,
            options,
        )),
        EnemyCommand::NAME => Some(EnemyCommand::handle_component_interaction(
            interaction,
            options,
        )),
        GetCommand::NAME => Some(GetCommand::handle_component_interaction(
            interaction,
            options,
        )),
        ItemCommand::NAME => Some(ItemCommand::handle_component_interaction(
            interaction,
            options,
        )),
        LevelCommand::NAME => Some(LevelCommand::handle_component_interaction(
            interaction,
            options,
        )),
        LootTableCommand::NAME => Some(LootTableCommand::handle_component_interaction(
            interaction,
            options,
        )),
        MissionCommand::NAME => Some(MissionCommand::handle_component_interaction(
            interaction,
            options,
        )),
        NpcCommand::NAME => Some(NpcCommand::handle_component_interaction(
            interaction,
            options,
        )),
        PackageCommand::NAME => Some(PackageCommand::handle_component_interaction(
            interaction,
            options,
        )),
        PreconditionsCommand::NAME => Some(PreconditionsCommand::handle_component_interaction(
            interaction,
            options,
        )),
        RewardCommand::NAME => Some(RewardCommand::handle_component_interaction(
            interaction,
            options,
        )),
        SkillCommand::NAME => Some(SkillCommand::handle_component_interaction(
            interaction,
            options,
        )),
        SkillItemsCommand::NAME => Some(SkillItemsCommand::handle_component_interaction(
            interaction,
            options,
        )),
        SkillsCommand::NAME => Some(SkillsCommand::handle_component_interaction(
            interaction,
            options,
        )),
        SmashCommand::NAME => Some(SmashCommand::handle_component_interaction(
            interaction,
            options,
        )),
        UnpackCommand::NAME => Some(UnpackCommand::handle_component_interaction(
            interaction,
            options,
        )),
        VendorCommand::NAME => Some(VendorCommand::handle_component_interaction(
            interaction,
            options,
        )),
        _ => None,
    }
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Autocomplete(ref completion) = interaction {
            let start = Instant::now();
            let content = handle_autocomplete(completion);
            let time = start.elapsed().as_millis();
            println!("Autocompletion query took {time}ms");

            let data =
                CreateAutocompleteResponse::new().set_choices(content.unwrap_or_else(|| vec![]));
            let builder = CreateInteractionResponse::Autocomplete(data);
            if let Err(why) = completion.create_response(&ctx.http, builder).await {
                println!("Cannot respond to auto-completion request: {why}");
            }
        }

        if let Interaction::Command(ref command) = interaction {
            // println!("Received command interaction: {command:#?}");
            let start = Instant::now();
            let content = handle_slash_command(command);
            let time = start.elapsed().as_millis();
            println!("Slash Command query took {time}ms");

            if let Some(content) = content {
                let builder = CreateInteractionResponse::Message(content);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    println!("Cannot respond to slash command: {why}");
                }
            }
        }

        if let Interaction::Component(ref interaction) = interaction {
            let Ok(ref options) = (match &interaction.data.kind {
                ComponentInteractionDataKind::Button => {
                    CustomIdOptions::from_custom_id(interaction.data.custom_id.as_str())
                }
                ComponentInteractionDataKind::StringSelect { values } => values
                    .first()
                    .ok_or_else(|| format!("No selection given"))
                    .and_then(|value| CustomIdOptions::from_custom_id(value)),
                _ => Err(format!("Unsupported command interaction type")),
            }) else {
                return;
            };

            dbg!(&options);

            let start = Instant::now();
            if let Some(content) = handle_component_interaction(interaction, options) {
                let time = start.elapsed().as_millis();
                println!("Component query took {time}ms");
                let builder = match options.interaction {
                    CustomIdInteractionType::Reply => CreateInteractionResponse::Message(content),
                    CustomIdInteractionType::Update => {
                        CreateInteractionResponse::UpdateMessage(content)
                    }
                };
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

    // {
    //     // force it to load
    //     let start = Instant::now();
    //     let levels = &RW_CD_CLIENT.level_progression_lookup.len();
    //     println!("Total levels: {levels}");
    //     let end = start.elapsed().as_millis();
    //     println!("'cdclient.sqlite' loaded in {end}ms");
    //     // force it to load
    //     let start = Instant::now();
    //     let locale_count = &RW_LOCALE_XML.locales.len();
    //     println!("Total locales: {locale_count}");
    //     let end = start.elapsed().as_millis();
    //     println!("'locale.xml' loaded in {end}ms");
    // }

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
