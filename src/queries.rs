use std::fmt::Display;

use serenity::all::AutocompleteChoice;

use crate::{
    cdclient::{
        components::{ITEM_COMPONENT, RENDER_COMPONENT, VENDOR_COMPONENT},
        CdClient, ItemComponent, Objects, RenderComponent, SkillBehavior, VendorComponent,
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
}

impl ObjectQueries for CdClient {
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
        let icon = self.icons.at_key(&skill.skill_icon?)?;
        Some(icon_asset_as_url(icon.icon_path.as_ref()?))
    }
}
