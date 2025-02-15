use crate::cdclient::MissionPreReqType;
use crate::custom::OptionBuilder;
use crate::interaction_command::{CommandResult, CustomIdOptions, InteractionCommand, ToCustomId};
use crate::queries::{AchievementQueries, AutocompleteQueries, LocaleQueries, ObjectQueries};
use crate::{int_option, CD_CLIENT, CONFIG, LOCALE_XML};
use heck::ToTitleCase;
use serenity::all::{AutocompleteChoice, CommandOptionType, CreateCommandOption, ResolvedOption};

pub struct AchievementCommand;

pub struct AchievementArguments {
    achievement: i32,
}

impl ToCustomId for AchievementArguments {
    const CMD: &'static str = AchievementCommand::NAME;

    fn parameters(&self) -> String {
        let AchievementArguments { achievement } = self;
        format!("achievement={achievement}")
    }
}

impl<'a> TryFrom<CustomIdOptions<'a>> for AchievementArguments {
    type Error = String;

    fn try_from(options: CustomIdOptions<'a>) -> Result<Self, Self::Error> {
        Ok(AchievementArguments {
            achievement: options.parse("achievement")?,
        })
    }
}

impl<'a> TryFrom<&'a [ResolvedOption<'a>]> for AchievementArguments {
    type Error = String;

    fn try_from(options: &'a [ResolvedOption<'a>]) -> Result<Self, Self::Error> {
        Ok(AchievementArguments {
            achievement: int_option!(options, "achievement"),
        })
    }
}

impl InteractionCommand for AchievementCommand {
    const NAME: &'static str = "achievement";

    const DESCRIPTION: &'static str = "View the stats of an achievement!";

    fn options() -> Option<Vec<CreateCommandOption>> {
        Some(vec![CreateCommandOption::new(
            CommandOptionType::Integer,
            "achievement",
            "An achievement in LEGO Universe.",
        )
        .required(true)
        .set_autocomplete(true)])
    }

    type Arguments = AchievementArguments;

    fn handle_autocomplete(
        autocomplete_option: serenity::model::prelude::AutocompleteOption<'_>,
    ) -> Option<Vec<serenity::all::AutocompleteChoice>> {
        let input = autocomplete_option.value;
        Some(CD_CLIENT.autocomplete_achievement(input))
    }

    fn run(arguments: Self::Arguments) -> CommandResult {
        let AchievementArguments { achievement: id } = arguments;

        let mission = CD_CLIENT.get_achievement(id)?;
        let explorer_url = CD_CLIENT.achievement_explorer_url(id);
        let name = CD_CLIENT.req_achievement_name(id);
        let full_path = CD_CLIENT.full_achievement_path(id);

        let giver = mission.offer_object_id;
        let target = mission.target_object_id;
        let is_mission = giver.is_some_and(|id| id != -1) && target.is_some_and(|id| id != -1);
        let type_label = if is_mission { "Mission" } else { "Achievement" };

        let mut embed = CONFIG
            .default_embed()
            .title(format!("{full_path} [{id}]"))
            .url(explorer_url)
            .field(type_label, full_path, false);

        let objective = CD_CLIENT
            .locale()
            .mission_text
            .get(&id)
            .map(|txt| txt.in_progress.as_ref())
            .flatten();

        if let Some(objective) = objective {
            embed = embed.field("Objective", objective, false);
        }

        // pre reqs

        if let Some(pre_reqs) = &mission.prereq_mission_id {
            let content = pre_reqs
                .into_iter()
                .enumerate()
                .map(|(idx, req)| {
                    let num = idx + 1;
                    match req {
                        MissionPreReqType::OneOf(ids) => {
                            let names = ids
                                .into_iter()
                                .map(|id| CD_CLIENT.achievement_hyperlinked_name(*id))
                                .collect::<Vec<_>>()
                                .join(", ");
                            format!("**{num}.** *One of the following:* {names}")
                        }
                        MissionPreReqType::Required(id) => {
                            let name = CD_CLIENT.achievement_hyperlinked_name(*id);
                            format!("**{num}.** {name}")
                        }
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            embed = embed.field("Pre-Requisites", content, false)
        }

        // giver
        if is_mission {
            // checked earlier and made sure it was not -1 too
            let giver_name = CD_CLIENT.object_hyperlinked_name(giver.unwrap());
            let target_name = CD_CLIENT.object_hyperlinked_name(target.unwrap());
            embed = embed.fields([
                ("Accept From", giver_name, true),
                ("Return To", target_name, true),
            ])
        }

        // rewards
        if let Some(rewards) = CD_CLIENT.get_achievement_rewards(id) {
            let choose_one = mission.is_choice_reward.is_some_and(|is| is);
            let mut value = rewards
                .into_iter()
                .enumerate()
                .map(|(idx, r)| {
                    let name = CD_CLIENT.object_hyperlinked_name(r.item_id);
                    let count = r.count;
                    let num = idx + 1;
                    if choose_one {
                        format!("- {name} x `{count}`")
                    } else {
                        format!("**{num}.** {name} x `{count}`")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            if choose_one {
                value = format!("Choose One: {value}");
            }
            embed = embed.field("Rewards", value, false);
        }

        // emote rewards
        let emotes = vec![
            Some(mission.reward_emote),
            Some(mission.reward_emote2),
            mission.reward_emote3,
            mission.reward_emote4,
        ]
        .into_iter()
        .filter(|emote| emote.is_some_and(|e| e > 0))
        .enumerate()
        .map(|(idx, emote)| {
            let num = idx + 1;
            let emote = emote.unwrap();
            let name = CD_CLIENT
                .emotes
                .at_key(&emote)
                .map(|e| e.animation_name.as_ref().map(|name| name.to_title_case()))
                .flatten()
                .unwrap_or_else(|| format!("Emote {emote}"));
            format!("**{num}.** {name} `[{emote}]`")
        })
        .collect::<Vec<_>>();
        if emotes.len() > 0 {
            embed = embed.field("Emotes", emotes.join("\n"), true);
        }

        // stat rewards

        embed = embed.field(
            "LEGO Score",
            format!("`{}` Experience", mission.lego_score),
            true,
        );
        embed = embed.field(
            "Coins",
            format!("`{}` Coins", mission.reward_currency.unwrap_or(0)),
            true,
        );

        if let Some(value) = mission.reward_reputation {
            if value > 0 {
                embed = embed.field("Reputation", format!("`{}` Reputation", value), true);
            }
        }

        if mission.reward_maxinventory != 0 {
            embed = embed.field(
                "Backpack Slots",
                format!("`{}` Backpack Slots", mission.reward_maxinventory),
                true,
            );
        }

        if let Some(value) = mission.reward_maxmodel {
            if value > 0 {
                embed = embed.field("Model Slots", format!("`{}` Model Slots", value), true);
            }
        }

        if let Some(value) = mission.reward_bankinventory {
            if value > 0 {
                embed = embed.field("Vault Slots", format!("`{}` Vault Slots", value), true);
            }
        }

        if mission.reward_maxhealth != 0 {
            embed = embed.field(
                "Health",
                format!("`{}` Health", mission.reward_maxhealth),
                true,
            );
        }

        if mission.reward_maximagination != 0 {
            embed = embed.field(
                "Imagination",
                format!("`{}` Imagination", mission.reward_maximagination),
                true,
            );
        }

        if let Some(url) = CD_CLIENT.achievement_icon_url(id) {
            embed = embed.thumbnail(url);
        }

        Ok((embed, None))
    }
}
