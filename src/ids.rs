use std::{fmt::Display, ops::Deref};

use crate::{
    cdclient::{
        components::{
            DESTROYABLE_COMPONENT, ITEM_COMPONENT, PACKAGE_COMPONENT, RENDER_COMPONENT,
            VENDOR_COMPONENT,
        },
        CdClientItemComponent, CdClientLootMatrix, CdClientLootTable, CdClientObjects,
        CdClientRarityTable, CdClientRenderComponent, CdClientVendorComponent,
    },
    custom::CollectIntoOptionalVec,
    Api, CONFIG,
};

pub type MsgResult<T> = Result<T, String>;

pub fn explorer_hyperlink(
    name: impl Display,
    id: impl Display,
    explorer_url: impl Display,
) -> String {
    format!("{name} [`[{id}]`]({explorer_url})")
}

pub fn fix_icon_asset(asset: &str) -> String {
    asset
        .replace("\\", "/")
        .replace("../", "")
        .replace("./", "")
}

pub fn icon_asset_as_url(asset: impl AsRef<str>) -> String {
    CONFIG.explorer_res_uri(&fix_icon_asset(asset.as_ref()))
}

/*
* Here is the plan:
* [x] 1. prefix all structs cdclient with 'CdClient'
* [ ] 2. Create new-type to store id
* [ ] 3. Impl trait to give it access to cdclient/locale/config
* [ ] 4. Update HasKey and HasGroupKey to use 'CdClient<ident>Id' so that way I don't pass the wrong kind of id
* [ ] 5. Make sure I can pass `CdClientObjectsId` to something that takes i32 (or do comparisons)
* [ ] 6. Perhaps with the HasKey, impl a trait with an id() method and a get() method
* [ ] 7. Actually replace the i32 ids with `CdClient{ident}Id` in `CdClient{ident}` structs (idk about that)
* [ ] 8. Consider where custom structs come in; because i can't have a mere newtype wrapper and also
*    extra fields; i probably just want to return custom structs from newtype wrapper methods
*/

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientLootTableId(i32);
impl Api for CdClientLootTableId {}
impl LUExplorer for CdClientLootTableId {
    const NAME: &'static str = "Loot Table";
    const ENDPOINT: &'static str = "objects/loot/table";

    fn id(&self) -> i32 {
        self.0
    }
}

impl CdClientLootTableId {
    pub fn fetch(&self) -> MsgResult<Vec<CdClientLootTable>> {
        self.cdclient()
            .loot_table
            .at_group_key(&self.0)
            .clone()
            .ok_or_else(|| self.err("does not exist"))
    }

    pub fn contained_items(&self) -> Option<Vec<CdClientObjectsId>> {
        self.cdclient()
            .loot_table
            .at_group_key(&self.0)?
            .iter()
            .map(|lt| CdClientObjectsId(lt.itemid))
            .collect_some()
    }

    pub fn items_of_rarity(&self, rarity: i32) -> Option<usize> {
        Some(
            self.fetch()
                .ok()?
                .into_iter()
                .filter(|lt| {
                    CdClientObjectsId(lt.itemid)
                        .item_component()
                        .is_ok_and(|item| {
                            item.rarity.is_some_and(|item_rarity| item_rarity == rarity)
                        })
                })
                .count(),
        )
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientLootMatrixId(i32);
impl Api for CdClientLootMatrixId {}

impl CdClientLootMatrixId {
    pub fn fetch(&self) -> MsgResult<Vec<CdClientLootMatrix>> {
        self.cdclient()
            .loot_matrix
            .at_group_key(&self.0)
            .clone()
            .ok_or_else(|| format!("Loot Matrix `{}` does not exist", self.0))
    }

    pub fn contained_items(&self) -> Option<Vec<CdClientObjectsId>> {
        self.cdclient()
            .loot_matrix
            .at_group_key(&self.0)?
            .iter()
            .filter_map(|lm| {
                let lti = CdClientLootTableId(lm.loot_table_index);
                lti.contained_items()
            })
            .flatten()
            .collect_some()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientRarityTableId(i32);
impl Api for CdClientRarityTableId {}
impl CdClientRarityTableId {
    pub fn chance_to_drop_rarity(&self, rarity: i32) -> Option<f64> {
        let rarity_table = self.cdclient().rarity_table.at_group_key(&self.0)?;
        let upper_threshold = rarity_table.iter().find(|r| r.rarity == rarity)?.randmax;
        let lower_threshold = rarity_table
            .iter()
            .find_map(|r| (r.rarity == rarity - 1).then(|| r.randmax))
            .unwrap_or(0.0);
        let chance_to_drop_rarity = upper_threshold - lower_threshold;
        Some(chance_to_drop_rarity)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientObjectsId(i32);
impl Api for CdClientObjectsId {}

impl CdClientObjectsId {
    pub fn err(&self, msg: impl Display) -> String {
        format!("{} {}!", self.hyperlink_name(), msg)
    }

    pub fn fetch(&self) -> MsgResult<CdClientObjects> {
        self.cdclient()
            .objects
            .at_key(&self.0)
            .cloned()
            .ok_or_else(|| self.err("does not exist"))
    }

    pub fn name(&self) -> Option<String> {
        self.locale()
            .objects
            .get(&self.0)
            .and_then(|o| o.name.clone())
            .or_else(|| {
                let item = self.fetch().ok()?;
                item.display_name.clone().or_else(|| item.name.clone())
            })
    }

    pub fn req_name(&self) -> String {
        self.name().unwrap_or(format!("Item {}", self.0))
    }

    pub fn explorer_url(&self) -> String {
        self.config().explorer_uri(format!("/objects/{}", self.0))
    }

    pub fn hyperlink_name(&self) -> String {
        let name = self.req_name();
        let explorer_url = self.explorer_url();
        explorer_hyperlink(name, self.0, explorer_url)
    }

    pub fn get_containing_loot_table_indexes(&self) -> Option<Vec<CdClientLootTableId>> {
        self.cdclient()
            .loot_table
            .iter()
            .filter(|lt| lt.itemid == self.0)
            .map(|lt| CdClientLootTableId(lt.loot_table_index))
            .collect_some()
    }

    pub fn get_containing_loot_matrix_indexes(&self) -> Option<Vec<CdClientLootMatrixId>> {
        let ltis = self.get_containing_loot_table_indexes()?;
        self.cdclient()
            .loot_matrix
            .iter()
            .filter(|lm| ltis.contains(&CdClientLootTableId(lm.loot_table_index)))
            .map(|lm| CdClientLootMatrixId(lm.loot_matrix_index))
            .collect_some()
    }

    pub fn rarity(&self) -> MsgResult<i32> {
        self.item_component()?
            .rarity
            .ok_or_else(|| self.err("has no rarity"))
    }

    pub fn chance_from_loot_matrix_index(&self, lmi: CdClientLootMatrixId) -> MsgResult<f64> {
        let rarity = self.rarity()?;
        let ltis = self
            .get_containing_loot_table_indexes()
            .ok_or_else(|| self.err("is not in any Loot Tables"))?;

        // I think there is generally no repeats, but we will calculate anyway
        let lms: Vec<_> = lmi
            .fetch()?
            .into_iter()
            .filter(|lm| ltis.contains(&CdClientLootTableId(lm.loot_table_index)))
            .collect();

        let total_chance: f64 = lms
            .into_iter()
            .filter_map(|lm| {
                let chance_to_drop_loot_table = lm.percent;
                let number_of_items_of_rarity_in_loot_table =
                    CdClientLootTableId(lm.loot_table_index).items_of_rarity(rarity)?;
                let chance_to_drop_rarity =
                    CdClientRarityTableId(lm.rarity_table_index).chance_to_drop_rarity(rarity)?;

                let chance = chance_to_drop_rarity
                    * chance_to_drop_loot_table
                    * (1.0 / number_of_items_of_rarity_in_loot_table as f64);
                let avg_dropped = (lm.min_to_drop as f64 + lm.max_to_drop as f64) / 2.0;

                Some(avg_dropped * chance)
            })
            .sum();

        Ok(total_chance)
    }

    pub fn render_component(&self) -> MsgResult<CdClientRenderComponent> {
        let components = self
            .cdclient()
            .components_registry
            .at_group_key(&self.0)
            .ok_or_else(|| self.err("has no Registered Components"))?;

        let render_component_id = components
            .iter()
            .find(|comp| comp.component_type == RENDER_COMPONENT)
            .ok_or_else(|| self.err("has no Registered render Component"))?
            .component_id;

        let render_component = self
            .cdclient()
            .render_component
            .at_key(&render_component_id)
            .ok_or_else(|| format!("Render Component `{}` does not exist", render_component_id))?
            .clone();

        Ok(render_component)
    }

    pub fn thumbnail(&self) -> Option<String> {
        let render_component = self.render_component().ok()?;
        Some(icon_asset_as_url(render_component.icon_asset?))
    }

    pub fn item_component(&self) -> MsgResult<CdClientItemComponent> {
        let components = self
            .cdclient()
            .components_registry
            .at_group_key(&self.0)
            .ok_or_else(|| self.err("has no Registered Components"))?;

        let item_component_id = components
            .iter()
            .find(|comp| comp.component_type == ITEM_COMPONENT)
            .ok_or_else(|| self.err("has no Registered Item Component"))?
            .component_id;

        let item_component = self
            .cdclient()
            .item_component
            .at_key(&item_component_id)
            .ok_or_else(|| format!("Item Component `{}` does not exist", item_component_id))?
            .clone();

        Ok(item_component)
    }

    /// All achievements/missions that earn an object
    // pub fn achievements(&self) -> MsgResult<Vec<CdClientActivityRewardsId>> {
    //     let lmis = self
    //         .get_containing_loot_matrix_indexes()
    //         .ok_or_else(|| self.err("it not in any Loot Matrices"))?;
    //
    //     let activities: Vec<_> = self
    //         .cdclient()
    //         .activity_rewards
    //         .iter()
    //         .filter(|activity| {
    //             activity
    //                 .loot_matrix_index
    //                 .is_some_and(|lmi| lmis.contains(&CdClientLootMatrixId(lmi)))
    //         })
    //         .map(|comp| CdClientActivityRewardsId(comp.object_template))
    //         .collect();
    //     Ok(activities)
    // }

    /// All activities that reward an object
    pub fn activities(&self) -> MsgResult<Vec<CdClientActivityRewardsId>> {
        let lmis = self
            .get_containing_loot_matrix_indexes()
            .ok_or_else(|| self.err("is not in any Loot Matrices"))?;

        let activities: Vec<_> = self
            .cdclient()
            .activity_rewards
            .iter()
            .filter(|activity| {
                activity
                    .loot_matrix_index
                    .is_some_and(|lmi| lmis.contains(&CdClientLootMatrixId(lmi)))
            })
            .map(|comp| CdClientActivityRewardsId(comp.object_template))
            .collect();
        Ok(activities)
    }

    /// All smashables that drop an object
    pub fn smashables(&self) -> MsgResult<Vec<CdClientObjectsId>> {
        let lmis = self
            .get_containing_loot_matrix_indexes()
            .ok_or_else(|| self.err("is not in any Loot Matrices"))?;

        let smashables: Vec<_> = self
            .cdclient()
            .destructible_component
            .iter()
            .filter(|comp| {
                comp.loot_matrix_index
                    .is_some_and(|lmi| lmis.contains(&CdClientLootMatrixId(lmi)))
            })
            .map(|comp| CdClientDestructibleComponentId(comp.id))
            .filter_map(|comp| comp.get_objects_with_component())
            .flatten()
            .collect();
        Ok(smashables)
    }

    /// All packages that unpack an object
    pub fn packages(&self) -> MsgResult<Vec<CdClientObjectsId>> {
        let lmis = self
            .get_containing_loot_matrix_indexes()
            .ok_or_else(|| self.err("is not in any Loot Matrices"))?;

        let packages: Vec<_> = self
            .cdclient()
            .package_component
            .iter()
            .filter(|comp| lmis.contains(&CdClientLootMatrixId(comp.loot_matrix_index)))
            .map(|comp| CdClientPackageComponentId(comp.id))
            .filter_map(|comp| comp.get_objects_with_component())
            .flatten()
            .collect();
        Ok(packages)
    }

    /// All vendors that sell an object
    pub fn vendors(&self) -> MsgResult<Vec<CdClientObjectsId>> {
        let lmis = self
            .get_containing_loot_matrix_indexes()
            .ok_or_else(|| self.err("is not in any Loot Matrices"))?;

        let vendors: Vec<_> = self
            .cdclient()
            .vendor_component
            .iter()
            .filter(|comp| lmis.contains(&CdClientLootMatrixId(comp.loot_matrix_index)))
            .map(|comp| CdClientVendorComponentId(comp.id))
            .filter_map(|comp| comp.get_objects_with_component())
            .flatten()
            .collect();

        Ok(vendors)
    }
}

pub trait LUExplorer: Api {
    const NAME: &'static str;
    /// **DO NOT PUT FRONT/BACK SLASHES**
    const ENDPOINT: &'static str;

    fn id(&self) -> i32;

    fn err(&self, msg: impl Display) -> String {
        format!("{} {}!", self.hyperlink_name(), msg)
    }

    fn explorer_url(&self) -> String {
        self.config()
            .explorer_uri(format!("/{}/{}", Self::ENDPOINT, self.id()))
    }

    fn name(&self) -> Option<String> {
        None
    }

    fn req_name(&self) -> String {
        self.name()
            .unwrap_or(format!("{} {}", Self::NAME, self.id()))
    }

    fn hyperlink_name(&self) -> String {
        let name = self.req_name();
        let explorer_url = self.explorer_url();
        explorer_hyperlink(name, self.id(), explorer_url)
    }
}

pub trait ComponentId: Api {
    const ID: i32;
    const NAME: &'static str;

    fn id(&self) -> i32;

    fn err(&self, msg: impl Display) -> String {
        format!("{} {}!", self.hyperlink_name(), msg)
    }

    fn explorer_url(&self) -> String {
        self.config()
            .explorer_uri(format!("/objects/components/{}", Self::ID))
    }

    fn name(&self) -> Option<String> {
        None
    }

    fn req_name(&self) -> String {
        self.name()
            .unwrap_or(format!("{} Component {}", Self::NAME, self.id()))
    }

    fn hyperlink_name(&self) -> String {
        let name = self.req_name();
        let explorer_url = self.explorer_url();
        explorer_hyperlink(name, self.id(), explorer_url)
    }

    fn get_objects_with_component(&self) -> Option<Vec<CdClientObjectsId>> {
        let component_id = self.id();
        self.cdclient()
            .components_registry
            .iter()
            .filter(|cr| cr.component_type == Self::ID && component_id == cr.component_id)
            .map(|cr| CdClientObjectsId(cr.id))
            .collect_some()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientDestructibleComponentId(i32);
impl Api for CdClientDestructibleComponentId {}
impl ComponentId for CdClientDestructibleComponentId {
    const ID: i32 = DESTROYABLE_COMPONENT;
    const NAME: &'static str = "Destructible";
    fn id(&self) -> i32 {
        self.0
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientPackageComponentId(i32);
impl Api for CdClientPackageComponentId {}
impl ComponentId for CdClientPackageComponentId {
    const ID: i32 = PACKAGE_COMPONENT;
    const NAME: &'static str = "Package";
    fn id(&self) -> i32 {
        self.0
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientVendorComponentId(i32);
impl Api for CdClientVendorComponentId {}
impl ComponentId for CdClientVendorComponentId {
    const ID: i32 = VENDOR_COMPONENT;
    const NAME: &'static str = "Vendor";
    fn id(&self) -> i32 {
        self.0
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientActivityRewardsId(i32);
impl Api for CdClientActivityRewardsId {}
impl CdClientActivityRewardsId {}
impl LUExplorer for CdClientActivityRewardsId {
    const NAME: &'static str = "Activity";
    const ENDPOINT: &'static str = "activities";

    fn id(&self) -> i32 {
        self.0
    }

    fn name(&self) -> Option<String> {
        self.locale()
            .activities
            .get(&self.0)?
            .activity_name
            .clone()
            .or_else(|| {
                let rewards = self.cdclient().activity_rewards.at_group_key(&self.0)?;
                Some(if rewards.len() == 1 {
                    rewards[0].description.clone()
                } else {
                    let desc = &rewards[0].description;
                    let num_idx = desc.find(char::is_numeric).unwrap_or(desc.len());
                    desc[0..num_idx].to_string()
                })
            })
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientComponentsRegistryId(Vec<i32>);
impl Api for CdClientComponentsRegistryId {}
impl CdClientComponentsRegistryId {
    pub fn get_objects_with_component<C: ComponentId>(
        &self,
        component: C,
    ) -> Option<Vec<CdClientObjectsId>> {
        let component_id = component.id();
        self.cdclient()
            .components_registry
            .iter()
            .filter(|cr| cr.component_type == C::ID && component_id == cr.component_id)
            .map(|cr| CdClientObjectsId(cr.id))
            .collect_some()
    }
}
