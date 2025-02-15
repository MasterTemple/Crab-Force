use std::fmt::Display;

use serenity::all::AutocompleteChoice;

use crate::{
    cdclient::{
        components::{ITEM_COMPONENT, PACKAGE_COMPONENT, RENDER_COMPONENT, VENDOR_COMPONENT},
        CdClient, ItemComponent, Missions, Objects, PackageComponent, RenderComponent,
        SkillBehavior, VendorComponent,
    },
    locale::LocaleTranslation,
    CD_CLIENT, CONFIG, LOCALE_XML,
};

pub struct Queries<'a>(&'a CdClient);

pub type MsgResult<T> = Result<T, String>;

pub trait LocaleQueries {
    fn locale(&self) -> &LocaleTranslation {
        LOCALE_XML.locales.get(&CONFIG.locale).unwrap()
    }
}

pub fn explorer_link_name(
    name: impl Display,
    id: impl Display,
    explorer_url: impl Display,
) -> String {
    format!("{name} [`[{id}]`]({explorer_url})")
}

impl LocaleQueries for CdClient {}

//------------------//
// Auto-Completions //
//------------------//
pub trait AutocompleteQueries {
    fn autocomplete_object(&self, input: &str) -> Vec<AutocompleteChoice>;
    fn autocomplete_achievement(&self, input: &str) -> Vec<AutocompleteChoice>;
    fn autocomplete_mission(&self, input: &str) -> Vec<AutocompleteChoice>;
    // fn autocomplete_item(input: &str) -> Vec<&Objects>;
    // fn autocomplete_enemy(input: &str) -> Vec<&Objects>;
    // fn autocomplete_brick(input: &str) -> Vec<&Objects>;
    fn autocomplete_skill(&self, input: &str) -> Vec<AutocompleteChoice>;
}

impl AutocompleteQueries for CdClient {
    // this can be greatly improved, but that is for later
    fn autocomplete_object(&self, input: &str) -> Vec<AutocompleteChoice> {
        if input.len() == 0 {
            return vec![];
        }
        self.objects
            .iter()
            // .filter(|item| item.name.len() > 0)
            .map(|item| {
                let id = item.id;
                let name = item
                    .display_name
                    .clone()
                    .unwrap_or_else(|| item.name.clone().unwrap_or_else(|| format!("Item {id}")));
                // dbg!(&item);
                (id, name)
                // let name = item.name.clone();
                // .clone()
                // .unwrap_or_else(|| item.name.clone());
                // AutocompleteChoice::new(format!("[{id}] {name}"), id)
            })
            .filter(|(_, name)| name.to_lowercase().contains(input))
            .take(25)
            .map(|(id, name)| AutocompleteChoice::new(format!("[{id}] {name}"), id))
            .collect()
    }

    fn autocomplete_skill(&self, input: &str) -> Vec<AutocompleteChoice> {
        if input.len() == 0 {
            return vec![];
        }
        self.skill_behavior
            .iter()
            .map(|skill| {
                let id = skill.skill_id;
                let name = self.req_skill_name(id);
                (id, name)
            })
            .filter(|(_, name)| name.to_lowercase().contains(input))
            .take(25)
            .map(|(id, name)| AutocompleteChoice::new(format!("[{id}] {name}"), id))
            .collect()
    }

    fn autocomplete_achievement(&self, input: &str) -> Vec<AutocompleteChoice> {
        if input.len() == 0 {
            return vec![];
        }
        self.locale()
            .missions
            .iter()
            .map(|(id, mission)| {
                let name = mission
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("Mission {id}"));
                (id, name)
            })
            .filter(|(_, name)| name.to_lowercase().contains(input))
            .take(25)
            .map(|(id, name)| AutocompleteChoice::new(format!("[{id}] {name}"), *id))
            .collect()
    }

    fn autocomplete_mission(&self, input: &str) -> Vec<AutocompleteChoice> {
        if input.len() == 0 {
            return vec![];
        }
        self.locale()
            .missions
            .iter()
            .map(|(id, mission)| {
                let name = mission
                    .name
                    .clone()
                    .unwrap_or_else(|| format!("Mission {id}"));
                (id, name)
            })
            .filter(|(_, name)| name.to_lowercase().contains(input))
            .take(25)
            .map(|(id, name)| AutocompleteChoice::new(format!("[{id}] {name}"), *id))
            .collect()
    }
}

pub fn fix_icon_asset(asset: &str) -> String {
    asset
        .replace("\\", "/")
        .replace("../", "")
        .replace("./", "")
}

pub fn icon_asset_as_url(asset: &str) -> String {
    CONFIG.explorer_res_uri(&fix_icon_asset(asset))
}

pub trait LootQueries {
    fn loot_table_indexes_with_item(&self, item_id: i32) -> Option<Vec<i32>>;
    fn loot_matrix_indexes_with_item(&self, item_id: i32) -> Option<Vec<i32>>;
    fn items_in_loot_matrix_index(&self, lmi: i32) -> Option<Vec<i32>>;
    fn items_in_loot_table_index(&self, lti: i32) -> Option<Vec<i32>>;
}

impl LootQueries for CdClient {
    fn loot_table_indexes_with_item(&self, item_id: i32) -> Option<Vec<i32>> {
        let ltis: Vec<i32> = self
            .loot_table
            .iter()
            .filter(|lt| lt.itemid == item_id)
            .map(|lt| lt.loot_table_index)
            .collect();
        (ltis.len() != 0).then_some(ltis)
    }

    fn loot_matrix_indexes_with_item(&self, item_id: i32) -> Option<Vec<i32>> {
        let ltis = self.loot_table_indexes_with_item(item_id)?;
        let lmis: Vec<i32> = self
            .loot_matrix
            .iter()
            .filter(|lm| ltis.contains(&lm.loot_table_index))
            .map(|lm| lm.loot_matrix_index)
            .collect();
        (lmis.len() != 0).then_some(lmis)
    }

    fn items_in_loot_matrix_index(&self, lmi: i32) -> Option<Vec<i32>> {
        let item_ids: Vec<i32> = self
            .loot_matrix
            .iter()
            .filter(|lm| lm.loot_table_index == lmi)
            .filter_map(|lm| self.items_in_loot_table_index(lm.loot_table_index))
            .flatten()
            .collect();
        (item_ids.len() != 0).then_some(item_ids)
    }

    fn items_in_loot_table_index(&self, lti: i32) -> Option<Vec<i32>> {
        let item_ids: Vec<i32> = self
            .loot_table
            .at_group_key(&lti)?
            .iter()
            .map(|lt| lt.itemid)
            .collect();
        (item_ids.len() != 0).then_some(item_ids)
    }
}

pub trait ObjectQueries {
    fn object_name(&self, item_id: i32) -> Option<String>;

    fn req_object_name(&self, item_id: i32) -> String;

    fn object_hyperlinked_name(&self, item_id: i32) -> String;

    fn get_object(&self, item_id: i32) -> MsgResult<&Objects>;

    fn object_item_component(&self, item_id: i32) -> MsgResult<&ItemComponent>;

    fn object_explorer_url(&self, item_id: i32) -> String;

    fn object_render_component(&self, item_id: i32) -> MsgResult<&RenderComponent>;

    fn object_icon_url(&self, item_id: i32) -> Option<String>;
    /// returns vendor ids
    fn object_vendor_ids(&self, item_id: i32) -> MsgResult<Vec<i32>>;

    fn object_package_ids(&self, id: i32) -> MsgResult<Vec<i32>>;

    fn object_package_component(&self, item_id: i32) -> MsgResult<&PackageComponent>;
}

impl ObjectQueries for CdClient {
    fn object_package_component(&self, item_id: i32) -> MsgResult<&PackageComponent> {
        let components = self
            .components_registry
            .at_group_key(&item_id)
            .ok_or_else(|| {
                format!(
                    "{} has no Registered Components",
                    self.object_explorer_url(item_id)
                )
            })?;

        let item_component_id = components
            .iter()
            .find(|comp| comp.component_type == PACKAGE_COMPONENT)
            .ok_or_else(|| {
                format!(
                    "{} has no Registered Package Component",
                    self.object_explorer_url(item_id)
                )
            })?
            .component_id;

        let item_component = self
            .package_component
            .at_key(&item_component_id)
            .ok_or_else(|| format!("Package Component `{}` does not exist", item_component_id))?;

        Ok(item_component)
    }

    fn object_package_ids(&self, item_id: i32) -> MsgResult<Vec<i32>> {
        let lmis = self.loot_matrix_indexes_with_item(item_id).ok_or_else(|| {
            format!(
                "{} it not in any Loot Matrices",
                self.object_explorer_url(item_id)
            )
        })?;

        let pkg_ids: Vec<i32> = self
            .package_component
            .iter()
            .filter(|pkg| lmis.contains(&pkg.loot_matrix_index))
            .map(|pkg| pkg.id)
            .collect();

        let object_ids = self
            .components_registry
            .iter()
            .filter(|cr| {
                cr.component_type == PACKAGE_COMPONENT && pkg_ids.contains(&cr.component_id)
            })
            .map(|cr| cr.id)
            .collect();

        Ok(object_ids)
    }

    fn object_name(&self, item_id: i32) -> Option<String> {
        let item = self.objects.at_key(&item_id)?;
        LOCALE_XML
            .locales
            .get(&CONFIG.locale)
            .unwrap()
            .objects
            .get(&item_id)
            .map(|o| o.name.clone())
            .flatten()
            .or_else(|| item.display_name.clone().or_else(|| item.name.clone()))
    }

    fn req_object_name(&self, item_id: i32) -> String {
        self.object_name(item_id)
            .unwrap_or_else(|| format!("Object {item_id}"))
    }

    fn object_hyperlinked_name(&self, item_id: i32) -> String {
        let name = self.req_object_name(item_id);
        let explorer_url = self.object_explorer_url(item_id);
        explorer_link_name(name, item_id, explorer_url)
    }

    fn get_object(&self, item_id: i32) -> MsgResult<&Objects> {
        CD_CLIENT.objects.at_key(&item_id).ok_or_else(|| {
            format!(
                "{} does not exist!",
                CD_CLIENT.object_hyperlinked_name(item_id)
            )
        })
    }

    fn object_item_component(&self, item_id: i32) -> MsgResult<&ItemComponent> {
        let components = self
            .components_registry
            .at_group_key(&item_id)
            .ok_or_else(|| {
                format!(
                    "{} has no Registered Components",
                    self.object_explorer_url(item_id)
                )
            })?;

        let item_component_id = components
            .iter()
            .find(|comp| comp.component_type == ITEM_COMPONENT)
            .ok_or_else(|| {
                format!(
                    "{} has no Registered Item Component",
                    self.object_explorer_url(item_id)
                )
            })?
            .component_id;

        let item_component = self
            .item_component
            .at_key(&item_component_id)
            .ok_or_else(|| format!("Item Component `{}` does not exist", item_component_id))?;

        Ok(item_component)
    }

    fn object_explorer_url(&self, item_id: i32) -> String {
        CONFIG.explorer_uri(format!("/objects/{}", item_id))
    }

    fn object_render_component(&self, item_id: i32) -> MsgResult<&RenderComponent> {
        let components = self
            .components_registry
            .at_group_key(&item_id)
            .ok_or_else(|| {
                format!(
                    "{} has no Registered Components",
                    self.object_explorer_url(item_id)
                )
            })?;

        let render_component_id = components
            .iter()
            .find(|comp| comp.component_type == RENDER_COMPONENT)
            .ok_or_else(|| {
                format!(
                    "{} has no Registered Render Component",
                    self.object_explorer_url(item_id)
                )
            })?
            .component_id;

        let render_component = self
            .render_component
            .at_key(&render_component_id)
            .ok_or_else(|| format!("Render Component `{}` does not exist", render_component_id))?;

        Ok(render_component)
    }

    fn object_icon_url(&self, item_id: i32) -> Option<String> {
        let render_component = self.object_render_component(item_id).ok()?;
        Some(icon_asset_as_url(render_component.icon_asset.as_ref()?))
    }

    fn object_vendor_ids(&self, item_id: i32) -> MsgResult<Vec<i32>> {
        let lmis = self.loot_matrix_indexes_with_item(item_id).ok_or_else(|| {
            format!(
                "{} it not in any Loot Matrices",
                self.object_explorer_url(item_id)
            )
        })?;
        let vc_ids: Vec<i32> = self
            .vendor_component
            .iter()
            .filter(|vc| lmis.contains(&vc.loot_matrix_index))
            .map(|vc| vc.id)
            .collect();
        let object_ids = self
            .components_registry
            .iter()
            .filter(|cr| cr.component_type == VENDOR_COMPONENT && vc_ids.contains(&cr.component_id))
            .map(|cr| cr.id)
            .collect();
        Ok(object_ids)
    }
}

// impl<'a> Queries<'a> {
//     // need some kind of meta object with render component, item component, and so on
//     pub fn object(id: i32) -> &'a Objects {
//         todo!()
//     }
//
//     pub fn object_name(item_id: i32) -> Option<String> {
//         let item = CD_CLIENT.objects.at_key(&item_id)?;
//         LOCALE_XML
//             .locales
//             .get(&CONFIG.locale)
//             .unwrap()
//             .objects
//             .get(&item_id)
//             .map(|o| o.name.clone())
//             .flatten()
//             .or_else(|| item.display_name.clone().or_else(|| item.name.clone()))
//     }
//
//     pub fn object_item_component(item_id: i32) -> Option<&'a ItemComponent> {
//         let components = CD_CLIENT.components_registry.at_group_key(&item_id)?;
//
//         let item_component_id = components
//             .iter()
//             .find(|comp| comp.component_type == ITEM_COMPONENT)?
//             .component_id;
//
//         let item_component = CD_CLIENT.item_component.at_key(&item_component_id)?;
//
//         Some(item_component)
//     }
//
//     pub fn object_render_component(item_id: i32) -> Option<&'a RenderComponent> {
//         let components = CD_CLIENT.components_registry.at_group_key(&item_id)?;
//
//         let render_component_id = components
//             .iter()
//             .find(|comp| comp.component_type == RENDER_COMPONENT)?
//             .component_id;
//
//         let render_component = CD_CLIENT.render_component.at_key(&render_component_id)?;
//
//         Some(render_component)
//     }
//
//     pub fn object_icon_url(item_id: i32) -> Option<String> {
//         let render_component = Self::object_render_component(item_id)?;
//         Some(icon_asset_as_url(render_component.icon_asset.as_ref()?))
//     }
// }

pub trait SkillQueries {
    fn skill_name(&self, id: i32) -> Option<String>;
    fn req_skill_name(&self, id: i32) -> String;
    fn skill_icon_url(&self, id: i32) -> Option<String>;
    fn skill_explorer_url(&self, id: i32) -> String;
    fn skill_hyperlinked_name(&self, id: i32) -> String;
    fn get_skill(&self, id: i32) -> MsgResult<&SkillBehavior>;
    fn cooldown_group_hyperlinked_name(&self, cdg: i32) -> String;
}

impl SkillQueries for CdClient {
    fn get_skill(&self, id: i32) -> MsgResult<&SkillBehavior> {
        self.skill_behavior
            .at_key(&id)
            .ok_or_else(|| format!("{} does not exist!", CD_CLIENT.skill_hyperlinked_name(id)))
    }

    fn skill_name(&self, id: i32) -> Option<String> {
        self.locale()
            .skill_behavior
            .get(&id)
            .map(|skill_behavior| skill_behavior.name.clone())
            .flatten()
    }

    fn req_skill_name(&self, id: i32) -> String {
        self.skill_name(id).unwrap_or_else(|| format!("Skill {id}"))
    }

    fn skill_explorer_url(&self, id: i32) -> String {
        CONFIG.explorer_uri(format!("/skills/{}", id))
    }
    fn cooldown_group_hyperlinked_name(&self, id: i32) -> String {
        let url = CONFIG.explorer_uri(format!("/skills/cooldowngroups/{}", id));
        explorer_link_name(format!("Group {id}"), id, url)
    }

    fn skill_hyperlinked_name(&self, id: i32) -> String {
        let name = self.req_skill_name(id);
        let explorer_url = self.skill_explorer_url(id);
        explorer_link_name(name, id, explorer_url)
    }

    fn skill_icon_url(&self, id: i32) -> Option<String> {
        let skill = self.skill_behavior.at_key(&id)?;
        self.get_icon_url(skill.skill_icon?)
    }
}

pub trait IconQueries {
    fn get_icon_url(&self, icon_id: i32) -> Option<String>;
}
impl IconQueries for CdClient {
    fn get_icon_url(&self, icon_id: i32) -> Option<String> {
        let icon = self.icons.at_key(&icon_id)?;
        Some(icon_asset_as_url(icon.icon_path.as_ref()?))
    }
}

pub trait AchievementQueries {
    fn achievement_name(&self, id: i32) -> Option<String>;
    fn req_achievement_name(&self, id: i32) -> String;
    fn achievement_icon_url(&self, id: i32) -> Option<String>;
    fn achievement_explorer_url(&self, id: i32) -> String {
        CONFIG.explorer_uri(format!("/missions/{}", id))
    }
    fn achievement_hyperlinked_name(&self, id: i32) -> String {
        let name = self.req_achievement_name(id);
        let url = CONFIG.explorer_uri(format!("/missions/{}", id));
        explorer_link_name(name, id, url)
    }
    fn get_achievement(&self, id: i32) -> MsgResult<&Missions>;
    fn full_achievement_path(&self, id: i32) -> String;
    fn get_achievement_rewards(&self, id: i32) -> Option<Vec<MissionReward>>;
}

pub struct MissionReward {
    pub item_id: i32,
    pub count: i32,
    pub repeatable: bool,
    pub repeat_count: i32,
}

impl AchievementQueries for CdClient {
    // let tasks = self.mission_tasks.iter().find(|mt| mt.id == id)?;
    fn get_achievement_rewards(&self, id: i32) -> Option<Vec<MissionReward>> {
        let mission = self.get_achievement(id).ok()?;
        let mut rewards = vec![];
        if mission.reward_item1 != -1 {
            rewards.push(MissionReward {
                item_id: mission.reward_item1,
                count: mission.reward_item1_count,
                repeatable: mission.reward_item1_repeatable != 0,
                repeat_count: mission.reward_item1_repeat_count,
            });
        }
        if mission.reward_item2 != -1 {
            rewards.push(MissionReward {
                item_id: mission.reward_item2,
                count: mission.reward_item2_count,
                repeatable: mission.reward_item2_repeatable != 0,
                repeat_count: mission.reward_item2_repeat_count,
            });
        }
        if mission.reward_item3 != -1 {
            rewards.push(MissionReward {
                item_id: mission.reward_item3,
                count: mission.reward_item3_count,
                repeatable: mission.reward_item3_repeatable != 0,
                repeat_count: mission.reward_item3_repeat_count,
            });
        }
        if mission.reward_item4 != -1 {
            rewards.push(MissionReward {
                item_id: mission.reward_item4,
                count: mission.reward_item4_count,
                repeatable: mission.reward_item4_repeatable != 0,
                repeat_count: mission.reward_item4_repeat_count,
            });
        }
        Some(rewards)
    }

    fn full_achievement_path(&self, id: i32) -> String {
        let name = self.req_achievement_name(id);
        let Some(mission) = self.missions.at_key(&id) else {
            return name;
        };
        let top = &mission.defined_type;
        let middle = &mission.defined_subtype;
        if let Some(middle) = middle {
            format!("{top} > {middle} > {name}")
        } else {
            format!("{top} > {name}")
        }
    }

    fn achievement_name(&self, id: i32) -> Option<String> {
        self.locale().missions.get(&id)?.name.clone()
    }

    fn req_achievement_name(&self, id: i32) -> String {
        self.achievement_name(id)
            .unwrap_or_else(|| format!("Achievement {id}"))
    }

    // Look into cdclient `Missions > missionIconID`
    fn achievement_icon_url(&self, id: i32) -> Option<String> {
        let url = self
            .mission_tasks
            .at_group_key(&id)
            .map(|slice| {
                let url = slice.into_iter().find_map(|e| {
                    self.get_icon_url(e.icon_id?)
                        .or_else(|| self.get_icon_url(e.large_task_icon_id?))
                });
                url
            })
            .flatten();
        url
    }

    fn get_achievement(&self, id: i32) -> MsgResult<&Missions> {
        self.missions.at_key(&id).ok_or_else(|| {
            format!(
                "{} does not exist!",
                CD_CLIENT.achievement_hyperlinked_name(id)
            )
        })
    }
}
