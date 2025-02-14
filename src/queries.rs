use serenity::all::AutocompleteChoice;

use crate::{
    cdclient::{
        components::{ITEM_COMPONENT, RENDER_COMPONENT},
        CdClient, ItemComponent, Objects, RenderComponent,
    },
    CD_CLIENT, CONFIG, LOCALE_XML,
};

pub struct Queries<'a>(&'a CdClient);

pub type MsgResult<T> = Result<T, String>;

//------------------//
// Auto-Completions //
//------------------//
pub trait AutocompleteQueries {
    fn autocomplete_object(&self, input: &str) -> Vec<AutocompleteChoice>;
    // fn autocomplete_item(input: &str) -> Vec<&Objects>;
    // fn autocomplete_enemy(input: &str) -> Vec<&Objects>;
    // fn autocomplete_brick(input: &str) -> Vec<&Objects>;
    // fn autocomplete_skill(input: &str) -> Vec<&Objects>;
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

pub trait ObjectQueries {
    fn object_name(&self, item_id: i32) -> Option<String>;

    fn req_object_name(&self, item_id: i32) -> String;

    fn object_hyperlinked_name(&self, item_id: i32) -> String;

    fn get_object(&self, item_id: i32) -> MsgResult<&Objects>;

    fn object_item_component(&self, item_id: i32) -> MsgResult<&ItemComponent>;

    fn object_explorer_url(&self, item_id: i32) -> String;

    fn object_render_component(&self, item_id: i32) -> Option<&RenderComponent>;

    fn object_icon_url(&self, item_id: i32) -> Option<String>;
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
        format!("{name} [`[{item_id}]`]({explorer_url})")
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
            .ok_or_else(|| format!("Item Component {} does not exist", item_component_id))?;

        Ok(item_component)
    }

    fn object_explorer_url(&self, item_id: i32) -> String {
        CONFIG.explorer_uri(format!("/objects/{}", item_id))
    }

    fn object_render_component(&self, item_id: i32) -> Option<&RenderComponent> {
        let components = self.components_registry.at_group_key(&item_id)?;

        let render_component_id = components
            .iter()
            .find(|comp| comp.component_type == RENDER_COMPONENT)?
            .component_id;

        let render_component = self.render_component.at_key(&render_component_id)?;

        Some(render_component)
    }

    fn object_icon_url(&self, item_id: i32) -> Option<String> {
        let render_component = self.object_render_component(item_id)?;
        Some(icon_asset_as_url(render_component.icon_asset.as_ref()?))
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
