pub mod cdclient;
mod commands;
pub mod config;
pub mod data;
pub mod embed;
pub mod error;
pub mod ids;
pub mod interaction_command;
pub mod locale;
pub mod loot;
pub mod pager;
pub mod search;

use std::process::ExitCode;
use std::time::Instant;

use serenity::all::{
    ButtonStyle, Colour, CommandInteraction, ComponentInteraction, ComponentInteractionDataKind,
    CreateActionRow, CreateAutocompleteResponse, CreateButton, CreateInteractionResponse,
    CreateInteractionResponseMessage, EditInteractionResponse, GuildId, Interaction, Ready,
};
use serenity::async_trait;
use serenity::model::application::Command;
use serenity::prelude::*;
use tracing::{debug, error, info, warn};

use crate::config::{config, BotConfig};
use crate::data::GameData;
use crate::embed::Embed;
use crate::error::{BotError, BotResult};
use crate::interaction_command::{CustomId, Mode, OptionKind, Response, Target, MAX_ROWS};
use crate::search::SearchCategory;

const ERROR_COLOR: Colour = Colour::new(0xed4245);

/// Runs `f` with the current game data on a blocking thread
/// - keeps CPU heavy lookups off the async runtime
/// - turns a panic into a [`BotError::Internal`] instead of silently dropping the interaction
async fn with_data<T, F>(f: F) -> BotResult<T>
where
    T: Send + 'static,
    F: FnOnce(&GameData) -> BotResult<T> + Send + 'static,
{
    let data = data::get()?;
    match tokio::task::spawn_blocking(move || f(&data)).await {
        Ok(result) => result,
        Err(join_error) if join_error.is_panic() => {
            let payload = join_error.into_panic();
            let msg = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| String::from("unknown panic"));
            Err(BotError::internal(format!("panicked: {msg}")))
        }
        Err(join_error) => Err(BotError::internal(format!("task failed: {join_error}"))),
    }
}

fn error_embed(err: &BotError) -> Embed {
    Embed::new()
        .title(err.title())
        .description(err.user_message())
        .color(ERROR_COLOR)
}

fn error_message(err: &BotError) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new()
        .embed(error_embed(err).build())
        .ephemeral(true)
}

/// Logs an error at the right level for its kind
fn log_error(context: &str, err: &BotError) {
    if err.is_internal() {
        error!("{context}: {err}");
    } else {
        debug!("{context}: {err}");
    }
}

/// Sends `response`; if Discord rejects it (e.g. a limit we did not anticipate), replies with
/// an error instead so the user is never left with "The application did not respond"
async fn respond(
    ctx: &Context,
    context: &str,
    send: impl AsyncFnOnce(CreateInteractionResponse) -> serenity::Result<()>,
    send_error: impl AsyncFnOnce(CreateInteractionResponse) -> serenity::Result<()>,
    response: CreateInteractionResponse,
) {
    let _ = ctx;
    if let Err(why) = send(response).await {
        error!("{context}: Discord rejected the response: {why}");
        let err = BotError::internal(format!("Discord rejected the response: {why}"));
        if let Err(why) = send_error(CreateInteractionResponse::Message(error_message(&err))).await {
            error!("{context}: could not send the error message either: {why}");
        }
    }
}

// ------------ //
// Autocomplete //
// ------------ //

async fn handle_autocomplete(ctx: &Context, interaction: &CommandInteraction) {
    let Some(focused) = interaction.data.autocomplete() else {
        return;
    };
    let category = match commands::find(&interaction.data.name).map(|spec| spec.option) {
        Some(OptionKind::Search(category)) => category,
        _ => return,
    };
    let query = focused.value.to_string();
    let start = Instant::now();
    let choices = with_data(move |d| Ok(d.search.autocomplete(category, &query)))
        .await
        .unwrap_or_else(|err| {
            log_error("autocomplete", &err);
            vec![]
        });
    debug!("autocomplete /{} `{}` took {:?}", interaction.data.name, focused.value, start.elapsed());

    let response = CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new().set_choices(choices));
    if let Err(why) = interaction.create_response(&ctx.http, response).await {
        // autocomplete requests are superseded constantly while typing; this is normal
        debug!("could not respond to autocomplete: {why}");
    }
}

// -------------- //
// Slash Commands //
// -------------- //

async fn handle_command(ctx: &Context, interaction: &CommandInteraction) {
    let name = interaction.data.name.clone();
    let context = format!("/{name} by {}", interaction.user.name);
    let start = Instant::now();

    if name == commands::reload::NAME {
        handle_reload(ctx, interaction).await;
        return;
    }

    let options: Vec<(String, String)> = interaction
        .data
        .options()
        .iter()
        .map(|o| (o.name.to_string(), format!("{:?}", o.value)))
        .collect();
    debug!("{context} {options:?}");

    let result = run_slash_command(interaction).await;
    let response = match result {
        Ok(response) => CreateInteractionResponse::Message(response.into_message()),
        Err(err) => {
            log_error(&format!("{context} {options:?}"), &err);
            CreateInteractionResponse::Message(error_message(&err))
        }
    };
    respond(
        ctx,
        &context,
        async |r| interaction.create_response(&ctx.http, r).await,
        async |r| interaction.create_response(&ctx.http, r).await,
        response,
    )
    .await;
    debug!("{context} took {:?}", start.elapsed());
}

async fn run_slash_command(interaction: &CommandInteraction) -> BotResult<Response> {
    let name = interaction.data.name.clone();
    if name == commands::search::NAME {
        let (query, category) = commands::search::parse_options(&interaction.data.options())?;
        return with_data(move |d| commands::search::run(d, &query, category)).await;
    }

    let spec = commands::find(&name)
        .ok_or_else(|| BotError::internal(format!("received unknown command `/{name}`")))?;
    // resolved options borrow the interaction, so turn them into a target before moving
    // into the blocking task
    let data = data::get()?;
    let target = (spec.target_from_options)(&data, &interaction.data.options())?;
    drop(data);
    with_data(move |d| (spec.run)(d, target)).await
}

async fn handle_reload(ctx: &Context, interaction: &CommandInteraction) {
    let config = config();
    if let Err(err) = commands::reload::check_permission(config, interaction) {
        let response = CreateInteractionResponse::Message(error_message(&err));
        if let Err(why) = interaction.create_response(&ctx.http, response).await {
            error!("/reload: could not respond: {why}");
        }
        return;
    }

    // loading takes longer than Discord's 3 second response window
    let defer = CreateInteractionResponse::Defer(CreateInteractionResponseMessage::new().ephemeral(true));
    if let Err(why) = interaction.create_response(&ctx.http, defer).await {
        error!("/reload: could not defer: {why}");
        return;
    }

    info!("/reload requested by {}", interaction.user.name);
    let start = Instant::now();
    let result = tokio::task::spawn_blocking(move || data::reload(config)).await;
    let embed = match result {
        Ok(Ok(warnings)) => {
            let mut description = format!("Reloaded `cdclient.sqlite` and `locale.xml` in {:.1?}.", start.elapsed());
            if !warnings.is_empty() {
                let tables: Vec<&str> = warnings.iter().map(|w| w.table).collect();
                description.push_str(&format!(
                    "\n\n{} optional table(s) could not be loaded and are empty: `{}`",
                    tables.len(),
                    tables.join("`, `")
                ));
            }
            info!("reload finished in {:?}", start.elapsed());
            Embed::new().title("Reloaded").description(description)
        }
        Ok(Err(why)) => {
            error!("reload failed: {why}");
            Embed::new()
                .title("Reload Failed")
                .description(format!("{why}\n\nThe previously loaded data is still being used."))
                .color(ERROR_COLOR)
        }
        Err(why) => {
            error!("reload task failed: {why}");
            Embed::new()
                .title("Reload Failed")
                .description("An unexpected error occurred. The previously loaded data is still being used.")
                .color(ERROR_COLOR)
        }
    };
    if let Err(why) = interaction
        .edit_response(&ctx.http, EditInteractionResponse::new().embed(embed.build()))
        .await
    {
        error!("/reload: could not send result: {why}");
    }
}

// ---------------------- //
// Buttons & Select Menus //
// ---------------------- //

fn component_custom_id(interaction: &ComponentInteraction) -> BotResult<CustomId> {
    match &interaction.data.kind {
        ComponentInteractionDataKind::Button => CustomId::parse(&interaction.data.custom_id),
        ComponentInteractionDataKind::StringSelect { values } => values
            .first()
            .ok_or_else(|| BotError::invalid("Nothing was selected."))
            .and_then(|value| CustomId::parse(value)),
        other => Err(BotError::internal(format!("unsupported component type: {other:?}"))),
    }
}

async fn handle_component(ctx: &Context, interaction: &ComponentInteraction) {
    let context = format!("component `{}` by {}", interaction.data.custom_id, interaction.user.name);
    let start = Instant::now();

    let result = async {
        let custom_id = component_custom_id(interaction)?;
        let spec = commands::find(&custom_id.command)
            .ok_or_else(|| BotError::invalid("This button no longer works. Please run the command again."))?;
        let target: Target = custom_id.target;
        let response = with_data(move |d| (spec.run)(d, target)).await?;
        Ok::<_, BotError>((custom_id.mode, response))
    }
    .await;

    let response = match result {
        Ok((Mode::Update, response)) => CreateInteractionResponse::UpdateMessage(response.into_message()),
        Ok((Mode::Reply, mut response)) => {
            // let people get back to where they clicked from
            if response.components.len() < MAX_ROWS {
                response.components.push(CreateActionRow::Buttons(vec![
                    CreateButton::new_link(interaction.message.link()).label("Back").style(ButtonStyle::Secondary),
                ]));
            }
            CreateInteractionResponse::Message(response.into_message())
        }
        Err(err) => {
            log_error(&context, &err);
            // errors go in a private message so the original message is left untouched
            CreateInteractionResponse::Message(error_message(&err))
        }
    };
    respond(
        ctx,
        &context,
        async |r| interaction.create_response(&ctx.http, r).await,
        async |r| interaction.create_response(&ctx.http, r).await,
        response,
    )
    .await;
    debug!("{context} took {:?}", start.elapsed());
}

// ------- //
// Handler //
// ------- //

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Autocomplete(ref completion) => handle_autocomplete(&ctx, completion).await,
            Interaction::Command(ref command) => handle_command(&ctx, command).await,
            Interaction::Component(ref component) => handle_component(&ctx, component).await,
            _ => {}
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected to {} server(s)", ready.user.name, ready.guilds.len());

        let registrations = commands::registrations();
        let count = registrations.len();
        let result = match config().guild_id {
            Some(guild) => GuildId::new(guild).set_commands(&ctx.http, registrations).await,
            None => Command::set_global_commands(&ctx.http, registrations).await,
        };
        match result {
            Ok(_) => info!(
                "registered {count} commands {}",
                match config().guild_id {
                    Some(guild) => format!("to server {guild}"),
                    None => String::from("globally (they may take up to an hour to appear)"),
                }
            ),
            Err(why) => error!("could not register commands: {why}"),
        }
    }
}

// ------- //
// Startup //
// ------- //

fn init_logging() {
    use tracing_subscriber::EnvFilter;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,serenity=warn"));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

const USAGE: &str = "\
Usage:
  crab_force                         run the bot
  crab_force check                   load the data files and report problems
  crab_force preview <command> <arg> print a command's output as JSON (no Discord needed)
  crab_force search <query>          print /search results as JSON";

/// Renders a response the way it would be sent, for testing without Discord
fn print_response(response: BotResult<Response>) -> ExitCode {
    match response {
        Ok(response) => {
            let message = response.into_message();
            match serenity::json::to_string_pretty(&message) {
                Ok(json) => println!("{json}"),
                Err(why) => eprintln!("could not serialize: {why}"),
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}: {}", err.title(), err);
            ExitCode::FAILURE
        }
    }
}

/// Checks the slash command definitions against Discord's rules (a single bad one makes Discord
/// reject all of them)
fn validate_registrations() -> Vec<String> {
    let valid_name = |name: &str| {
        (1..=32).contains(&name.len())
            && name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_')
    };
    let mut problems = vec![];
    let mut names = std::collections::HashSet::new();
    for command in commands::registrations() {
        let json = match serenity::json::to_value(&command) {
            Ok(json) => json,
            Err(why) => {
                problems.push(format!("could not serialize: {why}"));
                continue;
            }
        };
        let name = json["name"].as_str().unwrap_or_default().to_string();
        if !valid_name(&name) || !names.insert(name.clone()) {
            problems.push(format!("invalid or duplicate name `{name}`"));
        }
        let description = json["description"].as_str().unwrap_or_default();
        if !(1..=100).contains(&description.chars().count()) {
            problems.push(format!("/{name}: description must be 1-100 characters"));
        }
        let options = json["options"].as_array().cloned().unwrap_or_default();
        if options.len() > 25 {
            problems.push(format!("/{name}: more than 25 options"));
        }
        let mut seen_optional = false;
        for option in &options {
            let option_name = option["name"].as_str().unwrap_or_default();
            if !valid_name(option_name) {
                problems.push(format!("/{name}: invalid option name `{option_name}`"));
            }
            if !(1..=100).contains(&option["description"].as_str().unwrap_or_default().chars().count()) {
                problems.push(format!("/{name} {option_name}: description must be 1-100 characters"));
            }
            let required = option["required"].as_bool().unwrap_or(false);
            if required && seen_optional {
                problems.push(format!("/{name}: required option `{option_name}` after an optional one"));
            }
            seen_optional |= !required;
            let choices = option["choices"].as_array().map_or(0, Vec::len);
            if choices > 25 || (choices > 0 && option["autocomplete"].as_bool() == Some(true)) {
                problems.push(format!("/{name} {option_name}: invalid choices"));
            }
        }
    }
    problems
}

/// Checks a serialized message against Discord's limits, returning every violation
fn validate_message(message: &serenity::json::Value) -> Vec<String> {
    use serenity::json::Value;
    let mut problems = vec![];
    let len = |v: &Value| v.as_str().map_or(0, |s| s.chars().count());
    for embed in message["embeds"].as_array().into_iter().flatten() {
        let mut total = len(&embed["title"]) + len(&embed["description"]) + len(&embed["footer"]["text"]) + len(&embed["author"]["name"]);
        if len(&embed["title"]) > 256 {
            problems.push(String::from("title > 256"));
        }
        if len(&embed["description"]) > 4096 {
            problems.push(String::from("description > 4096"));
        }
        let fields = embed["fields"].as_array().cloned().unwrap_or_default();
        if fields.len() > 25 {
            problems.push(format!("{} fields", fields.len()));
        }
        for field in &fields {
            let (name, value) = (len(&field["name"]), len(&field["value"]));
            total += name + value;
            if name == 0 || name > 256 {
                problems.push(format!("field name length {name}"));
            }
            if value == 0 || value > 1024 {
                problems.push(format!("field value length {value}"));
            }
        }
        if total > 6000 {
            problems.push(format!("embed total {total}"));
        }
    }
    let rows = message["components"].as_array().cloned().unwrap_or_default();
    if rows.len() > MAX_ROWS - 1 {
        // one row is reserved for the "Back" button on replies
        problems.push(format!("{} rows (Back button would not fit)", rows.len()));
    }
    let mut ids = std::collections::HashSet::new();
    for row in &rows {
        let components = row["components"].as_array().cloned().unwrap_or_default();
        if components.is_empty() || components.len() > 5 {
            problems.push(format!("row with {} components", components.len()));
        }
        for c in &components {
            if let Some(id) = c["custom_id"].as_str() {
                if id.len() > 100 {
                    problems.push(format!("custom id too long: {id}"));
                }
                if !ids.insert(id.to_string()) {
                    problems.push(format!("duplicate custom id: {id}"));
                }
                if !id.starts_with("self/") && !id.starts_with("menu/") && CustomId::parse(id).is_err() {
                    problems.push(format!("unparsable custom id: {id}"));
                }
            }
            if len(&c["label"]) > 80 {
                problems.push(String::from("button label > 80"));
            }
            let options = c["options"].as_array().cloned().unwrap_or_default();
            if c["type"] == 3 {
                if options.is_empty() || options.len() > 25 {
                    problems.push(format!("select with {} options", options.len()));
                }
                let mut values = std::collections::HashSet::new();
                for o in &options {
                    let value = o["value"].as_str().unwrap_or_default();
                    if !values.insert(value.to_string()) {
                        problems.push(format!("duplicate option value {value}"));
                    }
                    if CustomId::parse(value).is_err() || value.len() > 100 {
                        problems.push(format!("bad option value {value}"));
                    }
                    if len(&o["label"]) == 0 || len(&o["label"]) > 100 || len(&o["description"]) > 100 {
                        problems.push(String::from("option label/description length"));
                    }
                }
            }
        }
    }
    problems
}

/// Runs every command on every searchable thing and validates the output
fn sweep(data: &GameData) -> ExitCode {
    let start = Instant::now();
    let (mut ok, mut expected_errors, mut failures) = (0usize, 0usize, 0usize);
    let mut check = |label: String, result: BotResult<Response>| match result {
        Ok(response) => {
            let json = match serenity::json::to_value(response.into_message()) {
                Ok(json) => json,
                Err(why) => {
                    failures += 1;
                    error!("{label}: could not serialize: {why}");
                    return;
                }
            };
            let problems = validate_message(&json);
            if problems.is_empty() {
                ok += 1;
            } else {
                failures += 1;
                error!("{label}: {}", problems.join("; "));
            }
        }
        Err(err) if err.is_internal() => {
            failures += 1;
            error!("{label}: {err}");
        }
        Err(err) => {
            debug!("{label}: {err}");
            expected_errors += 1;
        }
    };

    for spec in commands::COMMANDS {
        let ids: Vec<i32> = match spec.option {
            OptionKind::Search(category) => data
                .search
                .search(category, "", usize::MAX)
                .into_iter()
                .map(|hit| hit.entry.id)
                .collect(),
            OptionKind::Integer { .. } => data.cdclient.level_progression_lookup.iter().map(|l| l.id).collect(),
        };
        for id in ids {
            // first page, a page far out of range, and every tier for loot tables
            for (page, tier) in [(1, None), (9999, None), (1, Some(0)), (1, Some(3))] {
                if tier.is_some() && spec.name != "loottable" {
                    continue;
                }
                let target = Target::new(id).page(page).tier(tier);
                let result = std::panic::catch_unwind(|| (spec.run)(data, target))
                    .unwrap_or_else(|_| Err(BotError::internal("panicked")));
                check(format!("/{} {id} page {page} tier {tier:?}", spec.name), result);
            }
        }
        // ids that do not exist
        for id in [-1, 0, i32::MAX] {
            let result = std::panic::catch_unwind(|| (spec.run)(data, Target::new(id)))
                .unwrap_or_else(|_| Err(BotError::internal("panicked")));
            check(format!("/{} {id}", spec.name), result);
        }
    }
    for query in ["a", "bat lord", "sword", "123", "zzzzzzzzz", "maelstrom brick", "' \" ` * _"] {
        for category in std::iter::once(None).chain(SearchCategory::ALL.map(Some)) {
            check(format!("/search {query} {category:?}"), commands::search::run(data, query, category));
        }
    }

    info!("sweep finished in {:?}: {ok} ok, {expected_errors} expected errors (not found etc.), {failures} failures", start.elapsed());
    if failures == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn offline(args: &[String], config: &BotConfig) -> ExitCode {
    let data = match GameData::load(config) {
        Ok((data, _)) => data,
        Err(why) => {
            error!("{why}");
            return ExitCode::FAILURE;
        }
    };
    match args.first().map(String::as_str) {
        Some("check") => {
            for category in SearchCategory::ALL {
                info!("{:>15}: {} searchable", category.label(), data.search.len(category));
            }
            info!("{} commands + /search + /reload", commands::COMMANDS.len());
            let problems = validate_registrations();
            for problem in &problems {
                error!("command registration: {problem}");
            }
            if problems.is_empty() {
                ExitCode::SUCCESS
            } else {
                ExitCode::FAILURE
            }
        }
        Some("search") => print_response(commands::search::run(&data, &args[1..].join(" "), None)),
        Some("sweep") => sweep(&data),
        Some("preview") => {
            let Some(spec) = args.get(1).and_then(|name| commands::find(name)) else {
                eprintln!("unknown command\n\n{USAGE}");
                return ExitCode::FAILURE;
            };
            let input = args[2..].join(" ");
            let target = match spec.option {
                OptionKind::Search(category) => data.search.resolve(category, &input),
                OptionKind::Integer { .. } => input.trim().parse().map_err(|_| BotError::invalid("expected a number")),
            };
            let page = std::env::var("PAGE").ok().and_then(|p| p.parse().ok()).unwrap_or(1);
            let tier = std::env::var("TIER").ok().and_then(|t| t.parse().ok());
            print_response(target.and_then(|id| (spec.run)(&data, Target::new(id).page(page).tier(tier))))
        }
        _ => {
            eprintln!("{USAGE}");
            ExitCode::FAILURE
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    // a missing .env is fine; the variables may be set another way
    let dotenv = dotenvy::dotenv();
    init_logging();
    if let Err(why) = &dotenv {
        if !why.not_found() {
            error!("could not read .env: {why}");
            return ExitCode::FAILURE;
        }
    }

    let config = match BotConfig::from_env() {
        Ok(config) => config,
        Err(why) => {
            error!("{why}");
            return ExitCode::FAILURE;
        }
    };
    config::init(config);
    let config = config::config();

    let args: Vec<String> = std::env::args().skip(1).collect();
    if !args.is_empty() {
        return tokio::task::spawn_blocking(move || offline(&args, config))
            .await
            .unwrap_or(ExitCode::FAILURE);
    }

    if config.token.is_empty() {
        error!("missing required environment variable `DISCORD_TOKEN` (see .env.template)");
        return ExitCode::FAILURE;
    }

    // load before connecting so the bot never answers without data
    match tokio::task::spawn_blocking(|| GameData::load(config::config())).await {
        Ok(Ok((data, _warnings))) => data::install(data),
        Ok(Err(why)) => {
            error!("{why}");
            return ExitCode::FAILURE;
        }
        Err(why) => {
            error!("loading data crashed: {why}");
            return ExitCode::FAILURE;
        }
    }

    let mut client = match Client::builder(&config.token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
    {
        Ok(client) => client,
        Err(why) => {
            error!("could not create the Discord client: {why}");
            return ExitCode::FAILURE;
        }
    };

    let shard_manager = client.shard_manager.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            info!("shutting down");
            shard_manager.shutdown_all().await;
        }
    });

    // shards reconnect with exponential backoff on their own; this only returns on a fatal error
    if let Err(why) = client.start().await {
        match why {
            serenity::Error::Gateway(serenity::gateway::GatewayError::InvalidAuthentication) => {
                error!("Discord rejected the token; check DISCORD_TOKEN")
            }
            other => error!("Discord client stopped: {other}"),
        }
        return ExitCode::FAILURE;
    }
    warn!("Discord client stopped");
    ExitCode::SUCCESS
}
