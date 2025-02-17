use std::{fmt::Display, ops::Deref};

use itertools::Itertools;

use crate::{
    cdclient::{
        components::{
            DESTROYABLE_COMPONENT, ITEM_COMPONENT, PACKAGE_COMPONENT, RENDER_COMPONENT,
            VENDOR_COMPONENT,
        },
        CdClientActivityRewards, CdClientDestructibleComponent, CdClientItemComponent,
        CdClientLootMatrix, CdClientLootTable, CdClientObjects, CdClientPackageComponent,
        CdClientRarityTable, CdClientRenderComponent, CdClientSkillBehavior,
        CdClientVendorComponent,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientLootTableId(pub i32);
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

    pub fn loot_chances(&self, rarity_table_index: i32) -> LootTableChances {
        let rti = CdClientRarityTableId(rarity_table_index);
        let lt = CdClientLootTableId(self.id());

        let t1 = rti.chance_to_drop_rarity(1).map(|chance| {
            let count = lt.items_of_rarity(1).unwrap_or(0);
            LootTableRarityGroup { count, chance }
        });

        let t2 = rti.chance_to_drop_rarity(2).map(|chance| {
            let count = lt.items_of_rarity(2).unwrap_or(0);
            LootTableRarityGroup { count, chance }
        });

        let t3 = rti.chance_to_drop_rarity(3).map(|chance| {
            let count = lt.items_of_rarity(3).unwrap_or(0);
            LootTableRarityGroup { count, chance }
        });

        let t4 = rti.chance_to_drop_rarity(4).map(|chance| {
            let count = lt.items_of_rarity(4).unwrap_or(0);
            LootTableRarityGroup { count, chance }
        });

        LootTableChances {
            lti: *self,
            t1,
            t2,
            t3,
            t4,
        }
    }
}

/// while there may be loot tables that dont drop certain rarities in that loot table, i don't care
pub struct LootTableRarityGroup {
    pub count: usize,
    pub chance: f64,
}
impl LootTableRarityGroup {
    pub fn chance_any(&self) -> f64 {
        self.chance
    }
    pub fn chance_specific(&self) -> f64 {
        self.chance / self.count as f64
    }
}

pub struct LootTableChances {
    pub lti: CdClientLootTableId,
    pub t1: Option<LootTableRarityGroup>,
    pub t2: Option<LootTableRarityGroup>,
    pub t3: Option<LootTableRarityGroup>,
    pub t4: Option<LootTableRarityGroup>,
}
impl LootTableChances {
    /// like health pickups
    pub fn has_no_rarity(&self) -> bool {
        self.t1.is_none() && self.t2.is_none() && self.t3.is_none() && self.t4.is_none()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientLootMatrixId(pub i32);
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

    pub fn loot_tables(&self) -> Option<Vec<CdClientLootTableId>> {
        self.fetch()
            .ok()?
            .into_iter()
            .map(|lm| CdClientLootTableId(lm.loot_table_index))
            .collect_some()
    }

    pub fn loot_chances(&self) -> Option<Vec<LootTableChances>> {
        let loot_tables = self.fetch().ok()?;
        loot_tables
            .into_iter()
            .map(|lm_entry| {
                CdClientLootTableId(lm_entry.loot_table_index)
                    .loot_chances(lm_entry.rarity_table_index)
            })
            .collect_some()
    }

    pub fn id(&self) -> i32 {
        self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientRarityTableId(pub i32);
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

#[derive(Clone, Debug)]
pub struct LootMatrixObjectChances {
    pub lmi: CdClientLootMatrixId,
    pub chance: f64,
    pub sources: Vec<CdClientObjectsId>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientObjectsId(pub i32);
impl Api for CdClientObjectsId {}

impl CdClientObjectsId {
    /// - This can cause infinite recursion when it calls [`Self::hyperlink_name`] which calls
    /// [`Self::try_fetch`], which if it fails calls [`Self::hyperlink_name`]
    /// - The solution is to make sure that [`Self::hyperlink_name`] does not require
    /// [`Self::try_fetch`] to succeed
    /// - So [`Self::req_name`] will check if the object exists or not
    pub fn err(&self, msg: impl Display) -> String {
        format!("{} {}!", self.hyperlink_name(), msg)
    }

    pub fn fetch(&self) -> Option<CdClientObjects> {
        self.cdclient().objects.at_key(&self.0).cloned()
    }

    pub fn is_hq_valid(&self) -> bool {
        self.fetch()
            .is_some_and(|ob| ob.hq_valid.is_some_and(|hq_valid| hq_valid))
    }

    pub fn try_fetch(&self) -> MsgResult<CdClientObjects> {
        self.fetch().ok_or_else(|| self.err("does not exist"))
    }

    pub fn name(&self) -> Option<String> {
        let item = self.fetch()?;
        item.display_name.clone().or_else(|| item.name.clone())
    }

    pub fn req_name(&self) -> String {
        self.name().unwrap_or_else(|| format!("Object {}", self.0))
        // let item = self.cdclient().objects.at_key(&self.0).cloned();
        // let f = || format!("Object {}", self.0);
        // item.and_then(f)
        // if item.is_some() {
        //     self.name().unwrap_or_else(f)
        // } else {
        //     // self.name().unwrap_or(format!("Object {}", self.0))
        //     f()
        // }
    }

    // this is the stack overflow??
    pub fn explorer_url(&self) -> String {
        self.config().explorer_uri(format!("/objects/{}", self.0))
    }

    pub fn hyperlink_name(&self) -> String {
        let name = self.req_name();
        let explorer_url = self.explorer_url();
        explorer_hyperlink(name, self.0, explorer_url)
    }

    pub fn get_component<C: ComponentId>(&self, component: impl Fn(i32) -> C) -> MsgResult<C> {
        self.cdclient()
            .components_registry
            .at_group_key(&self.0)
            .ok_or_else(|| self.err("has no Registered Components"))?
            .into_iter()
            .find(|cr| cr.component_type == C::ID)
            .map(|cr| component(cr.component_id))
            .ok_or_else(|| self.err(format!("has no Registered {} Component", C::NAME)))
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
            .filter_map(|lm_entry| {
                let chance_to_drop_loot_table = lm_entry.percent;
                let number_of_items_of_rarity_in_loot_table =
                    CdClientLootTableId(lm_entry.loot_table_index).items_of_rarity(rarity)?;
                let chance_to_drop_rarity = CdClientRarityTableId(lm_entry.rarity_table_index)
                    .chance_to_drop_rarity(rarity)?;

                let chance = chance_to_drop_rarity
                    * chance_to_drop_loot_table
                    * (1.0 / number_of_items_of_rarity_in_loot_table as f64);
                let avg_dropped = (lm_entry.min_to_drop as f64 + lm_entry.max_to_drop as f64) / 2.0;

                Some(avg_dropped * chance)
            })
            .sum();

        Ok(total_chance)
    }

    pub fn render_component(&self) -> MsgResult<CdClientRenderComponent> {
        self.get_component(CdClientRenderComponentId)?.fetch()
    }

    pub fn thumbnail(&self) -> Option<String> {
        let render_component = self.render_component().ok()?;
        Some(icon_asset_as_url(render_component.icon_asset?))
    }

    pub fn item_component(&self) -> MsgResult<CdClientItemComponent> {
        self.get_component(CdClientItemComponentId)?.fetch()
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

    // so i need to return the ids and their chances
    pub fn smashables_chances(&self) -> MsgResult<Vec<LootMatrixObjectChances>> {
        let lmis = self
            .get_containing_loot_matrix_indexes()
            .ok_or_else(|| self.err("is not in any Loot Matrices"))?;

        let smashables: Vec<_> = self
            .cdclient()
            .destructible_component
            .iter()
            // .filter(|comp| lmis.contains(&CdClientLootMatrixId(comp.loot_matrix_index?)))
            .filter(|comp| {
                comp.loot_matrix_index
                    .is_some_and(|lmi| lmis.contains(&CdClientLootMatrixId(lmi)))
            })
            .group_by(|comp| CdClientLootMatrixId(comp.loot_matrix_index.expect("Already checked")))
            .into_iter()
            .filter_map(|(lmi, comps)| {
                let sources: Vec<_> = comps
                    .filter_map(|comp| {
                        CdClientDestructibleComponentId(comp.id).get_objects_with_component()
                    })
                    .flatten()
                    .filter(|ob| ob.is_hq_valid())
                    .collect();
                let chance = self.chance_from_loot_matrix_index(lmi).ok()?;

                Some(LootMatrixObjectChances {
                    lmi,
                    chance,
                    sources: (sources.len() > 0).then(|| sources)?,
                })
            })
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientDestructibleComponentId(pub i32);
impl Api for CdClientDestructibleComponentId {}
impl ComponentId for CdClientDestructibleComponentId {
    const ID: i32 = DESTROYABLE_COMPONENT;
    const NAME: &'static str = "Destructible";
    fn id(&self) -> i32 {
        self.0
    }
}
impl CdClientDestructibleComponentId {
    pub fn fetch(&self) -> MsgResult<CdClientDestructibleComponent> {
        self.cdclient()
            .destructible_component
            .at_key(&self.0)
            .cloned()
            .ok_or_else(|| self.err("does not exist"))
    }

    pub fn items_dropped(&self) -> Option<Vec<LootTableChances>> {
        let dc = self.fetch().ok()?;
        let lmi = CdClientLootMatrixId(dc.loot_matrix_index?);
        lmi.loot_chances()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientPackageComponentId(pub i32);
impl Api for CdClientPackageComponentId {}
impl ComponentId for CdClientPackageComponentId {
    const ID: i32 = PACKAGE_COMPONENT;
    const NAME: &'static str = "Package";
    fn id(&self) -> i32 {
        self.0
    }
}
impl CdClientPackageComponentId {
    pub fn fetch(&self) -> MsgResult<CdClientPackageComponent> {
        self.cdclient()
            .package_component
            .at_key(&self.0)
            .cloned()
            .ok_or_else(|| self.err("does not exist"))
    }

    pub fn items_dropped(&self) -> Option<Vec<LootTableChances>> {
        let pc = self.fetch().ok()?;
        let lmi = CdClientLootMatrixId(pc.loot_matrix_index);
        lmi.loot_chances()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientItemComponentId(pub i32);
impl Api for CdClientItemComponentId {}
impl ComponentId for CdClientItemComponentId {
    const ID: i32 = ITEM_COMPONENT;
    const NAME: &'static str = "Item";
    fn id(&self) -> i32 {
        self.0
    }
}
impl CdClientItemComponentId {
    pub fn fetch(&self) -> MsgResult<CdClientItemComponent> {
        self.cdclient()
            .item_component
            .at_key(&self.0)
            .cloned()
            .ok_or_else(|| self.err("does not exist"))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientRenderComponentId(pub i32);
impl Api for CdClientRenderComponentId {}
impl ComponentId for CdClientRenderComponentId {
    const ID: i32 = ITEM_COMPONENT;
    const NAME: &'static str = "Render";
    fn id(&self) -> i32 {
        self.0
    }
}
impl CdClientRenderComponentId {
    pub fn fetch(&self) -> MsgResult<CdClientRenderComponent> {
        self.cdclient()
            .render_component
            .at_key(&self.0)
            .cloned()
            .ok_or_else(|| self.err("does not exist"))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientVendorComponentId(pub i32);
impl Api for CdClientVendorComponentId {}
impl ComponentId for CdClientVendorComponentId {
    const ID: i32 = VENDOR_COMPONENT;
    const NAME: &'static str = "Vendor";
    fn id(&self) -> i32 {
        self.0
    }
}
impl CdClientVendorComponentId {
    pub fn fetch(&self) -> MsgResult<CdClientVendorComponent> {
        self.cdclient()
            .vendor_component
            .at_key(&self.0)
            .cloned()
            .ok_or_else(|| self.err("does not exist"))
    }

    pub fn items_sold(&self) -> Option<Vec<CdClientObjectsId>> {
        let vc = self.fetch().ok()?;
        let lmi = CdClientLootMatrixId(vc.loot_matrix_index);
        lmi.contained_items()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientActivityRewardsId(pub i32);
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
impl CdClientActivityRewardsId {
    pub fn fetch(&self) -> MsgResult<Vec<CdClientActivityRewards>> {
        self.cdclient()
            .activity_rewards
            .at_group_key(&self.0)
            .clone()
            .ok_or_else(|| self.err("does not exist"))
    }

    /// ! use a select menu with description to separate which one is being rewarded
    pub fn items_rewarded(&self) -> Option<Vec<LootTableChances>> {
        let activity_rewards = self.fetch().ok()?;
        activity_rewards
            .into_iter()
            .filter_map(|ar| {
                let lmi = CdClientLootMatrixId(ar.loot_matrix_index?);
                lmi.loot_chances()
            })
            .flatten()
            .collect_some()
        // let lmi = CdClientLootMatrixId(activity_rewards.loot_matrix_index?);
        // lmi.loot_chances()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CdClientSkillBehaviorId(pub i32);
impl Api for CdClientSkillBehaviorId {}
impl LUExplorer for CdClientSkillBehaviorId {
    const NAME: &'static str = "Skill";
    const ENDPOINT: &'static str = "skills";

    fn id(&self) -> i32 {
        self.0
    }

    fn name(&self) -> Option<String> {
        self.locale()
            .skill_behavior
            .get(&self.id())
            .and_then(|skill_behavior| skill_behavior.name.clone())
    }
}
impl CdClientSkillBehaviorId {
    pub fn fetch(&self) -> MsgResult<CdClientSkillBehavior> {
        self.cdclient()
            .skill_behavior
            .at_key(&self.0)
            .cloned()
            .ok_or_else(|| self.err("does not exist"))
    }

    pub fn thumbnail(&self) -> Option<String> {
        let skill = self.fetch().ok()?;
        let icon = self.cdclient().icons.at_key(&skill.skill_icon?).cloned()?;
        let asset = icon.icon_path?;
        Some(icon_asset_as_url(asset))
    }

    pub fn skill_items(&self) -> Option<Vec<CdClientObjectsId>> {
        self.cdclient()
            .object_skills
            .iter()
            // .filter_map(|ob| (ob.skill_id == self.0).then(|| CdClientObjectsId(ob.object_template)))
            .filter(|ob| ob.skill_id == self.0)
            .map(|ob| CdClientObjectsId(ob.object_template))
            .collect_some()
    }
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
// pub struct CdClientIconsId(pub i32);
// impl Api for CdClientIconsId {}
// impl LUExplorer for CdClientIconsId {
//     const NAME: &'static str = "Skill";
//     const ENDPOINT: &'static str = "skills";
//
//     fn id(&self) -> i32 {
//         self.0
//     }
//
//     fn name(&self) -> Option<String> {
//         self.locale()
//             .skill_behavior
//             .get(&self.id())
//             .and_then(|skill_behavior| skill_behavior.name.clone())
//     }
// }
// impl CdClientIconsId {
//     pub fn fetch(&self) -> MsgResult<CdClientIcons> {
//         self.cdclient()
//             .skill_behavior
//             .at_key(&self.0)
//             .cloned()
//             .ok_or_else(|| self.err("does not exist"))
//     }
//     pub fn thumbnail(&self, id: i32) -> Option<String> {
//         let skill = self.fetch().ok()?;
//         // self.get_icon_url(skill.skill_icon?)
//         Some(icon_asset_as_url(skill.skill_icon?))
//     }
// }
