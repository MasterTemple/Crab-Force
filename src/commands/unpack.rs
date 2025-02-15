use crate::cdclient::components::PACKAGE_COMPONENT;
use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AutocompleteQueries, LootQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct UnpackCommand;

pub struct UnpackArguments {
    item: i32,
}

impl ToCustomId for UnpackArguments {
    const CMD: &'static str = UnpackCommand::NAME;

    fn parameters(&self) -> String {
        let UnpackArguments { item } = self;
        format!("item={item}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for UnpackArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(UnpackArguments {
            item: options.parse("item")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for UnpackArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(UnpackArguments {
            item: int_option!(options, "item"),
        })
    }
}

impl InteractionCommand for UnpackCommand {
    const NAME: &'static str = "unpack";

    const DESCRIPTION: &'static str = "View all packages that drop an item!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "item",
            "An item in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = UnpackArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_object(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let UnpackArguments { item: id } = arguments;

        let explorer_url = CD_CLIENT.object_explorer_url(id);
        let name = CD_CLIENT.req_object_name(id);

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{} [{}]", name, id))
            .url(explorer_url);

        if let Some(icon_url) = CD_CLIENT.object_icon_url(id) {
            embed = embed.thumbnail(icon_url);
        }

        let item_component = CD_CLIENT.object_item_component(id)?;
        let rarity = item_component
            .rarity
            .ok_or_else(|| format!("{} has no Rarity", CD_CLIENT.object_explorer_url(id)))?;

        let packages = CD_CLIENT.object_package_ids(id).unwrap_or_else(|_| vec![]);

        let description = if packages.len() == 0 {
            format!("This item is not unpacked")
        } else {
            let packages = packages
                .into_iter()
                .enumerate()
                .filter_map(|(idx, package_id)| {
                    let pkg = CD_CLIENT.object_package_component(package_id).ok()?;
                    let chance = calculate_chance_to_drop(pkg.loot_matrix_index, id)?;
                    let name = CD_CLIENT.object_hyperlinked_name(package_id);
                    Some(format!("**{}.** {:.4}% {name}", idx + 1, chance * 100.0))
                })
                .collect::<Vec<_>>()
                .join("\n");
            format!("**Packages**\n{packages}")
        };
        embed = embed.description(description);

        Ok((embed, None))
    }
}

fn calculate_chance_to_drop(lmi: i32, id: i32) -> Option<f64> {
    // "chance to drop loot table" * "chance to drop rarity" * (1 / "number of items of same rarity in loot table")
    let lm_table = CD_CLIENT.loot_matrix.at_group_key(&lmi)?;
    let rarity = CD_CLIENT.object_item_component(id).ok()?.rarity?;
    // for lm in lm_table.iter() {
    //     let lti = lm.loot_table_index;
    //     let rti = lm.rarity_table_index;
    //     let chance_to_drop_rarity = calc_chance_to_drop_rarity(rarity, rti)
    // }

    // let lm_table = CD_CLIENT.loot_matrix.at_group_key(&lmi)?;
    let ltis = CD_CLIENT.loot_table_indexes_with_item(id)?;
    let lm = lm_table
        .iter()
        .find(|lm| ltis.contains(&lm.loot_table_index))?;

    let chance_to_drop_loot_table = lm.percent;

    let number_of_items_of_rarity_in_loot_table =
        calc_number_of_items_of_rarity_in_loot_table(rarity, lm.loot_table_index)?;

    let chance_to_drop_rarity = calc_chance_to_drop_rarity(rarity, lm.rarity_table_index)?;
    // dbg!(
    //     &chance_to_drop_rarity,
    //     &chance_to_drop_loot_table,
    //     &number_of_items_of_rarity_in_loot_table
    // );

    let chance = chance_to_drop_rarity
        * chance_to_drop_loot_table
        * (1.0 / number_of_items_of_rarity_in_loot_table as f64);

    // I should still multiple by number drops
    let avg_dropped = (lm.min_to_drop as f64 + lm.max_to_drop as f64) / 2.0;

    Some(avg_dropped * chance)
}

fn calc_chance_to_drop_rarity(rarity: i32, rti: i32) -> Option<f64> {
    // let rarity = CD_CLIENT.object_item_component(id).ok()?.rarity?;
    let rarity_table = CD_CLIENT.rarity_table.at_group_key(&rti)?;
    let upper_threshold = rarity_table.iter().find(|r| r.rarity == rarity)?.randmax;
    let lower_threshold = rarity_table
        .iter()
        .find_map(|r| (r.rarity == rarity - 1).then(|| r.randmax))
        .unwrap_or(0.0);
    let chance_to_drop_rarity = upper_threshold - lower_threshold;
    Some(chance_to_drop_rarity)
}

fn calc_number_of_items_of_rarity_in_loot_table(rarity: i32, lti: i32) -> Option<usize> {
    let loot_table = CD_CLIENT.loot_table.at_group_key(&lti)?;
    // let object_ids =
    let count = loot_table
        .iter()
        .filter_map(|lt| {
            CD_CLIENT
                .object_item_component(lt.itemid)
                .ok()
                .map(|ic| ic.rarity)
                .flatten()
        })
        .filter(|comp_rarity| *comp_rarity == rarity)
        .count();
    Some(count)
}
