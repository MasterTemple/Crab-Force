use crate::cdclient::components::{ITEM_COMPONENT, RENDER_COMPONENT};
use crate::interaction_command::{CustomIdOptions, InteractionCommand, ToCustomId};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{
    AutocompleteChoice, CommandOptionType, CreateActionRow, CreateCommandOption, CreateEmbed,
    ResolvedOption,
};

pub fn fix_icon_asset(asset: &str) -> String {
    asset
        .replace("\\", "/")
        .replace("../", "")
        .replace("./", "")
}

pub fn icon_asset_as_url(asset: &str) -> String {
    CONFIG.explorer_res_uri(&fix_icon_asset(asset))
}

pub struct PreconditionsCommand;

pub struct PreconditionsArguments {
    item: i32,
}

impl ToCustomId for PreconditionsArguments {
    const CMD: &'static str = PreconditionsCommand::NAME;

    fn parameters(&self) -> String {
        let PreconditionsArguments { item } = self;
        format!("item={item}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for PreconditionsArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(PreconditionsArguments {
            item: options.parse("item")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for PreconditionsArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(PreconditionsArguments {
            item: int_option!(options, "item"),
        })
    }
}

impl InteractionCommand for PreconditionsCommand {
    const NAME: &'static str = "preconditions";

    const DESCRIPTION: &'static str = "View the preconditions to use an item!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "item",
            "An item in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = PreconditionsArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        if input.len() == 0 {
            return Some(vec![]);
        }
        Some(
            CD_CLIENT
                .objects
                .iter()
                // .filter(|item| item.name.len() > 0)
                .map(|item| {
                    let id = item.id;
                    let name = item.display_name.clone().unwrap_or_else(|| {
                        item.name.clone().unwrap_or_else(|| format!("Item {id}"))
                    });
                    // dbg!(&item);
                    (id, name)
                    // let name = item.name.clone();
                    // .clone()
                    // .unwrap_or_else(|| item.name.clone());
                    // AutocompleteChoice::new(format!("[{id}] {name}"), id)
                })
                .filter(|(_, name)| name.to_lowercase().contains(input))
                .map(|(id, name)| AutocompleteChoice::new(format!("[{id}] {name}"), id))
                .take(25)
                .collect(),
        )
    }

    fn run(arguments: Self::Arguments) -> (CreateEmbed, Option<Vec<CreateActionRow>>) {
        let PreconditionsArguments { item: item_id } = arguments;

        let Some(item) = CD_CLIENT.objects.at_key(&item_id) else {
            return CONFIG.error_msg(format!("Object `{item_id}` does not exist!"));
        };

        let name = LOCALE_XML
            .locales
            .get("en_US")
            .unwrap()
            .objects
            .get(&item.id)
            .map(|o| o.name.clone())
            .flatten()
            .unwrap_or_else(|| {
                item.display_name.clone().unwrap_or_else(|| {
                    item.name
                        .clone()
                        .unwrap_or_else(|| format!("Item {item_id}"))
                })
            });

        let Some(components) = CD_CLIENT.components_registry.at_group_key(&item_id) else {
            return CONFIG.error_msg(format!("Object `{item_id}` has no Registered Components"));
        };

        let Some(item_component) = components
            .iter()
            .find(|comp| comp.component_type == ITEM_COMPONENT)
        else {
            return CONFIG.error_msg(format!(
                "Object `{item_id}` has no Registered Item Component"
            ));
        };

        let Some(item_component) = CD_CLIENT
            .item_component
            .at_key(&item_component.component_id)
        else {
            return CONFIG.error_msg(format!(
                "Item Component `{}` does not exist!",
                item_component.component_id
            ));
        };

        let value = format!("{:?}", item_component.req_precondition);

        let Some(render_component) = components
            .iter()
            .find(|comp| comp.component_type == RENDER_COMPONENT)
        else {
            return CONFIG.error_msg(format!(
                "Object `{item_id}` has no Registered Render Component"
            ));
        };

        let Some(render_component) = CD_CLIENT
            .render_component
            .at_key(&render_component.component_id)
        else {
            return CONFIG.error_msg(format!(
                "Render Component `{}` does not exist!",
                render_component.component_id
            ));
        };

        let icon_url = render_component
            .icon_asset
            .as_ref()
            .map(|asset| icon_asset_as_url(asset));

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} {}", name, item.id))
            .url(CONFIG.explorer_uri(format!("/objects/{}", item.id).as_str()))
            .field("Preconditions", value, false);

        if let Some(icon_url) = icon_url {
            embed = embed.thumbnail(icon_url);
        }

        // .thumbnail(CONFIG.explorer_uri("/lu-res/ui/ingame/passport_i90.png"));

        // let prev_level_button = PreconditionsArguments { item: level - 1 }
        //     .to_button(format!("Preconditions {}", level - 1))
        //     .disabled(level - 1 < min_level);
        // let next_level_button = PreconditionsArguments { level: level + 1 }
        //     .to_button(format!("Preconditions {}", level + 1))
        //     .disabled(level + 1 > max_level);
        //
        // let components = Some(vec![CreateActionRow::Buttons(vec![
        //     prev_level_button,
        //     next_level_button,
        // ])]);

        (embed, None)
    }
}
