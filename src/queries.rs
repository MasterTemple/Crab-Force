use crate::{
    cdclient::{
        components::{ITEM_COMPONENT, RENDER_COMPONENT},
        CdClient, ItemComponent, Objects, RenderComponent,
    },
    CD_CLIENT, CONFIG, LOCALE_XML,
};

pub struct Queries<'a>(&'a CdClient);

//------------------//
// Auto-Completions //
//------------------//
impl<'a> Queries<'a> {
    pub fn autocomplete_object(input: &str) -> Vec<&'a Objects> {
        todo!()
    }

    pub fn autocomplete_item(input: &str) -> Vec<&'a Objects> {
        todo!()
    }

    pub fn autocomplete_enemy(input: &str) -> Vec<&'a Objects> {
        todo!()
    }

    pub fn autocomplete_brick(input: &str) -> Vec<&'a Objects> {
        todo!()
    }

    pub fn autocomplete_skill(input: &str) -> Vec<&'a Objects> {
        todo!()
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

impl<'a> Queries<'a> {
    // need some kind of meta object with render component, item component, and so on
    pub fn object(id: i32) -> &'a Objects {
        todo!()
    }

    pub fn object_name(item_id: i32) -> Option<String> {
        let item = CD_CLIENT.objects.at_key(&item_id)?;
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

    pub fn object_item_component(item_id: i32) -> Option<&'a ItemComponent> {
        let components = CD_CLIENT.components_registry.at_group_key(&item_id)?;

        let item_component_id = components
            .iter()
            .find(|comp| comp.component_type == ITEM_COMPONENT)?
            .component_id;

        let item_component = CD_CLIENT.item_component.at_key(&item_component_id)?;

        Some(item_component)
    }

    pub fn object_render_component(item_id: i32) -> Option<&'a RenderComponent> {
        let components = CD_CLIENT.components_registry.at_group_key(&item_id)?;

        let render_component_id = components
            .iter()
            .find(|comp| comp.component_type == RENDER_COMPONENT)?
            .component_id;

        let render_component = CD_CLIENT.render_component.at_key(&render_component_id)?;

        Some(render_component)
    }

    pub fn object_icon_url(item_id: i32) -> Option<String> {
        let render_component = Self::object_render_component(item_id)?;
        Some(icon_asset_as_url(render_component.icon_asset.as_ref()?))
    }
}
