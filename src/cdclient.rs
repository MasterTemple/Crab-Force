use std::{
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
};

// ! I STILL SHOULD TRIM NON-NULLABLE STRINGS

// ! SOME STRINGS ARE ACTUALLY ARRAYS

/// https://docs.lu-dev.net/en/latest/components.html
pub mod components {
    pub const RENDER_COMPONENT: i32 = 2;
    pub const DESTROYABLE_COMPONENT: i32 = 7;
    pub const SKILL_COMPONENT: i32 = 9;
    pub const ITEM_COMPONENT: i32 = 11;
    pub const VENDOR_COMPONENT: i32 = 16;
    pub const INVENTORY_COMPONENT: i32 = 17;
}

// static LIST_REGEX: Lazy<Regex> =
//     Lazy::new(|| Regex::new(r#"<translation locale="([^"]+)">(.*)</translation>"#).unwrap());

fn parse_optional_comma_list<T: FromStr>(input: Option<String>) -> Option<Vec<T>> {
    parse_optional_list(input, ',')
}

fn parse_optional_list<T: FromStr>(input: Option<String>, splitter: char) -> Option<Vec<T>> {
    Some(parse_required_list(input?, splitter))
}

fn parse_required_comma_list<T: FromStr>(input: String) -> Vec<T> {
    parse_required_list(input, ',')
}

fn parse_required_list<T: FromStr>(input: String, splitter: char) -> Vec<T> {
    let mut elements = vec![];
    for num in input.split(splitter) {
        if let Ok(el) = num.trim().parse() {
            elements.push(el);
        }
    }
    elements
}

/// Trims text and converts values that are completely white-space to be None
fn trim_and_nullify(value: Option<String>) -> Option<String> {
    let value = value?;
    let trimmed = value.trim();
    if trimmed.len() == 0 {
        return None;
    }
    Some(if trimmed.len() != value.len() {
        trimmed.to_string()
    } else {
        value
    })
}

fn trim_to_string(value: String) -> String {
    value.trim().to_string()
}

pub struct KeyedVec<T: HasKey>(Vec<T>);

impl<T: HasKey> KeyedVec<T> {
    pub fn new(mut v: Vec<T>) -> Self {
        v.sort_by_key(|it| it.clone_key());
        Self(v)
    }
}

impl<T: HasKey> Deref for KeyedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: HasKey> DerefMut for KeyedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: HasKey> KeyedVec<T> {
    pub fn at_key(&self, key: &T::Key) -> Option<&T> {
        let idx = self.binary_search_by_key(key, |it| it.clone_key()).ok()?;
        self.0.get(idx)
    }
}

pub struct GroupKeyedVec<T: HasGroupKey>(Vec<T>);

impl<T: HasGroupKey> GroupKeyedVec<T> {
    pub fn new(mut v: Vec<T>) -> Self {
        v.sort_by_key(|it| it.clone_key());
        Self(v)
    }
}

impl<T: HasGroupKey> Deref for GroupKeyedVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: HasGroupKey> DerefMut for GroupKeyedVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: HasGroupKey> GroupKeyedVec<T> {
    pub fn at_group_key(&self, key: &T::Key) -> Option<&[T]> {
        let idx = self.binary_search_by_key(key, |it| it.clone_key()).ok()?;

        let mut min_idx = idx;
        while min_idx > 0 {
            if self.0[min_idx - 1].get_group_key() == key {
                min_idx -= 1;
            } else {
                break;
            }
        }

        let mut max_idx = idx;
        let max_len = self.0.len() - 1;
        while max_idx < max_len {
            if self.0[max_idx + 1].get_group_key() == key {
                max_idx += 1;
            } else {
                break;
            }
        }

        Some(&self.0[min_idx..=max_idx])
    }
}

/**!
* - Most things are grouped into id to value (Map<Key, Value) or into id to associated values (Map<Key, Vec<Value>)
* - I am storing all rows in [`CdClient::rows`] so that groupings don't remove anything that would be necessary
* - I am using Arc<Value> so that I don't need to clone the data and it is the easiest way to implement self-referential structs
*/

pub trait HasKey: Clone {
    type Key: Ord + Clone + std::fmt::Debug;
    fn get_key(&self) -> &Self::Key;
    fn clone_key(&self) -> Self::Key {
        self.get_key().clone()
    }
}

/**
* - For when there are multiple entries for a given key
* - While I could have a child key, I think it is best if I just have a list of elements available
* at that key
*/
pub trait HasGroupKey: Clone {
    type Key: Ord + Clone + std::fmt::Debug;
    fn get_group_key(&self) -> &Self::Key;
    fn clone_key(&self) -> Self::Key {
        self.get_group_key().clone()
    }
}

// All tables impl this
pub trait FromCdClient: Sized {
    const TABLE: &'static str;
    fn load(conn: &rusqlite::Connection) -> rusqlite::Result<Vec<Self>> {
        let mut stmt = conn.prepare(format!("SELECT * FROM {}", Self::TABLE).as_str())?;
        let result = stmt.query_map([], Self::query_map)?.into_iter().collect();
        result
    }
    // map row elements to corresponding fields
    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self>;
}

#[allow(dead_code)]
pub struct CdClientRows {
    pub ai_combat_roles: Vec<AiCombatRoles>,
    pub accessory_default_loc: Vec<AccessoryDefaultLoc>,
    pub activities: Vec<Activities>,
    pub activity_rewards: Vec<ActivityRewards>,
    pub activity_text: Vec<ActivityText>,
    pub animation_index: Vec<AnimationIndex>,
    pub animations: Vec<Animations>,
    pub base_combat_aicomponent: Vec<BaseCombatAIComponent>,
    pub behavior_effect: Vec<BehaviorEffect>,
    pub behavior_parameter: Vec<BehaviorParameter>,
    pub behavior_template: Vec<BehaviorTemplate>,
    pub behavior_template_name: Vec<BehaviorTemplateName>,
    pub blueprints: Vec<Blueprints>,
    pub brick_colors: Vec<BrickColors>,
    pub brick_idtable: Vec<BrickIDTable>,
    pub buff_definitions: Vec<BuffDefinitions>,
    pub buff_parameters: Vec<BuffParameters>,
    pub camera: Vec<Camera>,
    pub celebration_parameters: Vec<CelebrationParameters>,
    pub choice_build_component: Vec<ChoiceBuildComponent>,
    pub collectible_component: Vec<CollectibleComponent>,
    pub components_registry: Vec<ComponentsRegistry>,
    pub control_schemes: Vec<ControlSchemes>,
    pub currency_denominations: Vec<CurrencyDenominations>,
    pub currency_table: Vec<CurrencyTable>,
    pub db_exclude: Vec<DbExclude>,
    pub deletion_restrictions: Vec<DeletionRestrictions>,
    pub destructible_component: Vec<DestructibleComponent>,
    pub dev_model_behaviors: Vec<DevModelBehaviors>,
    pub emotes: Vec<Emotes>,
    pub event_gating: Vec<EventGating>,
    pub exhibit_component: Vec<ExhibitComponent>,
    pub factions: Vec<Factions>,
    pub feature_gating: Vec<FeatureGating>,
    pub flair_table: Vec<FlairTable>,
    pub icons: Vec<Icons>,
    pub inventory_component: Vec<InventoryComponent>,
    pub item_component: Vec<ItemComponent>,
    pub item_egg_data: Vec<ItemEggData>,
    pub item_food_data: Vec<ItemFoodData>,
    pub item_set_skills: Vec<ItemSetSkills>,
    pub item_sets: Vec<ItemSets>,
    pub jet_pack_pad_component: Vec<JetPackPadComponent>,
    pub lu_pexhibit_component: Vec<LuPExhibitComponent>,
    pub lu_pexhibit_model_data: Vec<LuPExhibitModelData>,
    pub lu_pzone_ids: Vec<LuPZoneIDs>,
    pub language_type: Vec<LanguageType>,
    pub level_progression_lookup: Vec<LevelProgressionLookup>,
    pub loot_matrix: Vec<LootMatrix>,
    pub loot_matrix_index: Vec<LootMatrixIndex>,
    pub loot_table: Vec<LootTable>,
    pub loot_table_index: Vec<LootTableIndex>,
    pub minifig_component: Vec<MinifigComponent>,
    pub minifig_decals_eyebrows: Vec<MinifigDecalsEyebrows>,
    pub minifig_decals_eyes: Vec<MinifigDecalsEyes>,
    pub minifig_decals_legs: Vec<MinifigDecalsLegs>,
    pub minifig_decals_mouths: Vec<MinifigDecalsMouths>,
    pub minifig_decals_torsos: Vec<MinifigDecalsTorsos>,
    pub mission_email: Vec<MissionEmail>,
    pub mission_npccomponent: Vec<MissionNPCComponent>,
    pub mission_tasks: Vec<MissionTasks>,
    pub mission_text: Vec<MissionText>,
    pub missions: Vec<Missions>,
    pub model_behavior: Vec<ModelBehavior>,
    pub modular_build_component: Vec<ModularBuildComponent>,
    pub module_component: Vec<ModuleComponent>,
    pub motion_fx: Vec<MotionFX>,
    pub movement_aicomponent: Vec<MovementAIComponent>,
    pub moving_platforms: Vec<MovingPlatforms>,
    pub npc_icons: Vec<NpcIcons>,
    pub object_behavior_xref: Vec<ObjectBehaviorXREF>,
    pub object_behaviors: Vec<ObjectBehaviors>,
    pub object_skills: Vec<ObjectSkills>,
    pub objects: Vec<Objects>,
    pub package_component: Vec<PackageComponent>,
    pub pet_abilities: Vec<PetAbilities>,
    pub pet_component: Vec<PetComponent>,
    pub pet_nest_component: Vec<PetNestComponent>,
    pub physics_component: Vec<PhysicsComponent>,
    pub player_flags: Vec<PlayerFlags>,
    pub player_statistics: Vec<PlayerStatistics>,
    pub preconditions: Vec<Preconditions>,
    pub property_entrance_component: Vec<PropertyEntranceComponent>,
    pub property_template: Vec<PropertyTemplate>,
    pub proximity_monitor_component: Vec<ProximityMonitorComponent>,
    pub proximity_types: Vec<ProximityTypes>,
    pub racing_module_component: Vec<RacingModuleComponent>,
    pub rail_activator_component: Vec<RailActivatorComponent>,
    pub rarity_table: Vec<RarityTable>,
    pub rarity_table_index: Vec<RarityTableIndex>,
    pub rebuild_component: Vec<RebuildComponent>,
    pub rebuild_sections: Vec<RebuildSections>,
    pub release_version: Vec<ReleaseVersion>,
    pub render_component: Vec<RenderComponent>,
    pub render_component_flash: Vec<RenderComponentFlash>,
    pub render_component_wrapper: Vec<RenderComponentWrapper>,
    pub render_icon_assets: Vec<RenderIconAssets>,
    pub reputation_rewards: Vec<ReputationRewards>,
    pub reward_codes: Vec<RewardCodes>,
    pub rewards: Vec<Rewards>,
    pub rocket_launchpad_control_component: Vec<RocketLaunchpadControlComponent>,
    pub scene_table: Vec<SceneTable>,
    pub script_component: Vec<ScriptComponent>,
    pub skill_behavior: Vec<SkillBehavior>,
    pub smashable_chain: Vec<SmashableChain>,
    pub smashable_chain_index: Vec<SmashableChainIndex>,
    pub smashable_component: Vec<SmashableComponent>,
    pub smashable_elements: Vec<SmashableElements>,
    pub speedchat_menu: Vec<SpeedchatMenu>,
    pub subscription_pricing: Vec<SubscriptionPricing>,
    pub surface_type: Vec<SurfaceType>,
    pub taming_build_puzzles: Vec<TamingBuildPuzzles>,
    pub text_description: Vec<TextDescription>,
    pub text_language: Vec<TextLanguage>,
    pub trail_effects: Vec<TrailEffects>,
    pub ug_behavior_sounds: Vec<UgBehaviorSounds>,
    pub vehicle_physics: Vec<VehiclePhysics>,
    pub vehicle_stat_map: Vec<VehicleStatMap>,
    pub vendor_component: Vec<VendorComponent>,
    pub whats_cool_item_spotlight: Vec<WhatsCoolItemSpotlight>,
    pub whats_cool_news_and_tips: Vec<WhatsCoolNewsAndTips>,
    pub world_config: Vec<WorldConfig>,
    pub zone_loading_tips: Vec<ZoneLoadingTips>,
    pub zone_summary: Vec<ZoneSummary>,
    pub zone_table: Vec<ZoneTable>,
    pub brick_attributes: Vec<BrickAttributes>,
    pub dtproperties: Vec<Dtproperties>,
    pub map_animation_priorities: Vec<MapAnimationPriorities>,
    pub map_asset_type: Vec<MapAssetType>,
    pub map_icon: Vec<MapIcon>,
    pub map_item_types: Vec<MapItemTypes>,
    pub map_render_effects: Vec<MapRenderEffects>,
    pub map_shaders: Vec<MapShaders>,
    pub map_texture_resource: Vec<MapTextureResource>,
    pub map_blueprint_category: Vec<MapBlueprintCategory>,
    pub sysdiagrams: Vec<Sysdiagrams>,
    pub possessable_component: Vec<PossessableComponent>,
    pub skill_sets: Vec<SkillSets>,
    pub map_faces_and_hair: Vec<MapFacesAndHair>,
}

impl CdClientRows {
    pub fn load_sqlite(path: &Path) -> rusqlite::Result<Self> {
        let conn = rusqlite::Connection::open(path)?;
        Ok(Self {
            ai_combat_roles: AiCombatRoles::load(&conn)?,
            accessory_default_loc: AccessoryDefaultLoc::load(&conn)?,
            activities: Activities::load(&conn)?,
            activity_rewards: ActivityRewards::load(&conn)?,
            activity_text: ActivityText::load(&conn)?,
            animation_index: AnimationIndex::load(&conn)?,
            animations: Animations::load(&conn)?,
            base_combat_aicomponent: BaseCombatAIComponent::load(&conn)?,
            behavior_effect: BehaviorEffect::load(&conn)?,
            behavior_parameter: BehaviorParameter::load(&conn)?,
            behavior_template: BehaviorTemplate::load(&conn)?,
            behavior_template_name: BehaviorTemplateName::load(&conn)?,
            blueprints: Blueprints::load(&conn)?,
            brick_colors: BrickColors::load(&conn)?,
            brick_idtable: BrickIDTable::load(&conn)?,
            buff_definitions: BuffDefinitions::load(&conn)?,
            buff_parameters: BuffParameters::load(&conn)?,
            camera: Camera::load(&conn)?,
            celebration_parameters: CelebrationParameters::load(&conn)?,
            choice_build_component: ChoiceBuildComponent::load(&conn)?,
            collectible_component: CollectibleComponent::load(&conn)?,
            components_registry: ComponentsRegistry::load(&conn)?,
            control_schemes: ControlSchemes::load(&conn)?,
            currency_denominations: CurrencyDenominations::load(&conn)?,
            currency_table: CurrencyTable::load(&conn)?,
            db_exclude: DbExclude::load(&conn)?,
            deletion_restrictions: DeletionRestrictions::load(&conn)?,
            destructible_component: DestructibleComponent::load(&conn)?,
            dev_model_behaviors: DevModelBehaviors::load(&conn)?,
            emotes: Emotes::load(&conn)?,
            event_gating: EventGating::load(&conn)?,
            exhibit_component: ExhibitComponent::load(&conn)?,
            factions: Factions::load(&conn)?,
            feature_gating: FeatureGating::load(&conn)?,
            flair_table: FlairTable::load(&conn)?,
            icons: Icons::load(&conn)?,
            inventory_component: InventoryComponent::load(&conn)?,
            item_component: ItemComponent::load(&conn)?,
            item_egg_data: ItemEggData::load(&conn)?,
            item_food_data: ItemFoodData::load(&conn)?,
            item_set_skills: ItemSetSkills::load(&conn)?,
            item_sets: ItemSets::load(&conn)?,
            jet_pack_pad_component: JetPackPadComponent::load(&conn)?,
            lu_pexhibit_component: LuPExhibitComponent::load(&conn)?,
            lu_pexhibit_model_data: LuPExhibitModelData::load(&conn)?,
            lu_pzone_ids: LuPZoneIDs::load(&conn)?,
            language_type: LanguageType::load(&conn)?,
            level_progression_lookup: LevelProgressionLookup::load(&conn)?,
            loot_matrix: LootMatrix::load(&conn)?,
            loot_matrix_index: LootMatrixIndex::load(&conn)?,
            loot_table: LootTable::load(&conn)?,
            loot_table_index: LootTableIndex::load(&conn)?,
            minifig_component: MinifigComponent::load(&conn)?,
            minifig_decals_eyebrows: MinifigDecalsEyebrows::load(&conn)?,
            minifig_decals_eyes: MinifigDecalsEyes::load(&conn)?,
            minifig_decals_legs: MinifigDecalsLegs::load(&conn)?,
            minifig_decals_mouths: MinifigDecalsMouths::load(&conn)?,
            minifig_decals_torsos: MinifigDecalsTorsos::load(&conn)?,
            mission_email: MissionEmail::load(&conn)?,
            mission_npccomponent: MissionNPCComponent::load(&conn)?,
            mission_tasks: MissionTasks::load(&conn)?,
            mission_text: MissionText::load(&conn)?,
            missions: Missions::load(&conn)?,
            model_behavior: ModelBehavior::load(&conn)?,
            modular_build_component: ModularBuildComponent::load(&conn)?,
            module_component: ModuleComponent::load(&conn)?,
            motion_fx: MotionFX::load(&conn)?,
            movement_aicomponent: MovementAIComponent::load(&conn)?,
            moving_platforms: MovingPlatforms::load(&conn)?,
            npc_icons: NpcIcons::load(&conn)?,
            object_behavior_xref: ObjectBehaviorXREF::load(&conn)?,
            object_behaviors: ObjectBehaviors::load(&conn)?,
            object_skills: ObjectSkills::load(&conn)?,
            objects: Objects::load(&conn)?,
            package_component: PackageComponent::load(&conn)?,
            pet_abilities: PetAbilities::load(&conn)?,
            pet_component: PetComponent::load(&conn)?,
            pet_nest_component: PetNestComponent::load(&conn)?,
            physics_component: PhysicsComponent::load(&conn)?,
            player_flags: PlayerFlags::load(&conn)?,
            player_statistics: PlayerStatistics::load(&conn)?,
            preconditions: Preconditions::load(&conn)?,
            property_entrance_component: PropertyEntranceComponent::load(&conn)?,
            property_template: PropertyTemplate::load(&conn)?,
            proximity_monitor_component: ProximityMonitorComponent::load(&conn)?,
            proximity_types: ProximityTypes::load(&conn)?,
            racing_module_component: RacingModuleComponent::load(&conn)?,
            rail_activator_component: RailActivatorComponent::load(&conn)?,
            rarity_table: RarityTable::load(&conn)?,
            rarity_table_index: RarityTableIndex::load(&conn)?,
            rebuild_component: RebuildComponent::load(&conn)?,
            rebuild_sections: RebuildSections::load(&conn)?,
            release_version: ReleaseVersion::load(&conn)?,
            render_component: RenderComponent::load(&conn)?,
            render_component_flash: RenderComponentFlash::load(&conn)?,
            render_component_wrapper: RenderComponentWrapper::load(&conn)?,
            render_icon_assets: RenderIconAssets::load(&conn)?,
            reputation_rewards: ReputationRewards::load(&conn)?,
            reward_codes: RewardCodes::load(&conn)?,
            rewards: Rewards::load(&conn)?,
            rocket_launchpad_control_component: RocketLaunchpadControlComponent::load(&conn)?,
            scene_table: SceneTable::load(&conn)?,
            script_component: ScriptComponent::load(&conn)?,
            skill_behavior: SkillBehavior::load(&conn)?,
            smashable_chain: SmashableChain::load(&conn)?,
            smashable_chain_index: SmashableChainIndex::load(&conn)?,
            smashable_component: SmashableComponent::load(&conn)?,
            smashable_elements: SmashableElements::load(&conn)?,
            speedchat_menu: SpeedchatMenu::load(&conn)?,
            subscription_pricing: SubscriptionPricing::load(&conn)?,
            surface_type: SurfaceType::load(&conn)?,
            taming_build_puzzles: TamingBuildPuzzles::load(&conn)?,
            text_description: TextDescription::load(&conn)?,
            text_language: TextLanguage::load(&conn)?,
            trail_effects: TrailEffects::load(&conn)?,
            ug_behavior_sounds: UgBehaviorSounds::load(&conn)?,
            vehicle_physics: VehiclePhysics::load(&conn)?,
            vehicle_stat_map: VehicleStatMap::load(&conn)?,
            vendor_component: VendorComponent::load(&conn)?,
            whats_cool_item_spotlight: WhatsCoolItemSpotlight::load(&conn)?,
            whats_cool_news_and_tips: WhatsCoolNewsAndTips::load(&conn)?,
            world_config: WorldConfig::load(&conn)?,
            zone_loading_tips: ZoneLoadingTips::load(&conn)?,
            zone_summary: ZoneSummary::load(&conn)?,
            zone_table: ZoneTable::load(&conn)?,
            brick_attributes: BrickAttributes::load(&conn)?,
            dtproperties: Dtproperties::load(&conn)?,
            map_animation_priorities: MapAnimationPriorities::load(&conn)?,
            map_asset_type: MapAssetType::load(&conn)?,
            map_icon: MapIcon::load(&conn)?,
            map_item_types: MapItemTypes::load(&conn)?,
            map_render_effects: MapRenderEffects::load(&conn)?,
            map_shaders: MapShaders::load(&conn)?,
            map_texture_resource: MapTextureResource::load(&conn)?,
            map_blueprint_category: MapBlueprintCategory::load(&conn)?,
            sysdiagrams: Sysdiagrams::load(&conn)?,
            possessable_component: PossessableComponent::load(&conn)?,
            skill_sets: SkillSets::load(&conn)?,
            map_faces_and_hair: MapFacesAndHair::load(&conn)?,
        })
    }
}

#[allow(dead_code)]
pub struct CdClient {
    pub ai_combat_roles: KeyedVec<AiCombatRoles>,
    pub accessory_default_loc: KeyedVec<AccessoryDefaultLoc>,
    pub activities: KeyedVec<Activities>,
    pub activity_rewards: GroupKeyedVec<ActivityRewards>,
    pub activity_text: GroupKeyedVec<ActivityText>,
    pub animation_index: KeyedVec<AnimationIndex>,
    pub animations: GroupKeyedVec<Animations>,
    pub base_combat_aicomponent: KeyedVec<BaseCombatAIComponent>,
    pub behavior_effect: GroupKeyedVec<BehaviorEffect>,
    pub behavior_parameter: GroupKeyedVec<BehaviorParameter>,
    pub behavior_template: GroupKeyedVec<BehaviorTemplate>,
    pub behavior_template_name: KeyedVec<BehaviorTemplateName>,
    pub blueprints: KeyedVec<Blueprints>,
    pub brick_colors: KeyedVec<BrickColors>,
    pub brick_idtable: KeyedVec<BrickIDTable>,
    pub buff_definitions: KeyedVec<BuffDefinitions>,
    pub buff_parameters: KeyedVec<BuffParameters>,
    pub camera: KeyedVec<Camera>,
    pub celebration_parameters: KeyedVec<CelebrationParameters>,
    pub choice_build_component: KeyedVec<ChoiceBuildComponent>,
    pub collectible_component: KeyedVec<CollectibleComponent>,
    pub components_registry: GroupKeyedVec<ComponentsRegistry>,
    pub control_schemes: KeyedVec<ControlSchemes>,
    pub currency_denominations: KeyedVec<CurrencyDenominations>,
    pub currency_table: KeyedVec<CurrencyTable>,
    pub db_exclude: KeyedVec<DbExclude>,
    pub deletion_restrictions: KeyedVec<DeletionRestrictions>,
    pub destructible_component: KeyedVec<DestructibleComponent>,
    pub dev_model_behaviors: KeyedVec<DevModelBehaviors>,
    pub emotes: KeyedVec<Emotes>,
    pub event_gating: KeyedVec<EventGating>,
    pub exhibit_component: KeyedVec<ExhibitComponent>,
    pub factions: KeyedVec<Factions>,
    pub feature_gating: KeyedVec<FeatureGating>,
    pub flair_table: KeyedVec<FlairTable>,
    pub icons: KeyedVec<Icons>,
    pub inventory_component: Vec<InventoryComponent>,
    pub item_component: KeyedVec<ItemComponent>,
    pub item_egg_data: KeyedVec<ItemEggData>,
    pub item_food_data: KeyedVec<ItemFoodData>,
    pub item_set_skills: GroupKeyedVec<ItemSetSkills>,
    pub item_sets: KeyedVec<ItemSets>,
    pub jet_pack_pad_component: KeyedVec<JetPackPadComponent>,
    pub lu_pexhibit_component: KeyedVec<LuPExhibitComponent>,
    pub lu_pexhibit_model_data: KeyedVec<LuPExhibitModelData>,
    pub lu_pzone_ids: KeyedVec<LuPZoneIDs>,
    pub language_type: KeyedVec<LanguageType>,
    pub level_progression_lookup: KeyedVec<LevelProgressionLookup>,
    pub loot_matrix: GroupKeyedVec<LootMatrix>,
    pub loot_matrix_index: KeyedVec<LootMatrixIndex>,
    pub loot_table: Vec<LootTable>,
    pub loot_table_index: KeyedVec<LootTableIndex>,
    pub minifig_component: KeyedVec<MinifigComponent>,
    pub minifig_decals_eyebrows: KeyedVec<MinifigDecalsEyebrows>,
    pub minifig_decals_eyes: KeyedVec<MinifigDecalsEyes>,
    pub minifig_decals_legs: KeyedVec<MinifigDecalsLegs>,
    pub minifig_decals_mouths: KeyedVec<MinifigDecalsMouths>,
    pub minifig_decals_torsos: KeyedVec<MinifigDecalsTorsos>,
    pub mission_email: KeyedVec<MissionEmail>,
    pub mission_npccomponent: Vec<MissionNPCComponent>,
    pub mission_tasks: Vec<MissionTasks>,
    pub mission_text: KeyedVec<MissionText>,
    pub missions: KeyedVec<Missions>,
    pub model_behavior: KeyedVec<ModelBehavior>,
    pub modular_build_component: KeyedVec<ModularBuildComponent>,
    pub module_component: KeyedVec<ModuleComponent>,
    pub motion_fx: KeyedVec<MotionFX>,
    pub movement_aicomponent: KeyedVec<MovementAIComponent>,
    pub moving_platforms: KeyedVec<MovingPlatforms>,
    pub npc_icons: KeyedVec<NpcIcons>,
    pub object_behavior_xref: KeyedVec<ObjectBehaviorXREF>,
    pub object_behaviors: KeyedVec<ObjectBehaviors>,
    pub object_skills: GroupKeyedVec<ObjectSkills>,
    pub objects: KeyedVec<Objects>,
    pub package_component: KeyedVec<PackageComponent>,
    pub pet_abilities: KeyedVec<PetAbilities>,
    pub pet_component: KeyedVec<PetComponent>,
    pub pet_nest_component: KeyedVec<PetNestComponent>,
    pub physics_component: KeyedVec<PhysicsComponent>,
    pub player_flags: KeyedVec<PlayerFlags>,
    pub player_statistics: KeyedVec<PlayerStatistics>,
    pub preconditions: KeyedVec<Preconditions>,
    pub property_entrance_component: KeyedVec<PropertyEntranceComponent>,
    pub property_template: KeyedVec<PropertyTemplate>,
    pub proximity_monitor_component: KeyedVec<ProximityMonitorComponent>,
    pub proximity_types: KeyedVec<ProximityTypes>,
    pub racing_module_component: KeyedVec<RacingModuleComponent>,
    pub rail_activator_component: KeyedVec<RailActivatorComponent>,
    pub rarity_table: KeyedVec<RarityTable>,
    pub rarity_table_index: KeyedVec<RarityTableIndex>,
    pub rebuild_component: KeyedVec<RebuildComponent>,
    pub rebuild_sections: KeyedVec<RebuildSections>,
    pub release_version: KeyedVec<ReleaseVersion>,
    pub render_component: KeyedVec<RenderComponent>,
    pub render_component_flash: GroupKeyedVec<RenderComponentFlash>,
    pub render_component_wrapper: KeyedVec<RenderComponentWrapper>,
    pub render_icon_assets: KeyedVec<RenderIconAssets>,
    pub reputation_rewards: KeyedVec<ReputationRewards>,
    pub reward_codes: KeyedVec<RewardCodes>,
    pub rewards: KeyedVec<Rewards>,
    pub rocket_launchpad_control_component: KeyedVec<RocketLaunchpadControlComponent>,
    pub scene_table: KeyedVec<SceneTable>,
    pub script_component: KeyedVec<ScriptComponent>,
    pub skill_behavior: KeyedVec<SkillBehavior>,
    pub smashable_chain: GroupKeyedVec<SmashableChain>,
    pub smashable_chain_index: KeyedVec<SmashableChainIndex>,
    pub smashable_component: KeyedVec<SmashableComponent>,
    pub smashable_elements: KeyedVec<SmashableElements>,
    pub speedchat_menu: KeyedVec<SpeedchatMenu>,
    pub subscription_pricing: KeyedVec<SubscriptionPricing>,
    pub surface_type: KeyedVec<SurfaceType>,
    pub taming_build_puzzles: KeyedVec<TamingBuildPuzzles>,
    pub text_description: KeyedVec<TextDescription>,
    pub text_language: KeyedVec<TextLanguage>,
    pub trail_effects: KeyedVec<TrailEffects>,
    pub ug_behavior_sounds: KeyedVec<UgBehaviorSounds>,
    pub vehicle_physics: KeyedVec<VehiclePhysics>,
    pub vehicle_stat_map: GroupKeyedVec<VehicleStatMap>,
    pub vendor_component: KeyedVec<VendorComponent>,
    pub whats_cool_item_spotlight: KeyedVec<WhatsCoolItemSpotlight>,
    pub whats_cool_news_and_tips: KeyedVec<WhatsCoolNewsAndTips>,
    pub world_config: KeyedVec<WorldConfig>,
    pub zone_loading_tips: KeyedVec<ZoneLoadingTips>,
    pub zone_summary: GroupKeyedVec<ZoneSummary>,
    pub zone_table: KeyedVec<ZoneTable>,
    pub brick_attributes: KeyedVec<BrickAttributes>,
    pub dtproperties: KeyedVec<Dtproperties>,
    pub map_animation_priorities: KeyedVec<MapAnimationPriorities>,
    pub map_asset_type: KeyedVec<MapAssetType>,
    pub map_icon: GroupKeyedVec<MapIcon>,
    pub map_item_types: KeyedVec<MapItemTypes>,
    pub map_render_effects: KeyedVec<MapRenderEffects>,
    pub map_shaders: KeyedVec<MapShaders>,
    pub map_texture_resource: KeyedVec<MapTextureResource>,
    pub map_blueprint_category: KeyedVec<MapBlueprintCategory>,
    pub sysdiagrams: KeyedVec<Sysdiagrams>,
    pub possessable_component: KeyedVec<PossessableComponent>,
    pub skill_sets: KeyedVec<SkillSets>,
    pub map_faces_and_hair: KeyedVec<MapFacesAndHair>,
}

impl CdClient {
    pub fn load_sqlite(path: &Path) -> rusqlite::Result<Self> {
        let cdclient = CdClientRows::load_sqlite(path)?;
        Ok(Self {
            ai_combat_roles: KeyedVec::new(cdclient.ai_combat_roles),
            accessory_default_loc: KeyedVec::new(cdclient.accessory_default_loc),
            activities: KeyedVec::new(cdclient.activities),
            activity_rewards: GroupKeyedVec::new(cdclient.activity_rewards),
            activity_text: GroupKeyedVec::new(cdclient.activity_text),
            animation_index: KeyedVec::new(cdclient.animation_index),
            animations: GroupKeyedVec::new(cdclient.animations),
            base_combat_aicomponent: KeyedVec::new(cdclient.base_combat_aicomponent),
            behavior_effect: GroupKeyedVec::new(cdclient.behavior_effect),
            behavior_parameter: GroupKeyedVec::new(cdclient.behavior_parameter),
            behavior_template: GroupKeyedVec::new(cdclient.behavior_template),
            behavior_template_name: KeyedVec::new(cdclient.behavior_template_name),
            blueprints: KeyedVec::new(cdclient.blueprints),
            brick_colors: KeyedVec::new(cdclient.brick_colors),
            brick_idtable: KeyedVec::new(cdclient.brick_idtable),
            buff_definitions: KeyedVec::new(cdclient.buff_definitions),
            buff_parameters: KeyedVec::new(cdclient.buff_parameters),
            camera: KeyedVec::new(cdclient.camera),
            celebration_parameters: KeyedVec::new(cdclient.celebration_parameters),
            choice_build_component: KeyedVec::new(cdclient.choice_build_component),
            collectible_component: KeyedVec::new(cdclient.collectible_component),
            components_registry: GroupKeyedVec::new(cdclient.components_registry),
            control_schemes: KeyedVec::new(cdclient.control_schemes),
            currency_denominations: KeyedVec::new(cdclient.currency_denominations),
            currency_table: KeyedVec::new(cdclient.currency_table),
            db_exclude: KeyedVec::new(cdclient.db_exclude),
            deletion_restrictions: KeyedVec::new(cdclient.deletion_restrictions),
            destructible_component: KeyedVec::new(cdclient.destructible_component),
            dev_model_behaviors: KeyedVec::new(cdclient.dev_model_behaviors),
            emotes: KeyedVec::new(cdclient.emotes),
            event_gating: KeyedVec::new(cdclient.event_gating),
            exhibit_component: KeyedVec::new(cdclient.exhibit_component),
            factions: KeyedVec::new(cdclient.factions),
            feature_gating: KeyedVec::new(cdclient.feature_gating),
            flair_table: KeyedVec::new(cdclient.flair_table),
            icons: KeyedVec::new(cdclient.icons),
            inventory_component: cdclient.inventory_component,
            item_component: KeyedVec::new(cdclient.item_component),
            item_egg_data: KeyedVec::new(cdclient.item_egg_data),
            item_food_data: KeyedVec::new(cdclient.item_food_data),
            item_set_skills: GroupKeyedVec::new(cdclient.item_set_skills),
            item_sets: KeyedVec::new(cdclient.item_sets),
            jet_pack_pad_component: KeyedVec::new(cdclient.jet_pack_pad_component),
            lu_pexhibit_component: KeyedVec::new(cdclient.lu_pexhibit_component),
            lu_pexhibit_model_data: KeyedVec::new(cdclient.lu_pexhibit_model_data),
            lu_pzone_ids: KeyedVec::new(cdclient.lu_pzone_ids),
            language_type: KeyedVec::new(cdclient.language_type),
            level_progression_lookup: KeyedVec::new(cdclient.level_progression_lookup),
            loot_matrix: GroupKeyedVec::new(cdclient.loot_matrix),
            loot_matrix_index: KeyedVec::new(cdclient.loot_matrix_index),
            loot_table: cdclient.loot_table,
            loot_table_index: KeyedVec::new(cdclient.loot_table_index),
            minifig_component: KeyedVec::new(cdclient.minifig_component),
            minifig_decals_eyebrows: KeyedVec::new(cdclient.minifig_decals_eyebrows),
            minifig_decals_eyes: KeyedVec::new(cdclient.minifig_decals_eyes),
            minifig_decals_legs: KeyedVec::new(cdclient.minifig_decals_legs),
            minifig_decals_mouths: KeyedVec::new(cdclient.minifig_decals_mouths),
            minifig_decals_torsos: KeyedVec::new(cdclient.minifig_decals_torsos),
            mission_email: KeyedVec::new(cdclient.mission_email),
            mission_npccomponent: cdclient.mission_npccomponent,
            mission_tasks: cdclient.mission_tasks,
            mission_text: KeyedVec::new(cdclient.mission_text),
            missions: KeyedVec::new(cdclient.missions),
            model_behavior: KeyedVec::new(cdclient.model_behavior),
            modular_build_component: KeyedVec::new(cdclient.modular_build_component),
            module_component: KeyedVec::new(cdclient.module_component),
            motion_fx: KeyedVec::new(cdclient.motion_fx),
            movement_aicomponent: KeyedVec::new(cdclient.movement_aicomponent),
            moving_platforms: KeyedVec::new(cdclient.moving_platforms),
            npc_icons: KeyedVec::new(cdclient.npc_icons),
            object_behavior_xref: KeyedVec::new(cdclient.object_behavior_xref),
            object_behaviors: KeyedVec::new(cdclient.object_behaviors),
            object_skills: GroupKeyedVec::new(cdclient.object_skills),
            objects: KeyedVec::new(cdclient.objects),
            package_component: KeyedVec::new(cdclient.package_component),
            pet_abilities: KeyedVec::new(cdclient.pet_abilities),
            pet_component: KeyedVec::new(cdclient.pet_component),
            pet_nest_component: KeyedVec::new(cdclient.pet_nest_component),
            physics_component: KeyedVec::new(cdclient.physics_component),
            player_flags: KeyedVec::new(cdclient.player_flags),
            player_statistics: KeyedVec::new(cdclient.player_statistics),
            preconditions: KeyedVec::new(cdclient.preconditions),
            property_entrance_component: KeyedVec::new(cdclient.property_entrance_component),
            property_template: KeyedVec::new(cdclient.property_template),
            proximity_monitor_component: KeyedVec::new(cdclient.proximity_monitor_component),
            proximity_types: KeyedVec::new(cdclient.proximity_types),
            racing_module_component: KeyedVec::new(cdclient.racing_module_component),
            rail_activator_component: KeyedVec::new(cdclient.rail_activator_component),
            rarity_table: KeyedVec::new(cdclient.rarity_table),
            rarity_table_index: KeyedVec::new(cdclient.rarity_table_index),
            rebuild_component: KeyedVec::new(cdclient.rebuild_component),
            rebuild_sections: KeyedVec::new(cdclient.rebuild_sections),
            release_version: KeyedVec::new(cdclient.release_version),
            render_component: KeyedVec::new(cdclient.render_component),
            render_component_flash: GroupKeyedVec::new(cdclient.render_component_flash),
            render_component_wrapper: KeyedVec::new(cdclient.render_component_wrapper),
            render_icon_assets: KeyedVec::new(cdclient.render_icon_assets),
            reputation_rewards: KeyedVec::new(cdclient.reputation_rewards),
            reward_codes: KeyedVec::new(cdclient.reward_codes),
            rewards: KeyedVec::new(cdclient.rewards),
            rocket_launchpad_control_component: KeyedVec::new(
                cdclient.rocket_launchpad_control_component,
            ),
            scene_table: KeyedVec::new(cdclient.scene_table),
            script_component: KeyedVec::new(cdclient.script_component),
            skill_behavior: KeyedVec::new(cdclient.skill_behavior),
            smashable_chain: GroupKeyedVec::new(cdclient.smashable_chain),
            smashable_chain_index: KeyedVec::new(cdclient.smashable_chain_index),
            smashable_component: KeyedVec::new(cdclient.smashable_component),
            smashable_elements: KeyedVec::new(cdclient.smashable_elements),
            speedchat_menu: KeyedVec::new(cdclient.speedchat_menu),
            subscription_pricing: KeyedVec::new(cdclient.subscription_pricing),
            surface_type: KeyedVec::new(cdclient.surface_type),
            taming_build_puzzles: KeyedVec::new(cdclient.taming_build_puzzles),
            text_description: KeyedVec::new(cdclient.text_description),
            text_language: KeyedVec::new(cdclient.text_language),
            trail_effects: KeyedVec::new(cdclient.trail_effects),
            ug_behavior_sounds: KeyedVec::new(cdclient.ug_behavior_sounds),
            vehicle_physics: KeyedVec::new(cdclient.vehicle_physics),
            vehicle_stat_map: GroupKeyedVec::new(cdclient.vehicle_stat_map),
            vendor_component: KeyedVec::new(cdclient.vendor_component),
            whats_cool_item_spotlight: KeyedVec::new(cdclient.whats_cool_item_spotlight),
            whats_cool_news_and_tips: KeyedVec::new(cdclient.whats_cool_news_and_tips),
            world_config: KeyedVec::new(cdclient.world_config),
            zone_loading_tips: KeyedVec::new(cdclient.zone_loading_tips),
            zone_summary: GroupKeyedVec::new(cdclient.zone_summary),
            zone_table: KeyedVec::new(cdclient.zone_table),
            brick_attributes: KeyedVec::new(cdclient.brick_attributes),
            dtproperties: KeyedVec::new(cdclient.dtproperties),
            map_animation_priorities: KeyedVec::new(cdclient.map_animation_priorities),
            map_asset_type: KeyedVec::new(cdclient.map_asset_type),
            map_icon: GroupKeyedVec::new(cdclient.map_icon),
            map_item_types: KeyedVec::new(cdclient.map_item_types),
            map_render_effects: KeyedVec::new(cdclient.map_render_effects),
            map_shaders: KeyedVec::new(cdclient.map_shaders),
            map_texture_resource: KeyedVec::new(cdclient.map_texture_resource),
            map_blueprint_category: KeyedVec::new(cdclient.map_blueprint_category),
            sysdiagrams: KeyedVec::new(cdclient.sysdiagrams),
            possessable_component: KeyedVec::new(cdclient.possessable_component),
            skill_sets: KeyedVec::new(cdclient.skill_sets),
            map_faces_and_hair: KeyedVec::new(cdclient.map_faces_and_hair),
        })
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AiCombatRoles {
    pub id: i32,
    pub preferred_role: i32,
    pub specified_min_range_nouse: Option<f64>,
    pub specified_max_range_nouse: Option<f64>,
    pub specific_min_range: Option<f64>,
    pub specific_max_range: Option<f64>,
}

impl FromCdClient for AiCombatRoles {
    const TABLE: &'static str = "AICombatRoles";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            preferred_role: row.get(1)?,
            specified_min_range_nouse: row.get(2)?,
            specified_max_range_nouse: row.get(3)?,
            specific_min_range: row.get(4)?,
            specific_max_range: row.get(5)?,
        })
    }
}

impl HasKey for AiCombatRoles {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AccessoryDefaultLoc {
    pub group_id: i32,
    pub description: String,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub rot_x: f64,
    pub rot_y: f64,
    pub rot_z: f64,
}

impl FromCdClient for AccessoryDefaultLoc {
    const TABLE: &'static str = "AccessoryDefaultLoc";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            group_id: row.get(0)?,
            description: trim_to_string(row.get(1)?),
            pos_x: row.get(2)?,
            pos_y: row.get(3)?,
            pos_z: row.get(4)?,
            rot_x: row.get(5)?,
            rot_y: row.get(6)?,
            rot_z: row.get(7)?,
        })
    }
}

impl HasKey for AccessoryDefaultLoc {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.group_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Activities {
    pub activity_id: i32,
    pub loc_status: i32,
    pub instance_map_id: Option<i32>,
    pub min_teams: Option<i32>,
    pub max_teams: Option<i32>,
    pub min_team_size: Option<i32>,
    pub max_team_size: Option<i32>,
    pub wait_time: Option<i32>,
    pub start_delay: Option<i32>,
    pub requires_unique_data: bool,
    pub leaderboard_type: Option<i32>,
    pub localize: bool,
    pub optional_cost_lot: Option<i32>,
    pub optional_cost_count: Option<i32>,
    pub show_uirewards: bool,
    pub community_activity_flag_id: Option<i32>,
    pub gate_version: Option<String>,
    pub no_team_loot_on_death: Option<bool>,
    pub optional_percentage: Option<f64>,
}

impl FromCdClient for Activities {
    const TABLE: &'static str = "Activities";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            activity_id: row.get(0)?,
            loc_status: row.get(1)?,
            instance_map_id: row.get(2)?,
            min_teams: row.get(3)?,
            max_teams: row.get(4)?,
            min_team_size: row.get(5)?,
            max_team_size: row.get(6)?,
            wait_time: row.get(7)?,
            start_delay: row.get(8)?,
            requires_unique_data: row.get(9)?,
            leaderboard_type: row.get(10)?,
            localize: row.get(11)?,
            optional_cost_lot: row.get(12)?,
            optional_cost_count: row.get(13)?,
            show_uirewards: row.get(14)?,
            community_activity_flag_id: row.get(15)?,
            gate_version: trim_and_nullify(row.get(16)?),
            no_team_loot_on_death: row.get(17)?,
            optional_percentage: row.get(18)?,
        })
    }
}

impl HasKey for Activities {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.activity_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ActivityRewards {
    pub object_template: i32,
    pub activity_reward_index: i32,
    pub activity_rating: i32,
    pub loot_matrix_index: Option<i32>,
    pub currency_index: Option<i32>,
    pub challenge_rating: i32,
    pub description: String,
}

impl FromCdClient for ActivityRewards {
    const TABLE: &'static str = "ActivityRewards";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            object_template: row.get(0)?,
            activity_reward_index: row.get(1)?,
            activity_rating: row.get(2)?,
            loot_matrix_index: row.get(3)?,
            currency_index: row.get(4)?,
            challenge_rating: row.get(5)?,
            description: trim_to_string(row.get(6)?),
        })
    }
}

impl HasGroupKey for ActivityRewards {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.object_template
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ActivityText {
    pub activity_id: i32,
    pub r#type: String,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for ActivityText {
    const TABLE: &'static str = "ActivityText";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            activity_id: row.get(0)?,
            r#type: trim_to_string(row.get(1)?),
            localize: row.get(2)?,
            loc_status: row.get(3)?,
            gate_version: trim_and_nullify(row.get(4)?),
        })
    }
}

impl HasGroupKey for ActivityText {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.activity_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AnimationIndex {
    pub animation_group_id: i32,
    pub description: String,
    pub group_type: Option<String>,
}

impl FromCdClient for AnimationIndex {
    const TABLE: &'static str = "AnimationIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            animation_group_id: row.get(0)?,
            description: trim_to_string(row.get(1)?),
            group_type: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for AnimationIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.animation_group_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Animations {
    pub animation_group_id: i32,
    pub animation_type: String,
    pub animation_name: String,
    pub chance_to_play: f64,
    pub min_loops: i32,
    pub max_loops: i32,
    pub animation_length: f64,
    pub hide_equip: bool,
    pub ignore_upper_body: bool,
    pub restartable: bool,
    pub face_animation_name: Option<Vec<i32>>,
    pub priority: Option<f64>,
    pub blend_time: Option<f64>,
}

impl FromCdClient for Animations {
    const TABLE: &'static str = "Animations";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            animation_group_id: row.get(0)?,
            animation_type: trim_to_string(row.get(1)?),
            animation_name: trim_to_string(row.get(2)?),
            chance_to_play: row.get(3)?,
            min_loops: row.get(4)?,
            max_loops: row.get(5)?,
            animation_length: row.get(6)?,
            hide_equip: row.get(7)?,
            ignore_upper_body: row.get(8)?,
            restartable: row.get(9)?,
            face_animation_name: parse_optional_comma_list(row.get(10)?),
            priority: row.get(11)?,
            blend_time: row.get(12)?,
        })
    }
}

impl HasGroupKey for Animations {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.animation_group_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BaseCombatAIComponent {
    pub id: i32,
    pub behavior_type: i32,
    pub combat_round_length: f64,
    pub combat_role: i32,
    pub min_round_length: f64,
    pub max_round_length: f64,
    pub tether_speed: f64,
    pub pursuit_speed: f64,
    pub combat_start_delay: Option<f64>,
    pub soft_tether_radius: f64,
    pub hard_tether_radius: f64,
    pub spawn_timer: Option<f64>,
    pub tether_effect_id: Option<i32>,
    pub ignore_mediator: bool,
    pub aggro_radius: Option<f64>,
    pub ignore_stat_reset: bool,
    pub ignore_parent: bool,
}

impl FromCdClient for BaseCombatAIComponent {
    const TABLE: &'static str = "BaseCombatAIComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            behavior_type: row.get(1)?,
            combat_round_length: row.get(2)?,
            combat_role: row.get(3)?,
            min_round_length: row.get(4)?,
            max_round_length: row.get(5)?,
            tether_speed: row.get(6)?,
            pursuit_speed: row.get(7)?,
            combat_start_delay: row.get(8)?,
            soft_tether_radius: row.get(9)?,
            hard_tether_radius: row.get(10)?,
            spawn_timer: row.get(11)?,
            tether_effect_id: row.get(12)?,
            ignore_mediator: row.get(13)?,
            aggro_radius: row.get(14)?,
            ignore_stat_reset: row.get(15)?,
            ignore_parent: row.get(16)?,
        })
    }
}

impl HasKey for BaseCombatAIComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BehaviorEffect {
    pub effect_id: i32,
    pub effect_type: Option<String>,
    pub effect_name: Option<String>,
    pub trail_id: Option<i32>,
    pub pcreate_duration: Option<f64>,
    pub animation_name: Option<String>,
    pub attach_to_object: Option<bool>,
    pub bone_name: Option<Vec<String>>,
    pub use_secondary: Option<bool>,
    pub camera_effect_type: Option<i32>,
    pub camera_duration: Option<f64>,
    pub camera_frequency: Option<f64>,
    pub camera_xamp: Option<f64>,
    pub camera_yamp: Option<f64>,
    pub camera_zamp: Option<f64>,
    pub camera_rot_frequency: Option<f64>,
    pub camera_roll: Option<f64>,
    pub camera_pitch: Option<f64>,
    pub camera_yaw: Option<f64>,
    pub audio_event_guid: Option<String>,
    pub render_effect_type: Option<i32>,
    pub render_effect_time: Option<f64>,
    pub render_start_val: Option<f64>,
    pub render_end_val: Option<f64>,
    pub render_delay_val: Option<f64>,
    pub render_value1: Option<f64>,
    pub render_value2: Option<f64>,
    pub render_value3: Option<f64>,
    pub render_rgba: Option<Vec<i32>>,
    pub render_shader_val: Option<i32>,
    pub motion_id: Option<i32>,
    pub mesh_id: Option<i32>,
    pub mesh_duration: Option<f64>,
    pub mesh_locked_node: Option<String>,
}

fn parse_bone_name(input: Option<String>) -> Option<Vec<String>> {
    Some(
        input?
            .split(&[',', ';'])
            .map(|it| it.trim().to_string())
            .collect(),
    )
}

impl FromCdClient for BehaviorEffect {
    const TABLE: &'static str = "BehaviorEffect";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            effect_id: row.get(0)?,
            effect_type: trim_and_nullify(row.get(1)?),
            effect_name: trim_and_nullify(row.get(2)?),
            trail_id: row.get(3)?,
            pcreate_duration: row.get(4)?,
            animation_name: trim_and_nullify(row.get(5)?),
            attach_to_object: row.get(6)?,
            bone_name: parse_bone_name(row.get(7)?),
            use_secondary: row.get(8)?,
            camera_effect_type: row.get(9)?,
            camera_duration: row.get(10)?,
            camera_frequency: row.get(11)?,
            camera_xamp: row.get(12)?,
            camera_yamp: row.get(13)?,
            camera_zamp: row.get(14)?,
            camera_rot_frequency: row.get(15)?,
            camera_roll: row.get(16)?,
            camera_pitch: row.get(17)?,
            camera_yaw: row.get(18)?,
            audio_event_guid: trim_and_nullify(row.get(19)?),
            render_effect_type: row.get(20)?,
            render_effect_time: row.get(21)?,
            render_start_val: row.get(22)?,
            render_end_val: row.get(23)?,
            render_delay_val: row.get(24)?,
            render_value1: row.get(25)?,
            render_value2: row.get(26)?,
            render_value3: row.get(27)?,
            render_rgba: parse_optional_comma_list(row.get(28)?),
            render_shader_val: row.get(29)?,
            motion_id: row.get(30)?,
            mesh_id: row.get(31)?,
            mesh_duration: row.get(32)?,
            mesh_locked_node: trim_and_nullify(row.get(33)?),
        })
    }
}

impl HasGroupKey for BehaviorEffect {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.effect_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BehaviorParameter {
    pub behavior_id: i32,
    pub parameter_id: String,
    pub value: f64,
}

impl FromCdClient for BehaviorParameter {
    const TABLE: &'static str = "BehaviorParameter";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            behavior_id: row.get(0)?,
            parameter_id: trim_to_string(row.get(1)?),
            value: row.get(2)?,
        })
    }
}

impl HasGroupKey for BehaviorParameter {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.behavior_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BehaviorTemplate {
    pub behavior_id: i32,
    pub template_id: i32,
    pub effect_id: i32,
    pub effect_handle: Option<String>,
}

impl FromCdClient for BehaviorTemplate {
    const TABLE: &'static str = "BehaviorTemplate";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            behavior_id: row.get(0)?,
            template_id: row.get(1)?,
            effect_id: row.get(2)?,
            effect_handle: trim_and_nullify(row.get(3)?),
        })
    }
}

impl HasGroupKey for BehaviorTemplate {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.behavior_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BehaviorTemplateName {
    pub template_id: i32,
    pub name: String,
}

impl FromCdClient for BehaviorTemplateName {
    const TABLE: &'static str = "BehaviorTemplateName";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            template_id: row.get(0)?,
            name: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for BehaviorTemplateName {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.template_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Blueprints {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub accountid: i64,
    pub characterid: i64,
    pub price: i32,
    pub rating: i32,
    pub categoryid: i32,
    pub lxfpath: String,
    pub deleted: bool,
    pub created: i64,
    pub modified: i64,
}

impl FromCdClient for Blueprints {
    const TABLE: &'static str = "Blueprints";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: trim_to_string(row.get(1)?),
            description: trim_to_string(row.get(2)?),
            accountid: row.get(3)?,
            characterid: row.get(4)?,
            price: row.get(5)?,
            rating: row.get(6)?,
            categoryid: row.get(7)?,
            lxfpath: trim_to_string(row.get(8)?),
            deleted: row.get(9)?,
            created: row.get(10)?,
            modified: row.get(11)?,
        })
    }
}

impl HasKey for Blueprints {
    type Key = i64;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BrickColors {
    pub id: i32,
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
    pub legopaletteid: i32,
    pub description: String,
    pub valid_types: i32,
    pub valid_characters: i32,
    pub factory_valid: bool,
}

impl FromCdClient for BrickColors {
    const TABLE: &'static str = "BrickColors";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            red: row.get(1)?,
            green: row.get(2)?,
            blue: row.get(3)?,
            alpha: row.get(4)?,
            legopaletteid: row.get(5)?,
            description: trim_to_string(row.get(6)?),
            valid_types: row.get(7)?,
            valid_characters: row.get(8)?,
            factory_valid: row.get(9)?,
        })
    }
}

impl HasKey for BrickColors {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BrickIDTable {
    pub ndobject_id: i32,
    pub legobrick_id: i32,
}

impl FromCdClient for BrickIDTable {
    const TABLE: &'static str = "BrickIDTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            ndobject_id: row.get(0)?,
            legobrick_id: row.get(1)?,
        })
    }
}

impl HasKey for BrickIDTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.ndobject_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BuffDefinitions {
    pub id: i32,
    pub priority: f64,
    pub uiicon: Option<String>,
}

impl FromCdClient for BuffDefinitions {
    const TABLE: &'static str = "BuffDefinitions";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            priority: row.get(1)?,
            uiicon: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for BuffDefinitions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BuffParameters {
    pub buff_id: i32,
    pub parameter_name: String,
    pub number_value: Option<f64>,
    pub string_value: Option<Vec<f64>>,
    pub effect_id: Option<i32>,
}

impl FromCdClient for BuffParameters {
    const TABLE: &'static str = "BuffParameters";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            buff_id: row.get(0)?,
            parameter_name: trim_to_string(row.get(1)?),
            number_value: row.get(2)?,
            string_value: parse_optional_comma_list(row.get(3)?),
            effect_id: row.get(4)?,
        })
    }
}

impl HasKey for BuffParameters {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.buff_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Camera {
    pub camera_name: String,
    pub pitch_angle_tolerance: f64,
    pub starting_zoom: f64,
    pub zoom_return_modifier: f64,
    pub pitch_return_modifier: f64,
    pub tether_out_return_modifier: f64,
    pub tether_in_return_multiplier: f64,
    pub verticle_movement_dampening_modifier: f64,
    pub return_from_incline_modifier: f64,
    pub horizontal_return_modifier: f64,
    pub yaw_behavior_speed_multiplier: f64,
    pub camera_collision_padding: f64,
    pub glide_speed: f64,
    pub fade_player_min_range: f64,
    pub min_movement_delta_tolerance: f64,
    pub min_glide_distance_tolerance: f64,
    pub look_forward_offset: f64,
    pub look_up_offset: f64,
    pub minimum_vertical_dampening_distance: f64,
    pub maximum_vertical_dampening_distance: f64,
    pub minimum_ignore_jump_distance: f64,
    pub maximum_ignore_jump_distance: f64,
    pub maximum_auto_glide_angle: f64,
    pub minimum_tether_glide_distance: f64,
    pub yaw_sign_correction: f64,
    pub set_1_look_forward_offset: Option<f64>,
    pub set_1_look_up_offset: Option<f64>,
    pub set_2_look_forward_offset: Option<f64>,
    pub set_2_look_up_offset: Option<f64>,
    pub set_0_speed_influence_on_dir: Option<f64>,
    pub set_1_speed_influence_on_dir: Option<f64>,
    pub set_2_speed_influence_on_dir: Option<f64>,
    pub set_0_angular_relaxation: Option<f64>,
    pub set_1_angular_relaxation: Option<f64>,
    pub set_2_angular_relaxation: Option<f64>,
    pub set_0_position_up_offset: Option<f64>,
    pub set_1_position_up_offset: Option<f64>,
    pub set_2_position_up_offset: Option<f64>,
    pub set_0_position_forward_offset: Option<f64>,
    pub set_1_position_forward_offset: Option<f64>,
    pub set_2_position_forward_offset: Option<f64>,
    pub set_0_fov: Option<f64>,
    pub set_1_fov: Option<f64>,
    pub set_2_fov: Option<f64>,
    pub set_0_max_yaw_angle: Option<f64>,
    pub set_1_max_yaw_angle: Option<f64>,
    pub set_2_max_yaw_angle: Option<f64>,
    pub set_1_fade_in_camera_set_change: Option<i32>,
    pub set_1_fade_out_camera_set_change: Option<i32>,
    pub set_2_fade_in_camera_set_change: Option<i32>,
    pub set_2_fade_out_camera_set_change: Option<i32>,
    pub input_movement_scalar: f64,
    pub input_rotation_scalar: f64,
    pub input_zoom_scalar: f64,
    pub minimum_pitch_desired: f64,
    pub maximum_pitch_desired: f64,
    pub minimum_zoom: f64,
    pub maximum_zoom: f64,
    pub horizontal_rotate_tolerance: f64,
    pub horizontal_rotate_modifier: f64,
}

impl FromCdClient for Camera {
    const TABLE: &'static str = "Camera";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            camera_name: trim_to_string(row.get(0)?),
            pitch_angle_tolerance: row.get(1)?,
            starting_zoom: row.get(2)?,
            zoom_return_modifier: row.get(3)?,
            pitch_return_modifier: row.get(4)?,
            tether_out_return_modifier: row.get(5)?,
            tether_in_return_multiplier: row.get(6)?,
            verticle_movement_dampening_modifier: row.get(7)?,
            return_from_incline_modifier: row.get(8)?,
            horizontal_return_modifier: row.get(9)?,
            yaw_behavior_speed_multiplier: row.get(10)?,
            camera_collision_padding: row.get(11)?,
            glide_speed: row.get(12)?,
            fade_player_min_range: row.get(13)?,
            min_movement_delta_tolerance: row.get(14)?,
            min_glide_distance_tolerance: row.get(15)?,
            look_forward_offset: row.get(16)?,
            look_up_offset: row.get(17)?,
            minimum_vertical_dampening_distance: row.get(18)?,
            maximum_vertical_dampening_distance: row.get(19)?,
            minimum_ignore_jump_distance: row.get(20)?,
            maximum_ignore_jump_distance: row.get(21)?,
            maximum_auto_glide_angle: row.get(22)?,
            minimum_tether_glide_distance: row.get(23)?,
            yaw_sign_correction: row.get(24)?,
            set_1_look_forward_offset: row.get(25)?,
            set_1_look_up_offset: row.get(26)?,
            set_2_look_forward_offset: row.get(27)?,
            set_2_look_up_offset: row.get(28)?,
            set_0_speed_influence_on_dir: row.get(29)?,
            set_1_speed_influence_on_dir: row.get(30)?,
            set_2_speed_influence_on_dir: row.get(31)?,
            set_0_angular_relaxation: row.get(32)?,
            set_1_angular_relaxation: row.get(33)?,
            set_2_angular_relaxation: row.get(34)?,
            set_0_position_up_offset: row.get(35)?,
            set_1_position_up_offset: row.get(36)?,
            set_2_position_up_offset: row.get(37)?,
            set_0_position_forward_offset: row.get(38)?,
            set_1_position_forward_offset: row.get(39)?,
            set_2_position_forward_offset: row.get(40)?,
            set_0_fov: row.get(41)?,
            set_1_fov: row.get(42)?,
            set_2_fov: row.get(43)?,
            set_0_max_yaw_angle: row.get(44)?,
            set_1_max_yaw_angle: row.get(45)?,
            set_2_max_yaw_angle: row.get(46)?,
            set_1_fade_in_camera_set_change: row.get(47)?,
            set_1_fade_out_camera_set_change: row.get(48)?,
            set_2_fade_in_camera_set_change: row.get(49)?,
            set_2_fade_out_camera_set_change: row.get(50)?,
            input_movement_scalar: row.get(51)?,
            input_rotation_scalar: row.get(52)?,
            input_zoom_scalar: row.get(53)?,
            minimum_pitch_desired: row.get(54)?,
            maximum_pitch_desired: row.get(55)?,
            minimum_zoom: row.get(56)?,
            maximum_zoom: row.get(57)?,
            horizontal_rotate_tolerance: row.get(58)?,
            horizontal_rotate_modifier: row.get(59)?,
        })
    }
}

impl HasKey for Camera {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.camera_name
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CelebrationParameters {
    pub id: i32,
    pub animation: String,
    pub background_object: i32,
    pub duration: f64,
    pub sub_text: Option<String>,
    pub main_text: Option<String>,
    pub icon_id: Option<i32>,
    pub cele_lead_in: f64,
    pub cele_lead_out: f64,
    pub camera_path_lot: i32,
    pub path_node_name: String,
    pub ambient_r: Option<f64>,
    pub ambient_g: Option<f64>,
    pub ambient_b: Option<f64>,
    pub directional_r: Option<f64>,
    pub directional_g: Option<f64>,
    pub directional_b: Option<f64>,
    pub specular_r: Option<f64>,
    pub specular_g: Option<f64>,
    pub specular_b: Option<f64>,
    pub light_position_x: Option<f64>,
    pub light_position_y: Option<f64>,
    pub light_position_z: Option<f64>,
    pub blend_time: Option<f64>,
    pub fog_color_r: Option<f64>,
    pub fog_color_g: Option<f64>,
    pub fog_color_b: Option<f64>,
    pub music_cue: Option<String>,
    pub sound_guid: Option<String>,
    pub mixer_program: Option<String>,
}

impl FromCdClient for CelebrationParameters {
    const TABLE: &'static str = "CelebrationParameters";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            animation: trim_to_string(row.get(1)?),
            background_object: row.get(2)?,
            duration: row.get(3)?,
            sub_text: trim_and_nullify(row.get(4)?),
            main_text: trim_and_nullify(row.get(5)?),
            icon_id: row.get(6)?,
            cele_lead_in: row.get(7)?,
            cele_lead_out: row.get(8)?,
            camera_path_lot: row.get(9)?,
            path_node_name: trim_to_string(row.get(10)?),
            ambient_r: row.get(11)?,
            ambient_g: row.get(12)?,
            ambient_b: row.get(13)?,
            directional_r: row.get(14)?,
            directional_g: row.get(15)?,
            directional_b: row.get(16)?,
            specular_r: row.get(17)?,
            specular_g: row.get(18)?,
            specular_b: row.get(19)?,
            light_position_x: row.get(20)?,
            light_position_y: row.get(21)?,
            light_position_z: row.get(22)?,
            blend_time: row.get(23)?,
            fog_color_r: row.get(24)?,
            fog_color_g: row.get(25)?,
            fog_color_b: row.get(26)?,
            music_cue: trim_and_nullify(row.get(27)?),
            sound_guid: trim_and_nullify(row.get(28)?),
            mixer_program: trim_and_nullify(row.get(29)?),
        })
    }
}

impl HasKey for CelebrationParameters {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ChoiceBuildComponent {
    pub id: i32,
    pub selections: Vec<i32>,
    pub imagination_override: Option<i32>,
}

impl FromCdClient for ChoiceBuildComponent {
    const TABLE: &'static str = "ChoiceBuildComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            selections: parse_required_comma_list(row.get(1)?),
            imagination_override: row.get(2)?,
        })
    }
}

impl HasKey for ChoiceBuildComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CollectibleComponent {
    pub id: i32,
    pub requirement_mission: Option<i32>,
}

impl FromCdClient for CollectibleComponent {
    const TABLE: &'static str = "CollectibleComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            requirement_mission: row.get(1)?,
        })
    }
}

impl HasKey for CollectibleComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ComponentsRegistry {
    pub id: i32,
    pub component_type: i32,
    pub component_id: i32,
}

impl FromCdClient for ComponentsRegistry {
    const TABLE: &'static str = "ComponentsRegistry";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            component_type: row.get(1)?,
            component_id: row.get(2)?,
        })
    }
}

impl HasGroupKey for ComponentsRegistry {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ControlSchemes {
    pub control_scheme: i32,
    pub scheme_name: String,
    pub rotation_speed: Option<f64>,
    pub walk_forward_speed: Option<f64>,
    pub walk_backward_speed: Option<f64>,
    pub walk_strafe_speed: Option<f64>,
    pub walk_strafe_forward_speed: Option<f64>,
    pub walk_strafe_backward_speed: Option<f64>,
    pub run_backward_speed: Option<f64>,
    pub run_strafe_speed: Option<f64>,
    pub run_strafe_forward_speed: Option<f64>,
    pub run_strafe_backward_speed: Option<f64>,
    pub keyboard_zoom_sensitivity: Option<f64>,
    pub keyboard_pitch_sensitivity: Option<f64>,
    pub keyboard_yaw_sensitivity: Option<f64>,
    pub mouse_zoom_wheel_sensitivity: Option<f64>,
    pub x_mouse_move_sensitivity_modifier: Option<f64>,
    pub y_mouse_move_sensitivity_modifier: Option<f64>,
    pub freecam_speed_modifier: Option<f64>,
    pub freecam_slow_speed_multiplier: Option<f64>,
    pub freecam_fast_speed_multiplier: Option<f64>,
    pub freecam_mouse_modifier: Option<f64>,
    pub gamepad_pitch_rot_sensitivity: Option<f64>,
    pub gamepad_yaw_rot_sensitivity: Option<f64>,
    pub gamepad_trigger_sensitivity: Option<f64>,
}

impl FromCdClient for ControlSchemes {
    const TABLE: &'static str = "ControlSchemes";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            control_scheme: row.get(0)?,
            scheme_name: trim_to_string(row.get(1)?),
            rotation_speed: row.get(2)?,
            walk_forward_speed: row.get(3)?,
            walk_backward_speed: row.get(4)?,
            walk_strafe_speed: row.get(5)?,
            walk_strafe_forward_speed: row.get(6)?,
            walk_strafe_backward_speed: row.get(7)?,
            run_backward_speed: row.get(8)?,
            run_strafe_speed: row.get(9)?,
            run_strafe_forward_speed: row.get(10)?,
            run_strafe_backward_speed: row.get(11)?,
            keyboard_zoom_sensitivity: row.get(12)?,
            keyboard_pitch_sensitivity: row.get(13)?,
            keyboard_yaw_sensitivity: row.get(14)?,
            mouse_zoom_wheel_sensitivity: row.get(15)?,
            x_mouse_move_sensitivity_modifier: row.get(16)?,
            y_mouse_move_sensitivity_modifier: row.get(17)?,
            freecam_speed_modifier: row.get(18)?,
            freecam_slow_speed_multiplier: row.get(19)?,
            freecam_fast_speed_multiplier: row.get(20)?,
            freecam_mouse_modifier: row.get(21)?,
            gamepad_pitch_rot_sensitivity: row.get(22)?,
            gamepad_yaw_rot_sensitivity: row.get(23)?,
            gamepad_trigger_sensitivity: row.get(24)?,
        })
    }
}

impl HasKey for ControlSchemes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.control_scheme
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CurrencyDenominations {
    pub value: i32,
    pub objectid: i32,
}

impl FromCdClient for CurrencyDenominations {
    const TABLE: &'static str = "CurrencyDenominations";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            value: row.get(0)?,
            objectid: row.get(1)?,
        })
    }
}

impl HasKey for CurrencyDenominations {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.objectid
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CurrencyTable {
    pub currency_index: i32,
    pub npcminlevel: i32,
    pub minvalue: i32,
    pub maxvalue: i32,
    pub id: i32,
}

impl FromCdClient for CurrencyTable {
    const TABLE: &'static str = "CurrencyTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            currency_index: row.get(0)?,
            npcminlevel: row.get(1)?,
            minvalue: row.get(2)?,
            maxvalue: row.get(3)?,
            id: row.get(4)?,
        })
    }
}

impl HasKey for CurrencyTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DbExclude {
    pub table: String,
    pub column: String,
}

impl FromCdClient for DbExclude {
    const TABLE: &'static str = "DBExclude";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            table: trim_to_string(row.get(0)?),
            column: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for DbExclude {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.table
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DeletionRestrictions {
    pub id: i32,
    pub restricted: bool,
    pub ids: Option<Vec<i32>>,
    pub check_type: i32,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for DeletionRestrictions {
    const TABLE: &'static str = "DeletionRestrictions";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            restricted: row.get(1)?,
            ids: parse_optional_comma_list(row.get(2)?),
            check_type: row.get(3)?,
            localize: row.get(4)?,
            loc_status: row.get(5)?,
            gate_version: trim_and_nullify(row.get(6)?),
        })
    }
}

impl HasKey for DeletionRestrictions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DestructibleComponent {
    pub id: i32,
    pub faction: Option<i32>,
    pub faction_list: Option<String>,
    pub life: Option<i32>,
    pub imagination: Option<i32>,
    pub loot_matrix_index: Option<i32>,
    pub currency_index: Option<i32>,
    pub level: Option<i32>,
    pub armor: Option<f64>,
    pub death_behavior: i32,
    pub isnpc: Option<bool>,
    pub attack_priority: i32,
    pub is_smashable: bool,
    pub difficulty_level: Option<i32>,
}

impl FromCdClient for DestructibleComponent {
    const TABLE: &'static str = "DestructibleComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            faction: row.get(1)?,
            faction_list: trim_and_nullify(row.get(2)?),
            life: row.get(3)?,
            imagination: row.get(4)?,
            loot_matrix_index: row.get(5)?,
            currency_index: row.get(6)?,
            level: row.get(7)?,
            armor: row.get(8)?,
            death_behavior: row.get(9)?,
            isnpc: row.get(10)?,
            attack_priority: row.get(11)?,
            is_smashable: row.get(12)?,
            difficulty_level: row.get(13)?,
        })
    }
}

impl HasKey for DestructibleComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DevModelBehaviors {
    pub model_id: i32,
    pub behavior_id: i32,
}

impl FromCdClient for DevModelBehaviors {
    const TABLE: &'static str = "DevModelBehaviors";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            model_id: row.get(0)?,
            behavior_id: row.get(1)?,
        })
    }
}

impl HasKey for DevModelBehaviors {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.model_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Emotes {
    pub id: i32,
    pub animation_name: Option<String>,
    pub icon_filename: String,
    pub channel: Option<String>,
    pub command: Option<String>,
    pub locked: bool,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for Emotes {
    const TABLE: &'static str = "Emotes";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            animation_name: trim_and_nullify(row.get(1)?),
            icon_filename: trim_to_string(row.get(2)?),
            channel: trim_and_nullify(row.get(3)?),
            command: trim_and_nullify(row.get(4)?),
            locked: row.get(5)?,
            localize: row.get(6)?,
            loc_status: row.get(7)?,
            gate_version: trim_and_nullify(row.get(8)?),
        })
    }
}

impl HasKey for Emotes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct EventGating {
    pub event_name: String,
    pub date_start: i64,
    pub date_end: i64,
}

impl FromCdClient for EventGating {
    const TABLE: &'static str = "EventGating";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            event_name: trim_to_string(row.get(0)?),
            date_start: row.get(1)?,
            date_end: row.get(2)?,
        })
    }
}

impl HasKey for EventGating {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.event_name
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ExhibitComponent {
    pub id: i32,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
    pub f_reputation_size_multiplier: f64,
    pub f_imagination_cost: f64,
}

impl FromCdClient for ExhibitComponent {
    const TABLE: &'static str = "ExhibitComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            length: row.get(1)?,
            width: row.get(2)?,
            height: row.get(3)?,
            offset_x: row.get(4)?,
            offset_y: row.get(5)?,
            offset_z: row.get(6)?,
            f_reputation_size_multiplier: row.get(7)?,
            f_imagination_cost: row.get(8)?,
        })
    }
}

impl HasKey for ExhibitComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Factions {
    pub faction: i32,
    pub faction_list: Vec<i32>,
    pub faction_list_friendly: bool,
    pub friend_list: Option<Vec<i32>>,
    pub enemy_list: Option<Vec<i32>>,
}

impl FromCdClient for Factions {
    const TABLE: &'static str = "Factions";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            faction: row.get(0)?,
            faction_list: parse_required_comma_list(row.get(1)?),
            faction_list_friendly: row.get(2)?,
            friend_list: parse_optional_comma_list(row.get(3)?),
            enemy_list: parse_optional_comma_list(row.get(4)?),
        })
    }
}

impl HasKey for Factions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.faction
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FeatureGating {
    pub feature_name: String,
    pub major: i32,
    pub current: i32,
    pub minor: i32,
    pub description: Option<String>,
}

impl FromCdClient for FeatureGating {
    const TABLE: &'static str = "FeatureGating";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            feature_name: trim_to_string(row.get(0)?),
            major: row.get(1)?,
            current: row.get(2)?,
            minor: row.get(3)?,
            description: trim_and_nullify(row.get(4)?),
        })
    }
}

impl HasKey for FeatureGating {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.feature_name
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FlairTable {
    pub id: i32,
    pub asset: String,
}

impl FromCdClient for FlairTable {
    const TABLE: &'static str = "FlairTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            asset: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for FlairTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Icons {
    pub icon_id: i32,
    pub icon_path: Option<String>,
    pub icon_name: Option<String>,
}

impl FromCdClient for Icons {
    const TABLE: &'static str = "Icons";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            icon_id: row.get(0)?,
            icon_path: trim_and_nullify(row.get(1)?),
            icon_name: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for Icons {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.icon_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InventoryComponent {
    pub id: i32,
    pub itemid: i32,
    pub count: i32,
    pub equip: bool,
}

impl FromCdClient for InventoryComponent {
    const TABLE: &'static str = "InventoryComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            itemid: row.get(1)?,
            count: row.get(2)?,
            equip: row.get(3)?,
        })
    }
}

// impl HasKey for InventoryComponent {
//     type Key = i32;
//
//     // multiple groupings
//     fn get_key(&self) -> &Self::Key {
//         todo!()
//         // &self.id
//     }
// }

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ItemComponent {
    pub id: i32,
    pub equip_location: Option<String>,
    pub base_value: Option<i32>,
    pub is_kit_piece: bool,
    pub rarity: Option<i32>,
    pub item_type: i32,
    pub item_info: Option<i64>,
    pub in_loot_table: bool,
    pub in_vendor: bool,
    pub is_unique: bool,
    pub is_bop: bool,
    pub is_boe: bool,
    pub req_flag_id: Option<i32>,
    pub req_specialty_id: Option<i32>,
    pub req_spec_rank: Option<i32>,
    pub req_achievement_id: Option<i32>,
    pub stack_size: Option<i32>,
    pub color1: Option<i32>,
    pub decal: Option<i32>,
    pub offset_group_id: Option<i32>,
    pub build_types: Option<i32>,
    pub req_precondition: Option<Vec<i32>>,
    pub animation_flag: Option<i32>,
    pub equip_effects: Option<i32>,
    pub ready_for_qa: Option<bool>,
    pub item_rating: Option<i32>,
    pub is_two_handed: Option<bool>,
    pub min_num_required: Option<i32>,
    pub del_res_index: Option<i32>,
    pub currency_lot: Option<i32>,
    pub alt_currency_cost: Option<i32>,
    pub sub_items: Option<Vec<i32>>,
    pub audio_event_use: Option<String>,
    pub no_equip_animation: bool,
    pub commendation_lot: Option<i32>,
    pub commendation_cost: Option<i32>,
    pub audio_equip_meta_event_set: Option<String>,
    pub currency_costs: Option<Vec<(i32, i32)>>,
    pub ingredient_info: Option<String>,
    pub loc_status: Option<i32>,
    pub forge_type: Option<i32>,
    pub sell_multiplier: Option<f64>,
}

fn parse_currency_costs(input: String) -> Option<Vec<(i32, i32)>> {
    let mut elements = vec![];
    for pair in input.split(',') {
        let (id, count) = pair.trim().split_once(':')?;
        elements.push((id.parse().ok()?, count.parse().ok()?))
    }
    Some(elements)
}

impl FromCdClient for ItemComponent {
    const TABLE: &'static str = "ItemComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            equip_location: trim_and_nullify(row.get(1)?),
            base_value: row.get(2)?,
            is_kit_piece: row.get(3)?,
            rarity: row.get(4)?,
            item_type: row.get(5)?,
            item_info: row.get(6)?,
            in_loot_table: row.get(7)?,
            in_vendor: row.get(8)?,
            is_unique: row.get(9)?,
            is_bop: row.get(10)?,
            is_boe: row.get(11)?,
            req_flag_id: row.get(12)?,
            req_specialty_id: row.get(13)?,
            req_spec_rank: row.get(14)?,
            req_achievement_id: row.get(15)?,
            stack_size: row.get(16)?,
            color1: row.get(17)?,
            decal: row.get(18)?,
            offset_group_id: row.get(19)?,
            build_types: row.get(20)?,
            req_precondition: parse_optional_list(row.get(21)?, ';'),
            animation_flag: row.get(22)?,
            equip_effects: row.get(23)?,
            ready_for_qa: row.get(24)?,
            item_rating: row.get(25)?,
            is_two_handed: row.get(26)?,
            min_num_required: row.get(27)?,
            del_res_index: row.get(28)?,
            currency_lot: row.get(29)?,
            alt_currency_cost: row.get(30)?,
            sub_items: parse_optional_comma_list(row.get(31)?),
            audio_event_use: trim_and_nullify(row.get(32)?),
            no_equip_animation: row.get(33)?,
            commendation_lot: row.get(34)?,
            commendation_cost: row.get(35)?,
            audio_equip_meta_event_set: trim_and_nullify(row.get(36)?),
            currency_costs: parse_currency_costs(row.get(37)?),
            ingredient_info: trim_and_nullify(row.get(38)?),
            loc_status: row.get(39)?,
            forge_type: row.get(40)?,
            sell_multiplier: row.get(41)?,
        })
    }
}

impl HasKey for ItemComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ItemEggData {
    pub id: i32,
    pub chassie_type_id: i32,
}

impl FromCdClient for ItemEggData {
    const TABLE: &'static str = "ItemEggData";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            chassie_type_id: row.get(1)?,
        })
    }
}

impl HasKey for ItemEggData {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ItemFoodData {
    pub id: i32,
    pub element_1: i32,
    pub element_1_amount: i32,
    pub element_2: i32,
    pub element_2_amount: i32,
    pub element_3: i32,
    pub element_3_amount: i32,
    pub element_4: i32,
    pub element_4_amount: i32,
}

impl FromCdClient for ItemFoodData {
    const TABLE: &'static str = "ItemFoodData";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            element_1: row.get(1)?,
            element_1_amount: row.get(2)?,
            element_2: row.get(3)?,
            element_2_amount: row.get(4)?,
            element_3: row.get(5)?,
            element_3_amount: row.get(6)?,
            element_4: row.get(7)?,
            element_4_amount: row.get(8)?,
        })
    }
}

impl HasKey for ItemFoodData {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ItemSetSkills {
    pub skill_set_id: i32,
    pub skill_id: i32,
    pub skill_cast_type: i32,
}

impl FromCdClient for ItemSetSkills {
    const TABLE: &'static str = "ItemSetSkills";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            skill_set_id: row.get(0)?,
            skill_id: row.get(1)?,
            skill_cast_type: row.get(2)?,
        })
    }
}

impl HasGroupKey for ItemSetSkills {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.skill_set_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ItemSets {
    pub set_id: i32,
    pub loc_status: i32,
    pub item_ids: Vec<i32>,
    pub kit_type: i32,
    pub kit_rank: Option<i32>,
    pub kit_image: Option<i32>,
    pub skill_set_with2: Option<i32>,
    pub skill_set_with3: Option<i32>,
    pub skill_set_with4: Option<i32>,
    pub skill_set_with5: Option<i32>,
    pub skill_set_with6: Option<i32>,
    pub localize: Option<bool>,
    pub gate_version: Option<String>,
    pub kit_id: Option<i32>,
    pub priority: Option<f64>,
}

impl FromCdClient for ItemSets {
    const TABLE: &'static str = "ItemSets";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            set_id: row.get(0)?,
            loc_status: row.get(1)?,
            item_ids: parse_required_comma_list(row.get(2)?),
            kit_type: row.get(3)?,
            kit_rank: row.get(4)?,
            kit_image: row.get(5)?,
            skill_set_with2: row.get(6)?,
            skill_set_with3: row.get(7)?,
            skill_set_with4: row.get(8)?,
            skill_set_with5: row.get(9)?,
            skill_set_with6: row.get(10)?,
            localize: row.get(11)?,
            gate_version: trim_and_nullify(row.get(12)?),
            kit_id: row.get(13)?,
            priority: row.get(14)?,
        })
    }
}

impl HasKey for ItemSets {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.set_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct JetPackPadComponent {
    pub id: i32,
    pub x_distance: f64,
    pub y_distance: f64,
    pub warn_distance: f64,
    pub lot_blocker: Option<i32>,
    pub lot_warning_volume: Option<i32>,
}

impl FromCdClient for JetPackPadComponent {
    const TABLE: &'static str = "JetPackPadComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            x_distance: row.get(1)?,
            y_distance: row.get(2)?,
            warn_distance: row.get(3)?,
            lot_blocker: row.get(4)?,
            lot_warning_volume: row.get(5)?,
        })
    }
}

impl HasKey for JetPackPadComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LuPExhibitComponent {
    pub id: i32,
    pub min_xz: f64,
    pub max_xz: f64,
    pub max_y: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
}

impl FromCdClient for LuPExhibitComponent {
    const TABLE: &'static str = "LUPExhibitComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            min_xz: row.get(1)?,
            max_xz: row.get(2)?,
            max_y: row.get(3)?,
            offset_x: row.get(4)?,
            offset_y: row.get(5)?,
            offset_z: row.get(6)?,
        })
    }
}

impl HasKey for LuPExhibitComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LuPExhibitModelData {
    pub lot: i32,
    pub min_xz: f64,
    pub max_xz: f64,
    pub max_y: f64,
    pub description: String,
    pub owner: String,
}

impl FromCdClient for LuPExhibitModelData {
    const TABLE: &'static str = "LUPExhibitModelData";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            lot: row.get(0)?,
            min_xz: row.get(1)?,
            max_xz: row.get(2)?,
            max_y: row.get(3)?,
            description: trim_to_string(row.get(4)?),
            owner: trim_to_string(row.get(5)?),
        })
    }
}

impl HasKey for LuPExhibitModelData {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.lot
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LuPZoneIDs {
    pub zone_id: i32,
}

impl FromCdClient for LuPZoneIDs {
    const TABLE: &'static str = "LUPZoneIDs";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            zone_id: row.get(0)?,
        })
    }
}

impl HasKey for LuPZoneIDs {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.zone_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LanguageType {
    pub language_id: i32,
    pub language_description: String,
}

impl FromCdClient for LanguageType {
    const TABLE: &'static str = "LanguageType";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            language_id: row.get(0)?,
            language_description: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for LanguageType {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.language_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LevelProgressionLookup {
    pub id: i32,
    pub required_uscore: i32,
    pub behavior_effect: Option<String>,
}

impl FromCdClient for LevelProgressionLookup {
    const TABLE: &'static str = "LevelProgressionLookup";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            required_uscore: row.get(1)?,
            behavior_effect: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for LevelProgressionLookup {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LootMatrix {
    pub loot_matrix_index: i32,
    pub loot_table_index: i32,
    pub rarity_table_index: i32,
    pub percent: f64,
    pub min_to_drop: i32,
    pub max_to_drop: i32,
    pub id: i32,
    pub flag_id: Option<i32>,
    pub gate_version: Option<String>,
}

impl FromCdClient for LootMatrix {
    const TABLE: &'static str = "LootMatrix";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            loot_matrix_index: row.get(0)?,
            loot_table_index: row.get(1)?,
            rarity_table_index: row.get(2)?,
            percent: row.get(3)?,
            min_to_drop: row.get(4)?,
            max_to_drop: row.get(5)?,
            id: row.get(6)?,
            flag_id: row.get(7)?,
            gate_version: trim_and_nullify(row.get(8)?),
        })
    }
}

impl HasGroupKey for LootMatrix {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.loot_matrix_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LootMatrixIndex {
    pub loot_matrix_index: i32,
    pub in_npc_editor: bool,
}

impl FromCdClient for LootMatrixIndex {
    const TABLE: &'static str = "LootMatrixIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            loot_matrix_index: row.get(0)?,
            in_npc_editor: row.get(1)?,
        })
    }
}

impl HasKey for LootMatrixIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.loot_matrix_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LootTable {
    pub itemid: i32,
    pub loot_table_index: i32,
    pub id: i32,
    pub mission_drop: bool,
    pub sort_priority: i32,
}

impl FromCdClient for LootTable {
    const TABLE: &'static str = "LootTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            itemid: row.get(0)?,
            loot_table_index: row.get(1)?,
            id: row.get(2)?,
            mission_drop: row.get(3)?,
            sort_priority: row.get(4)?,
        })
    }
}

// impl HasGroupKey for LootTable {
//     type Key = i32;
//
//     // multiple groupings
//     fn get_group_key(&self) -> &Self::Key {
//         &self.itemid
//     }
// }

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LootTableIndex {
    pub loot_table_index: i32,
}

impl FromCdClient for LootTableIndex {
    const TABLE: &'static str = "LootTableIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            loot_table_index: row.get(0)?,
        })
    }
}

impl HasKey for LootTableIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.loot_table_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MinifigComponent {
    pub id: i32,
    pub head: i32,
    pub chest: i32,
    pub legs: i32,
    pub hairstyle: i32,
    pub haircolor: i32,
    pub chestdecal: i32,
    pub headcolor: i32,
    pub lefthand: i32,
    pub righthand: i32,
    pub eyebrowstyle: i32,
    pub eyesstyle: i32,
    pub mouthstyle: i32,
}

impl FromCdClient for MinifigComponent {
    const TABLE: &'static str = "MinifigComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            head: row.get(1)?,
            chest: row.get(2)?,
            legs: row.get(3)?,
            hairstyle: row.get(4)?,
            haircolor: row.get(5)?,
            chestdecal: row.get(6)?,
            headcolor: row.get(7)?,
            lefthand: row.get(8)?,
            righthand: row.get(9)?,
            eyebrowstyle: row.get(10)?,
            eyesstyle: row.get(11)?,
            mouthstyle: row.get(12)?,
        })
    }
}

impl HasKey for MinifigComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MinifigDecalsEyebrows {
    pub id: i32,
    pub high_path: String,
    pub low_path: String,
    pub character_create_valid: bool,
    pub male: bool,
    pub female: bool,
}

impl FromCdClient for MinifigDecalsEyebrows {
    const TABLE: &'static str = "MinifigDecals_Eyebrows";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            high_path: trim_to_string(row.get(1)?),
            low_path: trim_to_string(row.get(2)?),
            character_create_valid: row.get(3)?,
            male: row.get(4)?,
            female: row.get(5)?,
        })
    }
}

impl HasKey for MinifigDecalsEyebrows {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MinifigDecalsEyes {
    pub id: i32,
    pub high_path: String,
    pub low_path: String,
    pub character_create_valid: bool,
    pub male: bool,
    pub female: bool,
}

impl FromCdClient for MinifigDecalsEyes {
    const TABLE: &'static str = "MinifigDecals_Eyes";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            high_path: trim_to_string(row.get(1)?),
            low_path: trim_to_string(row.get(2)?),
            character_create_valid: row.get(3)?,
            male: row.get(4)?,
            female: row.get(5)?,
        })
    }
}

impl HasKey for MinifigDecalsEyes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MinifigDecalsLegs {
    pub id: i32,
    pub high_path: String,
}

impl FromCdClient for MinifigDecalsLegs {
    const TABLE: &'static str = "MinifigDecals_Legs";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            high_path: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for MinifigDecalsLegs {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MinifigDecalsMouths {
    pub id: i32,
    pub high_path: String,
    pub low_path: String,
    pub character_create_valid: bool,
    pub male: bool,
    pub female: bool,
}

impl FromCdClient for MinifigDecalsMouths {
    const TABLE: &'static str = "MinifigDecals_Mouths";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            high_path: trim_to_string(row.get(1)?),
            low_path: trim_to_string(row.get(2)?),
            character_create_valid: row.get(3)?,
            male: row.get(4)?,
            female: row.get(5)?,
        })
    }
}

impl HasKey for MinifigDecalsMouths {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MinifigDecalsTorsos {
    pub id: i32,
    pub high_path: String,
    pub character_create_valid: bool,
    pub male: bool,
    pub female: bool,
}

impl FromCdClient for MinifigDecalsTorsos {
    const TABLE: &'static str = "MinifigDecals_Torsos";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            high_path: trim_to_string(row.get(1)?),
            character_create_valid: row.get(2)?,
            male: row.get(3)?,
            female: row.get(4)?,
        })
    }
}

impl HasKey for MinifigDecalsTorsos {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MissionEmail {
    pub id: i32,
    pub message_type: i32,
    pub notification_group: i32,
    pub mission_id: i32,
    pub attachment_lot: Option<i32>,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for MissionEmail {
    const TABLE: &'static str = "MissionEmail";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            message_type: row.get(1)?,
            notification_group: row.get(2)?,
            mission_id: row.get(3)?,
            attachment_lot: row.get(4)?,
            localize: row.get(5)?,
            loc_status: row.get(6)?,
            gate_version: trim_and_nullify(row.get(7)?),
        })
    }
}

impl HasKey for MissionEmail {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MissionNPCComponent {
    pub id: i32,
    pub mission_id: i32,
    pub offers_mission: bool,
    pub accepts_mission: bool,
    pub gate_version: Option<String>,
}

impl FromCdClient for MissionNPCComponent {
    const TABLE: &'static str = "MissionNPCComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            mission_id: row.get(1)?,
            offers_mission: row.get(2)?,
            accepts_mission: row.get(3)?,
            gate_version: trim_and_nullify(row.get(4)?),
        })
    }
}

// impl HasKey for MissionNPCComponent {
//     type Key = i32;
//
//     // multiple groupings
//     fn get_key(&self) -> &Self::Key {
//         &self.id
//     }
// }

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MissionTasks {
    pub id: i32,
    pub loc_status: i32,
    pub task_type: i32,
    pub target: Option<i32>,
    pub target_group: Option<Vec<i32>>,
    pub target_value: Option<i32>,
    pub task_param1: Option<Vec<i32>>,
    pub large_task_icon: Option<String>,
    pub icon_id: Option<i32>,
    pub uid: i32,
    pub large_task_icon_id: Option<i32>,
    pub localize: bool,
    pub gate_version: Option<String>,
}

impl FromCdClient for MissionTasks {
    const TABLE: &'static str = "MissionTasks";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            loc_status: row.get(1)?,
            task_type: row.get(2)?,
            target: row.get(3)?,
            target_group: parse_optional_comma_list(row.get(4)?),
            target_value: row.get(5)?,
            task_param1: parse_optional_comma_list(row.get(6)?),
            large_task_icon: trim_and_nullify(row.get(7)?),
            icon_id: row.get(8)?,
            uid: row.get(9)?,
            large_task_icon_id: row.get(10)?,
            localize: row.get(11)?,
            gate_version: trim_and_nullify(row.get(12)?),
        })
    }
}

// impl HasKey for MissionTasks {
//     type Key = i32;
//
//     fn get_key(&self) -> &Self::Key {
//         todo!("i dont know")
//         // &self.id
//     }
// }

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MissionText {
    pub id: i32,
    pub story_icon: Option<String>,
    pub mission_icon: Option<String>,
    pub offer_npcicon: Option<String>,
    pub icon_id: Option<i32>,
    pub state_1_anim: Option<String>,
    pub state_2_anim: Option<String>,
    pub state_3_anim: Option<String>,
    pub state_4_anim: Option<String>,
    pub state_3_turnin_anim: Option<String>,
    pub state_4_turnin_anim: Option<String>,
    pub onclick_anim: Option<String>,
    pub cinematic_accepted: Option<String>,
    pub cinematic_accepted_leadin: Option<f64>,
    pub cinematic_completed: Option<String>,
    pub cinematic_completed_leadin: Option<f64>,
    pub cinematic_repeatable: Option<String>,
    pub cinematic_repeatable_leadin: Option<f64>,
    pub cinematic_repeatable_completed: Option<String>,
    pub cinematic_repeatable_completed_leadin: Option<f64>,
    pub audio_event_guid_interact: Option<String>,
    pub audio_event_guid_offer_accept: Option<String>,
    pub audio_event_guid_offer_deny: Option<String>,
    pub audio_event_guid_completed: Option<String>,
    pub audio_event_guid_turn_in: Option<String>,
    pub audio_event_guid_failed: Option<String>,
    pub audio_event_guid_progress: Option<String>,
    pub audio_music_cue_offer_accept: Option<String>,
    pub audio_music_cue_turn_in: Option<String>,
    pub turn_in_icon_id: Option<i32>,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for MissionText {
    const TABLE: &'static str = "MissionText";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            story_icon: trim_and_nullify(row.get(1)?),
            mission_icon: trim_and_nullify(row.get(2)?),
            offer_npcicon: trim_and_nullify(row.get(3)?),
            icon_id: row.get(4)?,
            state_1_anim: trim_and_nullify(row.get(5)?),
            state_2_anim: trim_and_nullify(row.get(6)?),
            state_3_anim: trim_and_nullify(row.get(7)?),
            state_4_anim: trim_and_nullify(row.get(8)?),
            state_3_turnin_anim: trim_and_nullify(row.get(9)?),
            state_4_turnin_anim: trim_and_nullify(row.get(10)?),
            onclick_anim: trim_and_nullify(row.get(11)?),
            cinematic_accepted: trim_and_nullify(row.get(12)?),
            cinematic_accepted_leadin: row.get(13)?,
            cinematic_completed: trim_and_nullify(row.get(14)?),
            cinematic_completed_leadin: row.get(15)?,
            cinematic_repeatable: trim_and_nullify(row.get(16)?),
            cinematic_repeatable_leadin: row.get(17)?,
            cinematic_repeatable_completed: trim_and_nullify(row.get(18)?),
            cinematic_repeatable_completed_leadin: row.get(19)?,
            audio_event_guid_interact: trim_and_nullify(row.get(20)?),
            audio_event_guid_offer_accept: trim_and_nullify(row.get(21)?),
            audio_event_guid_offer_deny: trim_and_nullify(row.get(22)?),
            audio_event_guid_completed: trim_and_nullify(row.get(23)?),
            audio_event_guid_turn_in: trim_and_nullify(row.get(24)?),
            audio_event_guid_failed: trim_and_nullify(row.get(25)?),
            audio_event_guid_progress: trim_and_nullify(row.get(26)?),
            audio_music_cue_offer_accept: trim_and_nullify(row.get(27)?),
            audio_music_cue_turn_in: trim_and_nullify(row.get(28)?),
            turn_in_icon_id: row.get(29)?,
            localize: row.get(30)?,
            loc_status: row.get(31)?,
            gate_version: trim_and_nullify(row.get(32)?),
        })
    }
}

impl HasKey for MissionText {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Missions {
    pub id: i32,
    pub defined_type: String,
    pub defined_subtype: Option<String>,
    pub uisort_order: Option<i32>,
    pub offer_object_id: Option<i32>,
    pub target_object_id: Option<i32>,
    pub reward_currency: Option<i64>,
    pub lego_score: i32,
    pub reward_reputation: Option<i64>,
    pub is_choice_reward: Option<bool>,
    pub reward_item1: i32,
    pub reward_item1_count: i32,
    pub reward_item2: i32,
    pub reward_item2_count: i32,
    pub reward_item3: i32,
    pub reward_item3_count: i32,
    pub reward_item4: i32,
    pub reward_item4_count: i32,
    pub reward_emote: i32,
    pub reward_emote2: i32,
    pub reward_emote3: Option<i32>,
    pub reward_emote4: Option<i32>,
    pub reward_maximagination: i32,
    pub reward_maxhealth: i32,
    pub reward_maxinventory: i32,
    pub reward_maxmodel: Option<i32>,
    pub reward_maxwidget: Option<i32>,
    pub reward_maxwallet: Option<i64>,
    pub repeatable: bool,
    pub reward_currency_repeatable: Option<i64>,
    pub reward_item1_repeatable: i32,
    pub reward_item1_repeat_count: i32,
    pub reward_item2_repeatable: i32,
    pub reward_item2_repeat_count: i32,
    pub reward_item3_repeatable: i32,
    pub reward_item3_repeat_count: i32,
    pub reward_item4_repeatable: i32,
    pub reward_item4_repeat_count: i32,
    pub time_limit: Option<i32>,
    pub is_mission: bool,
    pub mission_icon_id: Option<i32>,
    pub prereq_mission_id: Option<Vec<MissionPreReqType>>,
    pub localize: bool,
    pub in_motd: bool,
    pub cooldown_time: Option<i64>,
    pub is_random: bool,
    pub random_pool: Option<Vec<i32>>,
    pub uiprereq_id: Option<i32>,
    pub gate_version: Option<String>,
    pub hudstates: Option<String>,
    pub loc_status: i32,
    pub reward_bankinventory: Option<i32>,
}

#[derive(Clone, Debug)]
pub enum MissionPreReqType {
    OneOf(Vec<i32>),
    Required(i32),
}

fn parse_mission_prereqs(input: String) -> Option<Vec<MissionPreReqType>> {
    let mut elements = vec![];
    for value in input.split(',') {
        if value.contains("|") {
            let options = value
                .split('|')
                .filter_map(|val| {
                    let start = val.find(char::is_numeric)?;
                    let remaining = &val[start..];
                    let end = remaining
                        .find(|c| !char::is_numeric(c))
                        .unwrap_or_else(|| remaining.len());
                    val[start..start + end].parse().ok()
                })
                .collect();
            elements.push(MissionPreReqType::OneOf(options));
        } else {
            elements.push(MissionPreReqType::Required(value.trim().parse().ok()?));
        }
    }
    Some(elements)
}

impl FromCdClient for Missions {
    const TABLE: &'static str = "Missions";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            defined_type: trim_to_string(row.get(1)?),
            defined_subtype: trim_and_nullify(row.get(2)?),
            uisort_order: row.get(3)?,
            offer_object_id: row.get(4)?,
            target_object_id: row.get(5)?,
            reward_currency: row.get(6)?,
            lego_score: row.get(7)?,
            reward_reputation: row.get(8)?,
            is_choice_reward: row.get(9)?,
            reward_item1: row.get(10)?,
            reward_item1_count: row.get(11)?,
            reward_item2: row.get(12)?,
            reward_item2_count: row.get(13)?,
            reward_item3: row.get(14)?,
            reward_item3_count: row.get(15)?,
            reward_item4: row.get(16)?,
            reward_item4_count: row.get(17)?,
            reward_emote: row.get(18)?,
            reward_emote2: row.get(19)?,
            reward_emote3: row.get(20)?,
            reward_emote4: row.get(21)?,
            reward_maximagination: row.get(22)?,
            reward_maxhealth: row.get(23)?,
            reward_maxinventory: row.get(24)?,
            reward_maxmodel: row.get(25)?,
            reward_maxwidget: row.get(26)?,
            reward_maxwallet: row.get(27)?,
            repeatable: row.get(28)?,
            reward_currency_repeatable: row.get(29)?,
            reward_item1_repeatable: row.get(30)?,
            reward_item1_repeat_count: row.get(31)?,
            reward_item2_repeatable: row.get(32)?,
            reward_item2_repeat_count: row.get(33)?,
            reward_item3_repeatable: row.get(34)?,
            reward_item3_repeat_count: row.get(35)?,
            reward_item4_repeatable: row.get(36)?,
            reward_item4_repeat_count: row.get(37)?,
            time_limit: row.get(38)?,
            is_mission: row.get(39)?,
            mission_icon_id: row.get(40)?,
            prereq_mission_id: parse_mission_prereqs(row.get(41)?),
            localize: row.get(42)?,
            in_motd: row.get(43)?,
            cooldown_time: row.get(44)?,
            is_random: row.get(45)?,
            random_pool: parse_optional_comma_list(row.get(46)?),
            uiprereq_id: row.get(47)?,
            gate_version: trim_and_nullify(row.get(48)?),
            hudstates: trim_and_nullify(row.get(49)?),
            loc_status: row.get(50)?,
            reward_bankinventory: row.get(51)?,
        })
    }
}

impl HasKey for Missions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ModelBehavior {
    pub id: i32,
    pub definition_xmlfilename: String,
}

impl FromCdClient for ModelBehavior {
    const TABLE: &'static str = "ModelBehavior";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            definition_xmlfilename: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for ModelBehavior {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ModularBuildComponent {
    pub id: i32,
    pub build_type: i32,
    pub xml: String,
    pub created_lot: i32,
    pub created_physics_id: i32,
    pub audio_event_guid_snap: String,
    pub audio_event_guid_complete: Option<String>,
    pub audio_event_guid_present: Option<String>,
}

impl FromCdClient for ModularBuildComponent {
    const TABLE: &'static str = "ModularBuildComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            build_type: row.get(1)?,
            xml: trim_to_string(row.get(2)?),
            created_lot: row.get(3)?,
            created_physics_id: row.get(4)?,
            audio_event_guid_snap: trim_to_string(row.get(5)?),
            audio_event_guid_complete: trim_and_nullify(row.get(6)?),
            audio_event_guid_present: trim_and_nullify(row.get(7)?),
        })
    }
}

impl HasKey for ModularBuildComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ModuleComponent {
    pub id: i32,
    pub part_code: i32,
    pub build_type: i32,
    pub xml: String,
    pub primary_sound_guid: Option<String>,
    pub assembled_effect_id: Option<i32>,
}

impl FromCdClient for ModuleComponent {
    const TABLE: &'static str = "ModuleComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            part_code: row.get(1)?,
            build_type: row.get(2)?,
            xml: trim_to_string(row.get(3)?),
            primary_sound_guid: trim_and_nullify(row.get(4)?),
            assembled_effect_id: row.get(5)?,
        })
    }
}

impl HasKey for ModuleComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MotionFX {
    pub id: i32,
    pub type_id: i32,
    pub slam_velocity: Option<f64>,
    pub add_velocity: Option<f64>,
    pub duration: Option<f64>,
    pub dest_group_name: Option<String>,
    pub start_scale: Option<f64>,
    pub end_scale: Option<f64>,
    pub velocity: Option<f64>,
    pub distance: Option<f64>,
}

impl FromCdClient for MotionFX {
    const TABLE: &'static str = "MotionFX";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            type_id: row.get(1)?,
            slam_velocity: row.get(2)?,
            add_velocity: row.get(3)?,
            duration: row.get(4)?,
            dest_group_name: trim_and_nullify(row.get(5)?),
            start_scale: row.get(6)?,
            end_scale: row.get(7)?,
            velocity: row.get(8)?,
            distance: row.get(9)?,
        })
    }
}

impl HasKey for MotionFX {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MovementAIComponent {
    pub id: i32,
    pub movement_type: String,
    pub wander_chance: f64,
    pub wander_delay_min: f64,
    pub wander_delay_max: f64,
    pub wander_speed: f64,
    pub wander_radius: f64,
    pub attached_path: Option<String>,
}

impl FromCdClient for MovementAIComponent {
    const TABLE: &'static str = "MovementAIComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            movement_type: trim_to_string(row.get(1)?),
            wander_chance: row.get(2)?,
            wander_delay_min: row.get(3)?,
            wander_delay_max: row.get(4)?,
            wander_speed: row.get(5)?,
            wander_radius: row.get(6)?,
            attached_path: trim_and_nullify(row.get(7)?),
        })
    }
}

impl HasKey for MovementAIComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MovingPlatforms {
    pub id: i32,
    pub platform_is_simple_mover: bool,
    pub platform_move_x: f64,
    pub platform_move_y: f64,
    pub platform_move_z: f64,
    pub platform_move_time: f64,
    pub platform_start_at_end: bool,
    pub description: String,
}

impl FromCdClient for MovingPlatforms {
    const TABLE: &'static str = "MovingPlatforms";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            platform_is_simple_mover: row.get(1)?,
            platform_move_x: row.get(2)?,
            platform_move_y: row.get(3)?,
            platform_move_z: row.get(4)?,
            platform_move_time: row.get(5)?,
            platform_start_at_end: row.get(6)?,
            description: trim_to_string(row.get(7)?),
        })
    }
}

impl HasKey for MovingPlatforms {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct NpcIcons {
    pub id: i32,
    pub color: i32,
    pub offset: f64,
    pub lot: i32,
    pub texture: Option<String>,
    pub is_clickable: bool,
    pub scale: f64,
    pub rotate_to_face: bool,
    pub composite_horiz_offset: Option<f64>,
    pub composite_vert_offset: Option<f64>,
    pub composite_scale: Option<f64>,
    pub composite_connection_node: Option<String>,
    pub composite_lotmulti_mission: Option<i32>,
    pub composite_lotmulti_mission_ventor: Option<i32>,
    pub composite_icon_texture: Option<String>,
}

impl FromCdClient for NpcIcons {
    const TABLE: &'static str = "NpcIcons";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            color: row.get(1)?,
            offset: row.get(2)?,
            lot: row.get(3)?,
            texture: trim_and_nullify(row.get(4)?),
            is_clickable: row.get(5)?,
            scale: row.get(6)?,
            rotate_to_face: row.get(7)?,
            composite_horiz_offset: row.get(8)?,
            composite_vert_offset: row.get(9)?,
            composite_scale: row.get(10)?,
            composite_connection_node: trim_and_nullify(row.get(11)?),
            composite_lotmulti_mission: row.get(12)?,
            composite_lotmulti_mission_ventor: row.get(13)?,
            composite_icon_texture: trim_and_nullify(row.get(14)?),
        })
    }
}

impl HasKey for NpcIcons {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ObjectBehaviorXREF {
    pub lot: i32,
    pub behavior_id1: i64,
    pub behavior_id2: i64,
    pub behavior_id3: i64,
    pub behavior_id4: i64,
    pub behavior_id5: i64,
    pub r#type: i32,
}

impl FromCdClient for ObjectBehaviorXREF {
    const TABLE: &'static str = "ObjectBehaviorXREF";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            lot: row.get(0)?,
            behavior_id1: row.get(1)?,
            behavior_id2: row.get(2)?,
            behavior_id3: row.get(3)?,
            behavior_id4: row.get(4)?,
            behavior_id5: row.get(5)?,
            r#type: row.get(6)?,
        })
    }
}

impl HasKey for ObjectBehaviorXREF {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.lot
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ObjectBehaviors {
    pub behavior_id: i64,
    pub xmldata: String,
}

impl FromCdClient for ObjectBehaviors {
    const TABLE: &'static str = "ObjectBehaviors";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            behavior_id: row.get(0)?,
            xmldata: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for ObjectBehaviors {
    type Key = i64;

    fn get_key(&self) -> &Self::Key {
        &self.behavior_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ObjectSkills {
    pub object_template: i32,
    pub skill_id: i32,
    pub cast_on_type: Option<i32>,
    pub aicombat_weight: Option<i32>,
}

impl FromCdClient for ObjectSkills {
    const TABLE: &'static str = "ObjectSkills";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            object_template: row.get(0)?,
            skill_id: row.get(1)?,
            cast_on_type: row.get(2)?,
            aicombat_weight: row.get(3)?,
        })
    }
}

impl HasGroupKey for ObjectSkills {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.object_template
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Objects {
    pub id: i32,
    pub name: Option<String>,
    pub placeable: bool,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub localize: bool,
    pub npc_template_id: Option<i32>,
    pub display_name: Option<String>,
    pub interaction_distance: Option<f64>,
    pub nametag: Option<bool>,
    pub internal_notes: Option<String>,
    pub loc_status: Option<i32>,
    pub gate_version: Option<String>,
    pub hq_valid: Option<bool>,
}

impl FromCdClient for Objects {
    const TABLE: &'static str = "Objects";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: trim_and_nullify(row.get(1)?),
            placeable: row.get(2)?,
            r#type: trim_and_nullify(row.get(3)?),
            description: trim_and_nullify(row.get(4)?),
            localize: row.get(5)?,
            npc_template_id: row.get(6)?,
            display_name: trim_and_nullify(row.get(7)?),
            interaction_distance: row.get(8)?,
            nametag: row.get(9)?,
            internal_notes: trim_and_nullify(row.get(10)?),
            loc_status: row.get(11)?,
            gate_version: trim_and_nullify(row.get(12)?),
            hq_valid: row.get(13)?,
        })
    }
}

// There are a few duplicates (35006-35008)
impl HasKey for Objects {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PackageComponent {
    pub id: i32,
    pub loot_matrix_index: i32,
    pub package_type: i32,
}

impl FromCdClient for PackageComponent {
    const TABLE: &'static str = "PackageComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            loot_matrix_index: row.get(1)?,
            package_type: row.get(2)?,
        })
    }
}

impl HasKey for PackageComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PetAbilities {
    pub id: i32,
    pub ability_name: String,
    pub imagination_cost: i32,
    pub loc_status: i32,
}

impl FromCdClient for PetAbilities {
    const TABLE: &'static str = "PetAbilities";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            ability_name: trim_to_string(row.get(1)?),
            imagination_cost: row.get(2)?,
            loc_status: row.get(3)?,
        })
    }
}

impl HasKey for PetAbilities {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PetComponent {
    pub id: i32,
    pub min_tame_update_time: f64,
    pub max_tame_update_time: f64,
    pub percent_tame_chance: f64,
    pub tamability: f64,
    pub element_type: i32,
    pub walk_speed: f64,
    pub run_speed: f64,
    pub sprint_speed: f64,
    pub idle_time_min: f64,
    pub idle_time_max: f64,
    pub pet_form: i32,
    pub imagination_drain_rate: f64,
    pub audio_meta_event_set: Option<String>,
    pub buff_ids: Option<String>,
}

impl FromCdClient for PetComponent {
    const TABLE: &'static str = "PetComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            min_tame_update_time: row.get(1)?,
            max_tame_update_time: row.get(2)?,
            percent_tame_chance: row.get(3)?,
            tamability: row.get(4)?,
            element_type: row.get(5)?,
            walk_speed: row.get(6)?,
            run_speed: row.get(7)?,
            sprint_speed: row.get(8)?,
            idle_time_min: row.get(9)?,
            idle_time_max: row.get(10)?,
            pet_form: row.get(11)?,
            imagination_drain_rate: row.get(12)?,
            audio_meta_event_set: trim_and_nullify(row.get(13)?),
            buff_ids: trim_and_nullify(row.get(14)?),
        })
    }
}

impl HasKey for PetComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PetNestComponent {
    pub id: i32,
    pub elemental_type: i32,
}

impl FromCdClient for PetNestComponent {
    const TABLE: &'static str = "PetNestComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            elemental_type: row.get(1)?,
        })
    }
}

impl HasKey for PetNestComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PhysicsComponent {
    pub id: i32,
    pub r#static: f64,
    pub physics_asset: Option<String>,
    pub jump: f64,
    pub doublejump: f64,
    pub speed: Option<f64>,
    pub rot_speed: Option<f64>,
    pub player_height: Option<f64>,
    pub player_radius: Option<f64>,
    pub pc_shape_type: i32,
    pub collision_group: i32,
    pub air_speed: f64,
    pub boundary_asset: Option<String>,
    pub jump_air_speed: Option<f64>,
    pub friction: Option<f64>,
    pub gravity_volume_asset: Option<String>,
}

impl FromCdClient for PhysicsComponent {
    const TABLE: &'static str = "PhysicsComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            r#static: row.get(1)?,
            physics_asset: trim_and_nullify(row.get(2)?),
            jump: row.get(3)?,
            doublejump: row.get(4)?,
            speed: row.get(5)?,
            rot_speed: row.get(6)?,
            player_height: row.get(7)?,
            player_radius: row.get(8)?,
            pc_shape_type: row.get(9)?,
            collision_group: row.get(10)?,
            air_speed: row.get(11)?,
            boundary_asset: trim_and_nullify(row.get(12)?),
            jump_air_speed: row.get(13)?,
            friction: row.get(14)?,
            gravity_volume_asset: trim_and_nullify(row.get(15)?),
        })
    }
}

impl HasKey for PhysicsComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PlayerFlags {
    pub id: i32,
    pub session_only: bool,
    pub only_set_by_server: bool,
    pub session_zone_only: bool,
}

impl FromCdClient for PlayerFlags {
    const TABLE: &'static str = "PlayerFlags";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            session_only: row.get(1)?,
            only_set_by_server: row.get(2)?,
            session_zone_only: row.get(3)?,
        })
    }
}

impl HasKey for PlayerFlags {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PlayerStatistics {
    pub stat_id: i32,
    pub sort_order: Option<i32>,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for PlayerStatistics {
    const TABLE: &'static str = "PlayerStatistics";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            stat_id: row.get(0)?,
            sort_order: row.get(1)?,
            loc_status: row.get(2)?,
            gate_version: trim_and_nullify(row.get(3)?),
        })
    }
}

impl HasKey for PlayerStatistics {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.stat_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Preconditions {
    pub id: i32,
    pub r#type: Option<i32>,
    pub target_lot: Option<Vec<i32>>,
    pub target_group: Option<String>,
    pub target_count: Option<i32>,
    pub icon_id: Option<i32>,
    pub localize: bool,
    pub valid_contexts: i64,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for Preconditions {
    const TABLE: &'static str = "Preconditions";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            r#type: row.get(1)?,
            target_lot: parse_optional_comma_list(row.get(2)?),
            target_group: trim_and_nullify(row.get(3)?),
            target_count: row.get(4)?,
            icon_id: row.get(5)?,
            localize: row.get(6)?,
            valid_contexts: row.get(7)?,
            loc_status: row.get(8)?,
            gate_version: trim_and_nullify(row.get(9)?),
        })
    }
}

impl HasKey for Preconditions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PropertyEntranceComponent {
    pub id: i32,
    pub map_id: i32,
    pub property_name: String,
    pub is_on_property: bool,
    pub group_type: Option<String>,
}

impl FromCdClient for PropertyEntranceComponent {
    const TABLE: &'static str = "PropertyEntranceComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            map_id: row.get(1)?,
            property_name: trim_to_string(row.get(2)?),
            is_on_property: row.get(3)?,
            group_type: trim_and_nullify(row.get(4)?),
        })
    }
}

impl HasKey for PropertyEntranceComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PropertyTemplate {
    pub id: i32,
    pub map_id: i32,
    pub vendor_map_id: i32,
    pub spawn_name: String,
    pub r#type: i32,
    pub sizecode: i32,
    pub minimum_price: i32,
    pub rent_duration: i32,
    pub path: Option<String>,
    pub clone_limit: i32,
    pub duration_type: i32,
    pub achievement_required: i32,
    pub zone_x: f64,
    pub zone_y: f64,
    pub zone_z: f64,
    pub max_build_height: f64,
    pub localize: bool,
    pub reputation_per_minute: i32,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for PropertyTemplate {
    const TABLE: &'static str = "PropertyTemplate";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            map_id: row.get(1)?,
            vendor_map_id: row.get(2)?,
            spawn_name: trim_to_string(row.get(3)?),
            r#type: row.get(4)?,
            sizecode: row.get(5)?,
            minimum_price: row.get(6)?,
            rent_duration: row.get(7)?,
            path: trim_and_nullify(row.get(8)?),
            clone_limit: row.get(9)?,
            duration_type: row.get(10)?,
            achievement_required: row.get(11)?,
            zone_x: row.get(12)?,
            zone_y: row.get(13)?,
            zone_z: row.get(14)?,
            max_build_height: row.get(15)?,
            localize: row.get(16)?,
            reputation_per_minute: row.get(17)?,
            loc_status: row.get(18)?,
            gate_version: trim_and_nullify(row.get(19)?),
        })
    }
}

impl HasKey for PropertyTemplate {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProximityMonitorComponent {
    pub id: i32,
    pub proximities: Vec<i32>,
    pub load_on_client: bool,
    pub load_on_server: bool,
}

impl FromCdClient for ProximityMonitorComponent {
    const TABLE: &'static str = "ProximityMonitorComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            proximities: parse_required_comma_list(row.get(1)?),
            load_on_client: row.get(2)?,
            load_on_server: row.get(3)?,
        })
    }
}

impl HasKey for ProximityMonitorComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProximityTypes {
    pub id: i32,
    pub name: String,
    pub radius: i32,
    pub collision_group: i32,
    pub passive_checks: bool,
    pub icon_id: i32,
    pub load_on_client: bool,
    pub load_on_server: bool,
}

impl FromCdClient for ProximityTypes {
    const TABLE: &'static str = "ProximityTypes";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: trim_to_string(row.get(1)?),
            radius: row.get(2)?,
            collision_group: row.get(3)?,
            passive_checks: row.get(4)?,
            icon_id: row.get(5)?,
            load_on_client: row.get(6)?,
            load_on_server: row.get(7)?,
        })
    }
}

impl HasKey for ProximityTypes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RacingModuleComponent {
    pub id: i32,
    pub top_speed: Option<f64>,
    pub acceleration: Option<f64>,
    pub handling: Option<f64>,
    pub stability: Option<f64>,
    pub imagination: Option<f64>,
}

impl FromCdClient for RacingModuleComponent {
    const TABLE: &'static str = "RacingModuleComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            top_speed: row.get(1)?,
            acceleration: row.get(2)?,
            handling: row.get(3)?,
            stability: row.get(4)?,
            imagination: row.get(5)?,
        })
    }
}

impl HasKey for RacingModuleComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RailActivatorComponent {
    pub id: i32,
    pub start_anim: String,
    pub loop_anim: Option<String>,
    pub stop_anim: String,
    pub start_sound: Option<String>,
    pub loop_sound: Option<String>,
    pub stop_sound: Option<String>,
    pub effect_ids: Option<String>,
    pub preconditions: Option<String>,
    pub player_collision: bool,
    pub camera_locked: bool,
    pub start_effect_id: Option<String>,
    pub stop_effect_id: Option<String>,
    pub damage_immune: bool,
    pub no_aggro: bool,
    pub show_name_billboard: bool,
}

impl FromCdClient for RailActivatorComponent {
    const TABLE: &'static str = "RailActivatorComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            start_anim: trim_to_string(row.get(1)?),
            loop_anim: trim_and_nullify(row.get(2)?),
            stop_anim: trim_to_string(row.get(3)?),
            start_sound: trim_and_nullify(row.get(4)?),
            loop_sound: trim_and_nullify(row.get(5)?),
            stop_sound: trim_and_nullify(row.get(6)?),
            effect_ids: trim_and_nullify(row.get(7)?),
            preconditions: trim_and_nullify(row.get(8)?),
            player_collision: row.get(9)?,
            camera_locked: row.get(10)?,
            start_effect_id: trim_and_nullify(row.get(11)?),
            stop_effect_id: trim_and_nullify(row.get(12)?),
            damage_immune: row.get(13)?,
            no_aggro: row.get(14)?,
            show_name_billboard: row.get(15)?,
        })
    }
}

impl HasKey for RailActivatorComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RarityTable {
    pub id: i32,
    pub randmax: f64,
    pub rarity: i32,
    pub rarity_table_index: i32,
}

impl FromCdClient for RarityTable {
    const TABLE: &'static str = "RarityTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            randmax: row.get(1)?,
            rarity: row.get(2)?,
            rarity_table_index: row.get(3)?,
        })
    }
}

impl HasKey for RarityTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RarityTableIndex {
    pub rarity_table_index: i32,
}

impl FromCdClient for RarityTableIndex {
    const TABLE: &'static str = "RarityTableIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            rarity_table_index: row.get(0)?,
        })
    }
}

impl HasKey for RarityTableIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.rarity_table_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RebuildComponent {
    pub id: i32,
    pub reset_time: f64,
    pub complete_time: Option<f64>,
    pub take_imagination: i32,
    pub interruptible: bool,
    pub self_activator: bool,
    pub custom_modules: Option<Vec<i32>>,
    pub activity_id: Option<i32>,
    pub post_imagination_cost: Option<i32>,
    pub time_before_smash: f64,
}

impl FromCdClient for RebuildComponent {
    const TABLE: &'static str = "RebuildComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            reset_time: row.get(1)?,
            complete_time: row.get(2)?,
            take_imagination: row.get(3)?,
            interruptible: row.get(4)?,
            self_activator: row.get(5)?,
            custom_modules: parse_optional_comma_list(row.get(6)?),
            activity_id: row.get(7)?,
            post_imagination_cost: row.get(8)?,
            time_before_smash: row.get(9)?,
        })
    }
}

impl HasKey for RebuildComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RebuildSections {
    pub id: i32,
    pub rebuild_id: i32,
    pub object_id: i32,
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
    pub fall_angle_x: Option<f64>,
    pub fall_angle_y: Option<f64>,
    pub fall_angle_z: Option<f64>,
    pub fall_height: Option<f64>,
    pub requires_list: Option<Vec<i32>>,
    pub size: i32,
    pub b_placed: bool,
}

impl FromCdClient for RebuildSections {
    const TABLE: &'static str = "RebuildSections";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            rebuild_id: row.get(1)?,
            object_id: row.get(2)?,
            offset_x: row.get(3)?,
            offset_y: row.get(4)?,
            offset_z: row.get(5)?,
            fall_angle_x: row.get(6)?,
            fall_angle_y: row.get(7)?,
            fall_angle_z: row.get(8)?,
            fall_height: row.get(9)?,
            requires_list: parse_optional_comma_list(row.get(10)?),
            size: row.get(11)?,
            b_placed: row.get(12)?,
        })
    }
}

impl HasKey for RebuildSections {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ReleaseVersion {
    pub release_version: String,
    pub release_date: i64,
}

impl FromCdClient for ReleaseVersion {
    const TABLE: &'static str = "Release_Version";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            release_version: trim_to_string(row.get(0)?),
            release_date: row.get(1)?,
        })
    }
}

impl HasKey for ReleaseVersion {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.release_version
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RenderComponent {
    pub id: i32,
    pub render_asset: Option<String>,
    pub icon_asset: Option<String>,
    pub icon_id: Option<i32>,
    pub shader_id: Option<i32>,
    pub effect1: Option<i32>,
    pub effect2: Option<i32>,
    pub effect3: Option<i32>,
    pub effect4: Option<i32>,
    pub effect5: Option<i32>,
    pub effect6: Option<i32>,
    pub animation_group_ids: Option<Vec<i32>>,
    pub fade: Option<bool>,
    pub usedropshadow: Option<bool>,
    pub preload_animations: Option<bool>,
    pub fade_in_time: Option<f64>,
    pub max_shadow_distance: Option<f64>,
    pub ignore_camera_collision: Option<bool>,
    pub render_component_lod1: Option<i32>,
    pub render_component_lod2: Option<i32>,
    pub gradual_snap: Option<bool>,
    pub animation_flag: Option<i32>,
    pub audio_meta_event_set: Option<String>,
    pub billboard_height: Option<f64>,
    pub chat_bubble_offset: Option<f64>,
    pub static_billboard: Option<bool>,
    pub lxfmlfolder: Option<String>,
    pub attach_indicators_to_node: Option<bool>,
}

impl FromCdClient for RenderComponent {
    const TABLE: &'static str = "RenderComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            render_asset: trim_and_nullify(row.get(1)?),
            icon_asset: trim_and_nullify(row.get(2)?),
            icon_id: row.get(3)?,
            shader_id: row.get(4)?,
            effect1: row.get(5)?,
            effect2: row.get(6)?,
            effect3: row.get(7)?,
            effect4: row.get(8)?,
            effect5: row.get(9)?,
            effect6: row.get(10)?,
            animation_group_ids: parse_optional_comma_list(row.get(11)?),
            fade: row.get(12)?,
            usedropshadow: row.get(13)?,
            preload_animations: row.get(14)?,
            fade_in_time: row.get(15)?,
            max_shadow_distance: row.get(16)?,
            ignore_camera_collision: row.get(17)?,
            render_component_lod1: row.get(18)?,
            render_component_lod2: row.get(19)?,
            gradual_snap: row.get(20)?,
            animation_flag: row.get(21)?,
            audio_meta_event_set: trim_and_nullify(row.get(22)?),
            billboard_height: row.get(23)?,
            chat_bubble_offset: row.get(24)?,
            static_billboard: row.get(25)?,
            lxfmlfolder: trim_and_nullify(row.get(26)?),
            attach_indicators_to_node: row.get(27)?,
        })
    }
}

impl HasKey for RenderComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RenderComponentFlash {
    pub id: i32,
    pub interactive: bool,
    pub animated: bool,
    pub node_name: String,
    pub flash_path: String,
    pub element_name: Option<String>,
    pub uid: i32,
}

impl FromCdClient for RenderComponentFlash {
    const TABLE: &'static str = "RenderComponentFlash";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            interactive: row.get(1)?,
            animated: row.get(2)?,
            node_name: trim_to_string(row.get(3)?),
            flash_path: trim_to_string(row.get(4)?),
            element_name: trim_and_nullify(row.get(5)?),
            uid: row.get(6)?,
        })
    }
}

impl HasGroupKey for RenderComponentFlash {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RenderComponentWrapper {
    pub id: i32,
    pub default_wrapper_asset: String,
}

impl FromCdClient for RenderComponentWrapper {
    const TABLE: &'static str = "RenderComponentWrapper";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            default_wrapper_asset: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for RenderComponentWrapper {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RenderIconAssets {
    pub id: i32,
    pub icon_asset: Option<String>,
    pub blank_column: Option<String>,
}

impl FromCdClient for RenderIconAssets {
    const TABLE: &'static str = "RenderIconAssets";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            icon_asset: trim_and_nullify(row.get(1)?),
            blank_column: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for RenderIconAssets {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ReputationRewards {
    pub rep_level: i32,
    pub sublevel: i32,
    pub reputation: f64,
}

impl FromCdClient for ReputationRewards {
    const TABLE: &'static str = "ReputationRewards";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            rep_level: row.get(0)?,
            sublevel: row.get(1)?,
            reputation: row.get(2)?,
        })
    }
}

impl HasKey for ReputationRewards {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.rep_level
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RewardCodes {
    pub id: i32,
    pub code: String,
    pub attachment_lot: Option<i32>,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for RewardCodes {
    const TABLE: &'static str = "RewardCodes";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            code: trim_to_string(row.get(1)?),
            attachment_lot: row.get(2)?,
            loc_status: row.get(3)?,
            gate_version: trim_and_nullify(row.get(4)?),
        })
    }
}

impl HasKey for RewardCodes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Rewards {
    pub id: i32,
    pub level_id: i32,
    pub mission_id: Option<i32>,
    pub reward_type: i32,
    pub value: i32,
    pub count: Option<i32>,
}

impl FromCdClient for Rewards {
    const TABLE: &'static str = "Rewards";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            level_id: row.get(1)?,
            mission_id: row.get(2)?,
            reward_type: row.get(3)?,
            value: row.get(4)?,
            count: row.get(5)?,
        })
    }
}

impl HasKey for Rewards {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RocketLaunchpadControlComponent {
    pub id: i32,
    pub target_zone: i32,
    pub default_zone_id: i32,
    pub target_scene: Option<String>,
    pub gm_level: i32,
    pub player_animation: String,
    pub rocket_animation: String,
    pub launch_music: Option<String>,
    pub use_launch_precondition: bool,
    pub use_alt_landing_precondition: bool,
    pub launch_precondition: Option<String>,
    pub alt_landing_precondition: Option<String>,
    pub alt_landing_spawn_point_name: Option<String>,
}

impl FromCdClient for RocketLaunchpadControlComponent {
    const TABLE: &'static str = "RocketLaunchpadControlComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            target_zone: row.get(1)?,
            default_zone_id: row.get(2)?,
            target_scene: trim_and_nullify(row.get(3)?),
            gm_level: row.get(4)?,
            player_animation: trim_to_string(row.get(5)?),
            rocket_animation: trim_to_string(row.get(6)?),
            launch_music: trim_and_nullify(row.get(7)?),
            use_launch_precondition: row.get(8)?,
            use_alt_landing_precondition: row.get(9)?,
            launch_precondition: trim_and_nullify(row.get(10)?),
            alt_landing_precondition: trim_and_nullify(row.get(11)?),
            alt_landing_spawn_point_name: trim_and_nullify(row.get(12)?),
        })
    }
}

impl HasKey for RocketLaunchpadControlComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SceneTable {
    pub scene_id: i32,
    pub scene_name: String,
}

impl FromCdClient for SceneTable {
    const TABLE: &'static str = "SceneTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            scene_id: row.get(0)?,
            scene_name: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for SceneTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.scene_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ScriptComponent {
    pub id: i32,
    pub script_name: Option<String>,
    pub client_script_name: Option<String>,
}

impl FromCdClient for ScriptComponent {
    const TABLE: &'static str = "ScriptComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            script_name: trim_and_nullify(row.get(1)?),
            client_script_name: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for ScriptComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SkillBehavior {
    pub skill_id: i32,
    pub loc_status: i32,
    pub behavior_id: i32,
    pub imaginationcost: i32,
    pub cooldowngroup: Option<i32>,
    pub cooldown: Option<f64>,
    pub in_npc_editor: bool,
    pub skill_icon: Option<i32>,
    pub oom_skill_id: Option<Vec<i32>>,
    pub oom_behavior_effect_id: Option<i32>,
    pub cast_type_desc: Option<i32>,
    pub im_bonus_ui: Option<i32>,
    pub life_bonus_ui: Option<i32>,
    pub armor_bonus_ui: Option<i32>,
    pub damage_ui: Option<i32>,
    pub hide_icon: bool,
    pub localize: bool,
    pub gate_version: Option<String>,
    pub cancel_type: Option<i32>,
}

impl FromCdClient for SkillBehavior {
    const TABLE: &'static str = "SkillBehavior";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            skill_id: row.get(0)?,
            loc_status: row.get(1)?,
            behavior_id: row.get(2)?,
            imaginationcost: row.get(3)?,
            cooldowngroup: row.get(4)?,
            cooldown: row.get(5)?,
            in_npc_editor: row.get(6)?,
            skill_icon: row.get(7)?,
            oom_skill_id: parse_optional_comma_list(row.get(8)?),
            oom_behavior_effect_id: row.get(9)?,
            cast_type_desc: row.get(10)?,
            im_bonus_ui: row.get(11)?,
            life_bonus_ui: row.get(12)?,
            armor_bonus_ui: row.get(13)?,
            damage_ui: row.get(14)?,
            hide_icon: row.get(15)?,
            localize: row.get(16)?,
            gate_version: trim_and_nullify(row.get(17)?),
            cancel_type: row.get(18)?,
        })
    }
}

impl HasKey for SkillBehavior {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.skill_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SmashableChain {
    pub chain_index: i32,
    pub chain_level: i32,
    pub loot_matrix_id: i32,
    pub rarity_table_index: i32,
    pub currency_index: i32,
    pub currency_level: i32,
    pub smash_count: i32,
    pub time_limit: i32,
    pub chain_step_id: i32,
}

impl FromCdClient for SmashableChain {
    const TABLE: &'static str = "SmashableChain";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            chain_index: row.get(0)?,
            chain_level: row.get(1)?,
            loot_matrix_id: row.get(2)?,
            rarity_table_index: row.get(3)?,
            currency_index: row.get(4)?,
            currency_level: row.get(5)?,
            smash_count: row.get(6)?,
            time_limit: row.get(7)?,
            chain_step_id: row.get(8)?,
        })
    }
}

impl HasGroupKey for SmashableChain {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.chain_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SmashableChainIndex {
    pub id: i32,
    pub target_group: String,
    pub description: String,
    pub continuous: i32,
}

impl FromCdClient for SmashableChainIndex {
    const TABLE: &'static str = "SmashableChainIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            target_group: trim_to_string(row.get(1)?),
            description: trim_to_string(row.get(2)?),
            continuous: row.get(3)?,
        })
    }
}

impl HasKey for SmashableChainIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SmashableComponent {
    pub id: i32,
    pub loot_matrix_index: i32,
}

impl FromCdClient for SmashableComponent {
    const TABLE: &'static str = "SmashableComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            loot_matrix_index: row.get(1)?,
        })
    }
}

impl HasKey for SmashableComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SmashableElements {
    pub element_id: i32,
    pub drop_weight: i32,
}

impl FromCdClient for SmashableElements {
    const TABLE: &'static str = "SmashableElements";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            element_id: row.get(0)?,
            drop_weight: row.get(1)?,
        })
    }
}

impl HasKey for SmashableElements {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.element_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SpeedchatMenu {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub emote_id: Option<i32>,
    pub image_name: Option<String>,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for SpeedchatMenu {
    const TABLE: &'static str = "SpeedchatMenu";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            emote_id: row.get(2)?,
            image_name: trim_and_nullify(row.get(3)?),
            localize: row.get(4)?,
            loc_status: row.get(5)?,
            gate_version: trim_and_nullify(row.get(6)?),
        })
    }
}

impl HasKey for SpeedchatMenu {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SubscriptionPricing {
    pub id: i32,
    pub country_code: String,
    pub monthly_fee_gold: String,
    pub monthly_fee_silver: String,
    pub monthly_fee_bronze: String,
    pub monetary_symbol: i32,
    pub symbol_is_appended: bool,
}

impl FromCdClient for SubscriptionPricing {
    const TABLE: &'static str = "SubscriptionPricing";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            country_code: trim_to_string(row.get(1)?),
            monthly_fee_gold: trim_to_string(row.get(2)?),
            monthly_fee_silver: trim_to_string(row.get(3)?),
            monthly_fee_bronze: trim_to_string(row.get(4)?),
            monetary_symbol: row.get(5)?,
            symbol_is_appended: row.get(6)?,
        })
    }
}

impl HasKey for SubscriptionPricing {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SurfaceType {
    pub surface_type: i32,
    pub footstep_ndaudio_meta_event_set_name: Option<String>,
}

impl FromCdClient for SurfaceType {
    const TABLE: &'static str = "SurfaceType";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            surface_type: row.get(0)?,
            footstep_ndaudio_meta_event_set_name: trim_and_nullify(row.get(1)?),
        })
    }
}

impl HasKey for SurfaceType {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.surface_type
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TamingBuildPuzzles {
    pub id: i32,
    pub puzzle_model_lot: i32,
    pub npclot: i32,
    pub valid_pieces_lxf: String,
    pub invalid_pieces_lxf: String,
    pub difficulty: i32,
    pub timelimit: i32,
    pub num_valid_pieces: i32,
    pub total_num_pieces: i32,
    pub model_name: String,
    pub full_model_lxf: String,
    pub duration: f64,
    pub imag_cost_per_build: i32,
}

impl FromCdClient for TamingBuildPuzzles {
    const TABLE: &'static str = "TamingBuildPuzzles";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            puzzle_model_lot: row.get(1)?,
            npclot: row.get(2)?,
            valid_pieces_lxf: trim_to_string(row.get(3)?),
            invalid_pieces_lxf: trim_to_string(row.get(4)?),
            difficulty: row.get(5)?,
            timelimit: row.get(6)?,
            num_valid_pieces: row.get(7)?,
            total_num_pieces: row.get(8)?,
            model_name: trim_to_string(row.get(9)?),
            full_model_lxf: trim_to_string(row.get(10)?),
            duration: row.get(11)?,
            imag_cost_per_build: row.get(12)?,
        })
    }
}

impl HasKey for TamingBuildPuzzles {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TextDescription {
    pub text_id: i32,
    pub test_description: String,
}

impl FromCdClient for TextDescription {
    const TABLE: &'static str = "TextDescription";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            text_id: row.get(0)?,
            test_description: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for TextDescription {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.text_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TextLanguage {
    pub text_id: i32,
    pub language_id: i32,
    pub text: String,
}

impl FromCdClient for TextLanguage {
    const TABLE: &'static str = "TextLanguage";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            text_id: row.get(0)?,
            language_id: row.get(1)?,
            text: trim_to_string(row.get(2)?),
        })
    }
}

impl HasKey for TextLanguage {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.text_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TrailEffects {
    pub trail_id: i32,
    pub texture_name: String,
    pub blendmode: i32,
    pub cardlifetime: f64,
    pub colorlifetime: f64,
    pub min_tail_fade: f64,
    pub tail_fade: f64,
    pub max_particles: i32,
    pub birth_delay: f64,
    pub death_delay: f64,
    pub bone1: String,
    pub bone2: String,
    pub tex_length: f64,
    pub tex_width: f64,
    pub start_color_r: f64,
    pub start_color_g: f64,
    pub start_color_b: f64,
    pub start_color_a: f64,
    pub middle_color_r: f64,
    pub middle_color_g: f64,
    pub middle_color_b: f64,
    pub middle_color_a: f64,
    pub end_color_r: f64,
    pub end_color_g: f64,
    pub end_color_b: f64,
    pub end_color_a: f64,
}

impl FromCdClient for TrailEffects {
    const TABLE: &'static str = "TrailEffects";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            trail_id: row.get(0)?,
            texture_name: trim_to_string(row.get(1)?),
            blendmode: row.get(2)?,
            cardlifetime: row.get(3)?,
            colorlifetime: row.get(4)?,
            min_tail_fade: row.get(5)?,
            tail_fade: row.get(6)?,
            max_particles: row.get(7)?,
            birth_delay: row.get(8)?,
            death_delay: row.get(9)?,
            bone1: trim_to_string(row.get(10)?),
            bone2: trim_to_string(row.get(11)?),
            tex_length: row.get(12)?,
            tex_width: row.get(13)?,
            start_color_r: row.get(14)?,
            start_color_g: row.get(15)?,
            start_color_b: row.get(16)?,
            start_color_a: row.get(17)?,
            middle_color_r: row.get(18)?,
            middle_color_g: row.get(19)?,
            middle_color_b: row.get(20)?,
            middle_color_a: row.get(21)?,
            end_color_r: row.get(22)?,
            end_color_g: row.get(23)?,
            end_color_b: row.get(24)?,
            end_color_a: row.get(25)?,
        })
    }
}

impl HasKey for TrailEffects {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.trail_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct UgBehaviorSounds {
    pub id: i32,
    pub guid: String,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for UgBehaviorSounds {
    const TABLE: &'static str = "UGBehaviorSounds";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            guid: trim_to_string(row.get(1)?),
            localize: row.get(2)?,
            loc_status: row.get(3)?,
            gate_version: trim_and_nullify(row.get(4)?),
        })
    }
}

impl HasKey for UgBehaviorSounds {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct VehiclePhysics {
    pub id: i32,
    pub hkx_filename: Option<String>,
    pub f_gravity_scale: f64,
    pub f_mass: f64,
    pub f_chassis_friction: f64,
    pub f_max_speed: f64,
    pub f_engine_torque: f64,
    pub f_brake_front_torque: f64,
    pub f_brake_rear_torque: f64,
    pub f_brake_min_input_to_block: f64,
    pub f_brake_min_time_to_block: f64,
    pub f_steering_max_angle: f64,
    pub f_steering_speed_limit_for_max_angle: f64,
    pub f_steering_min_angle: f64,
    pub f_fwd_bias: f64,
    pub f_front_tire_friction: f64,
    pub f_rear_tire_friction: f64,
    pub f_front_tire_friction_slide: f64,
    pub f_rear_tire_friction_slide: f64,
    pub f_front_tire_slip_angle: f64,
    pub f_rear_tire_slip_angle: f64,
    pub f_wheel_width: f64,
    pub f_wheel_radius: f64,
    pub f_wheel_mass: f64,
    pub f_reorient_pitch_strength: f64,
    pub f_reorient_roll_strength: f64,
    pub f_suspension_length: f64,
    pub f_suspension_strength: f64,
    pub f_suspension_damping_compression: f64,
    pub f_suspension_damping_relaxation: f64,
    pub i_chassis_collision_group: i32,
    pub f_normal_spin_damping: f64,
    pub f_collision_spin_damping: f64,
    pub f_collision_threshold: f64,
    pub f_torque_roll_factor: f64,
    pub f_torque_pitch_factor: f64,
    pub f_torque_yaw_factor: f64,
    pub f_inertia_roll: f64,
    pub f_inertia_pitch: f64,
    pub f_inertia_yaw: f64,
    pub f_extra_torque_factor: f64,
    pub f_center_of_mass_fwd: f64,
    pub f_center_of_mass_up: f64,
    pub f_center_of_mass_right: f64,
    pub f_wheel_hardpoint_front_fwd: f64,
    pub f_wheel_hardpoint_front_up: f64,
    pub f_wheel_hardpoint_front_right: f64,
    pub f_wheel_hardpoint_rear_fwd: f64,
    pub f_wheel_hardpoint_rear_up: f64,
    pub f_wheel_hardpoint_rear_right: f64,
    pub f_input_turn_speed: f64,
    pub f_input_dead_turn_back_speed: f64,
    pub f_input_accel_speed: f64,
    pub f_input_dead_accel_down_speed: f64,
    pub f_input_decel_speed: f64,
    pub f_input_dead_decel_down_speed: f64,
    pub f_input_slope_change_point_x: f64,
    pub f_input_initial_slope: f64,
    pub f_input_dead_zone: f64,
    pub f_aero_air_density: f64,
    pub f_aero_frontal_area: f64,
    pub f_aero_drag_coefficient: f64,
    pub f_aero_lift_coefficient: f64,
    pub f_aero_extra_gravity: f64,
    pub f_boost_top_speed: f64,
    pub f_boost_cost_per_second: f64,
    pub f_boost_accelerate_change: f64,
    pub f_boost_damping_change: f64,
    pub f_powerslide_neutral_angle: f64,
    pub f_powerslide_torque_strength: f64,
    pub i_powerslide_num_torque_applications: i32,
    pub f_imagination_tank_size: f64,
    pub f_skill_cost: f64,
    pub f_wreck_speed_base: Option<f64>,
    pub f_wreck_speed_percent: Option<f64>,
    pub f_wreck_min_angle: Option<f64>,
    pub audio_event_engine: Option<String>,
    pub audio_event_skid: Option<String>,
    pub audio_event_light_hit: Option<String>,
    pub audio_speed_threshold_light_hit: f64,
    pub audio_timeout_light_hit: f64,
    pub audio_event_heavy_hit: Option<String>,
    pub audio_speed_threshold_heavy_hit: f64,
    pub audio_timeout_heavy_hit: f64,
    pub audio_event_start: Option<String>,
    pub audio_event_tread_concrete: Option<String>,
    pub audio_event_tread_sand: Option<String>,
    pub audio_event_tread_wood: Option<String>,
    pub audio_event_tread_dirt: Option<String>,
    pub audio_event_tread_plastic: Option<String>,
    pub audio_event_tread_grass: Option<String>,
    pub audio_event_tread_gravel: Option<String>,
    pub audio_event_tread_mud: Option<String>,
    pub audio_event_tread_water: Option<String>,
    pub audio_event_tread_snow: Option<String>,
    pub audio_event_tread_ice: Option<String>,
    pub audio_event_tread_metal: Option<String>,
    pub audio_event_tread_leaves: Option<String>,
    pub audio_event_light_land: Option<String>,
    pub audio_airtime_for_light_land: f64,
    pub audio_event_heavy_land: Option<String>,
    pub audio_airtime_for_heavy_land: f64,
    pub b_wheels_visible: Option<bool>,
}

impl FromCdClient for VehiclePhysics {
    const TABLE: &'static str = "VehiclePhysics";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            hkx_filename: trim_and_nullify(row.get(1)?),
            f_gravity_scale: row.get(2)?,
            f_mass: row.get(3)?,
            f_chassis_friction: row.get(4)?,
            f_max_speed: row.get(5)?,
            f_engine_torque: row.get(6)?,
            f_brake_front_torque: row.get(7)?,
            f_brake_rear_torque: row.get(8)?,
            f_brake_min_input_to_block: row.get(9)?,
            f_brake_min_time_to_block: row.get(10)?,
            f_steering_max_angle: row.get(11)?,
            f_steering_speed_limit_for_max_angle: row.get(12)?,
            f_steering_min_angle: row.get(13)?,
            f_fwd_bias: row.get(14)?,
            f_front_tire_friction: row.get(15)?,
            f_rear_tire_friction: row.get(16)?,
            f_front_tire_friction_slide: row.get(17)?,
            f_rear_tire_friction_slide: row.get(18)?,
            f_front_tire_slip_angle: row.get(19)?,
            f_rear_tire_slip_angle: row.get(20)?,
            f_wheel_width: row.get(21)?,
            f_wheel_radius: row.get(22)?,
            f_wheel_mass: row.get(23)?,
            f_reorient_pitch_strength: row.get(24)?,
            f_reorient_roll_strength: row.get(25)?,
            f_suspension_length: row.get(26)?,
            f_suspension_strength: row.get(27)?,
            f_suspension_damping_compression: row.get(28)?,
            f_suspension_damping_relaxation: row.get(29)?,
            i_chassis_collision_group: row.get(30)?,
            f_normal_spin_damping: row.get(31)?,
            f_collision_spin_damping: row.get(32)?,
            f_collision_threshold: row.get(33)?,
            f_torque_roll_factor: row.get(34)?,
            f_torque_pitch_factor: row.get(35)?,
            f_torque_yaw_factor: row.get(36)?,
            f_inertia_roll: row.get(37)?,
            f_inertia_pitch: row.get(38)?,
            f_inertia_yaw: row.get(39)?,
            f_extra_torque_factor: row.get(40)?,
            f_center_of_mass_fwd: row.get(41)?,
            f_center_of_mass_up: row.get(42)?,
            f_center_of_mass_right: row.get(43)?,
            f_wheel_hardpoint_front_fwd: row.get(44)?,
            f_wheel_hardpoint_front_up: row.get(45)?,
            f_wheel_hardpoint_front_right: row.get(46)?,
            f_wheel_hardpoint_rear_fwd: row.get(47)?,
            f_wheel_hardpoint_rear_up: row.get(48)?,
            f_wheel_hardpoint_rear_right: row.get(49)?,
            f_input_turn_speed: row.get(50)?,
            f_input_dead_turn_back_speed: row.get(51)?,
            f_input_accel_speed: row.get(52)?,
            f_input_dead_accel_down_speed: row.get(53)?,
            f_input_decel_speed: row.get(54)?,
            f_input_dead_decel_down_speed: row.get(55)?,
            f_input_slope_change_point_x: row.get(56)?,
            f_input_initial_slope: row.get(57)?,
            f_input_dead_zone: row.get(58)?,
            f_aero_air_density: row.get(59)?,
            f_aero_frontal_area: row.get(60)?,
            f_aero_drag_coefficient: row.get(61)?,
            f_aero_lift_coefficient: row.get(62)?,
            f_aero_extra_gravity: row.get(63)?,
            f_boost_top_speed: row.get(64)?,
            f_boost_cost_per_second: row.get(65)?,
            f_boost_accelerate_change: row.get(66)?,
            f_boost_damping_change: row.get(67)?,
            f_powerslide_neutral_angle: row.get(68)?,
            f_powerslide_torque_strength: row.get(69)?,
            i_powerslide_num_torque_applications: row.get(70)?,
            f_imagination_tank_size: row.get(71)?,
            f_skill_cost: row.get(72)?,
            f_wreck_speed_base: row.get(73)?,
            f_wreck_speed_percent: row.get(74)?,
            f_wreck_min_angle: row.get(75)?,
            audio_event_engine: trim_and_nullify(row.get(76)?),
            audio_event_skid: trim_and_nullify(row.get(77)?),
            audio_event_light_hit: trim_and_nullify(row.get(78)?),
            audio_speed_threshold_light_hit: row.get(79)?,
            audio_timeout_light_hit: row.get(80)?,
            audio_event_heavy_hit: trim_and_nullify(row.get(81)?),
            audio_speed_threshold_heavy_hit: row.get(82)?,
            audio_timeout_heavy_hit: row.get(83)?,
            audio_event_start: trim_and_nullify(row.get(84)?),
            audio_event_tread_concrete: trim_and_nullify(row.get(85)?),
            audio_event_tread_sand: trim_and_nullify(row.get(86)?),
            audio_event_tread_wood: trim_and_nullify(row.get(87)?),
            audio_event_tread_dirt: trim_and_nullify(row.get(88)?),
            audio_event_tread_plastic: trim_and_nullify(row.get(89)?),
            audio_event_tread_grass: trim_and_nullify(row.get(90)?),
            audio_event_tread_gravel: trim_and_nullify(row.get(91)?),
            audio_event_tread_mud: trim_and_nullify(row.get(92)?),
            audio_event_tread_water: trim_and_nullify(row.get(93)?),
            audio_event_tread_snow: trim_and_nullify(row.get(94)?),
            audio_event_tread_ice: trim_and_nullify(row.get(95)?),
            audio_event_tread_metal: trim_and_nullify(row.get(96)?),
            audio_event_tread_leaves: trim_and_nullify(row.get(97)?),
            audio_event_light_land: trim_and_nullify(row.get(98)?),
            audio_airtime_for_light_land: row.get(99)?,
            audio_event_heavy_land: trim_and_nullify(row.get(100)?),
            audio_airtime_for_heavy_land: row.get(101)?,
            b_wheels_visible: row.get(102)?,
        })
    }
}

impl HasKey for VehiclePhysics {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct VehicleStatMap {
    pub id: i32,
    pub module_stat: String,
    pub havok_stat: String,
    pub havok_change_per_module_stat: f64,
}

impl FromCdClient for VehicleStatMap {
    const TABLE: &'static str = "VehicleStatMap";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            module_stat: trim_to_string(row.get(1)?),
            havok_stat: trim_to_string(row.get(2)?),
            havok_change_per_module_stat: row.get(3)?,
        })
    }
}

impl HasGroupKey for VehicleStatMap {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct VendorComponent {
    pub id: i32,
    pub buy_scalar: f64,
    pub sell_scalar: f64,
    pub refresh_time_seconds: f64,
    pub loot_matrix_index: i32,
}

impl FromCdClient for VendorComponent {
    const TABLE: &'static str = "VendorComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            buy_scalar: row.get(1)?,
            sell_scalar: row.get(2)?,
            refresh_time_seconds: row.get(3)?,
            loot_matrix_index: row.get(4)?,
        })
    }
}

impl HasKey for VendorComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WhatsCoolItemSpotlight {
    pub id: i32,
    pub item_id: i32,
    pub localize: bool,
    pub gate_version: Option<String>,
    pub loc_status: i32,
}

impl FromCdClient for WhatsCoolItemSpotlight {
    const TABLE: &'static str = "WhatsCoolItemSpotlight";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            item_id: row.get(1)?,
            localize: row.get(2)?,
            gate_version: trim_and_nullify(row.get(3)?),
            loc_status: row.get(4)?,
        })
    }
}

impl HasKey for WhatsCoolItemSpotlight {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WhatsCoolNewsAndTips {
    pub id: i32,
    pub icon_id: Option<i32>,
    pub r#type: i32,
    pub localize: bool,
    pub gate_version: Option<String>,
    pub loc_status: i32,
}

impl FromCdClient for WhatsCoolNewsAndTips {
    const TABLE: &'static str = "WhatsCoolNewsAndTips";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            icon_id: row.get(1)?,
            r#type: row.get(2)?,
            localize: row.get(3)?,
            gate_version: trim_and_nullify(row.get(4)?),
            loc_status: row.get(5)?,
        })
    }
}

impl HasKey for WhatsCoolNewsAndTips {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WorldConfig {
    pub world_config_id: i32,
    pub pegravityvalue: f64,
    pub pebroadphaseworldsize: f64,
    pub pegameobjscalefactor: f64,
    pub character_rotation_speed: f64,
    pub character_walk_forward_speed: f64,
    pub character_walk_backward_speed: f64,
    pub character_walk_strafe_speed: f64,
    pub character_walk_strafe_forward_speed: f64,
    pub character_walk_strafe_backward_speed: f64,
    pub character_run_backward_speed: f64,
    pub character_run_strafe_speed: f64,
    pub character_run_strafe_forward_speed: f64,
    pub character_run_strafe_backward_speed: f64,
    pub global_cooldown: f64,
    pub character_grounded_time: f64,
    pub character_grounded_speed: f64,
    pub global_immunity_time: f64,
    pub character_max_slope: f64,
    pub defaultrespawntime: f64,
    pub mission_tooltip_timeout: f64,
    pub vendor_buy_multiplier: f64,
    pub pet_follow_radius: f64,
    pub character_eye_height: f64,
    pub flight_vertical_velocity: f64,
    pub flight_airspeed: f64,
    pub flight_fuel_ratio: f64,
    pub flight_max_airspeed: f64,
    pub f_reputation_per_vote: f64,
    pub n_property_clone_limit: i32,
    pub default_homespace_template: i32,
    pub coins_lost_on_death_percent: f64,
    pub coins_lost_on_death_min: i32,
    pub coins_lost_on_death_max: i32,
    pub character_votes_per_day: i32,
    pub property_moderation_request_approval_cost: i32,
    pub property_moderation_request_review_cost: i32,
    pub property_mod_requests_allowed_spike: i32,
    pub property_mod_requests_allowed_interval: i32,
    pub property_mod_requests_allowed_total: i32,
    pub property_mod_requests_spike_duration: i32,
    pub property_mod_requests_interval_duration: i32,
    pub model_moderate_on_create: bool,
    pub default_property_max_height: f64,
    pub reputation_per_vote_cast: f64,
    pub reputation_per_vote_received: f64,
    pub showcase_top_model_consideration_battles: i32,
    pub reputation_per_battle_promotion: f64,
    pub coins_lost_on_death_min_timeout: f64,
    pub coins_lost_on_death_max_timeout: f64,
    pub mail_base_fee: i32,
    pub mail_percent_attachment_fee: f64,
    pub property_reputation_delay: i32,
    pub level_cap: i32,
    pub level_up_behavior_effect: String,
    pub character_version: i32,
    pub level_cap_currency_conversion: i32,
}

impl FromCdClient for WorldConfig {
    const TABLE: &'static str = "WorldConfig";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            world_config_id: row.get(0)?,
            pegravityvalue: row.get(1)?,
            pebroadphaseworldsize: row.get(2)?,
            pegameobjscalefactor: row.get(3)?,
            character_rotation_speed: row.get(4)?,
            character_walk_forward_speed: row.get(5)?,
            character_walk_backward_speed: row.get(6)?,
            character_walk_strafe_speed: row.get(7)?,
            character_walk_strafe_forward_speed: row.get(8)?,
            character_walk_strafe_backward_speed: row.get(9)?,
            character_run_backward_speed: row.get(10)?,
            character_run_strafe_speed: row.get(11)?,
            character_run_strafe_forward_speed: row.get(12)?,
            character_run_strafe_backward_speed: row.get(13)?,
            global_cooldown: row.get(14)?,
            character_grounded_time: row.get(15)?,
            character_grounded_speed: row.get(16)?,
            global_immunity_time: row.get(17)?,
            character_max_slope: row.get(18)?,
            defaultrespawntime: row.get(19)?,
            mission_tooltip_timeout: row.get(20)?,
            vendor_buy_multiplier: row.get(21)?,
            pet_follow_radius: row.get(22)?,
            character_eye_height: row.get(23)?,
            flight_vertical_velocity: row.get(24)?,
            flight_airspeed: row.get(25)?,
            flight_fuel_ratio: row.get(26)?,
            flight_max_airspeed: row.get(27)?,
            f_reputation_per_vote: row.get(28)?,
            n_property_clone_limit: row.get(29)?,
            default_homespace_template: row.get(30)?,
            coins_lost_on_death_percent: row.get(31)?,
            coins_lost_on_death_min: row.get(32)?,
            coins_lost_on_death_max: row.get(33)?,
            character_votes_per_day: row.get(34)?,
            property_moderation_request_approval_cost: row.get(35)?,
            property_moderation_request_review_cost: row.get(36)?,
            property_mod_requests_allowed_spike: row.get(37)?,
            property_mod_requests_allowed_interval: row.get(38)?,
            property_mod_requests_allowed_total: row.get(39)?,
            property_mod_requests_spike_duration: row.get(40)?,
            property_mod_requests_interval_duration: row.get(41)?,
            model_moderate_on_create: row.get(42)?,
            default_property_max_height: row.get(43)?,
            reputation_per_vote_cast: row.get(44)?,
            reputation_per_vote_received: row.get(45)?,
            showcase_top_model_consideration_battles: row.get(46)?,
            reputation_per_battle_promotion: row.get(47)?,
            coins_lost_on_death_min_timeout: row.get(48)?,
            coins_lost_on_death_max_timeout: row.get(49)?,
            mail_base_fee: row.get(50)?,
            mail_percent_attachment_fee: row.get(51)?,
            property_reputation_delay: row.get(52)?,
            level_cap: row.get(53)?,
            level_up_behavior_effect: trim_to_string(row.get(54)?),
            character_version: row.get(55)?,
            level_cap_currency_conversion: row.get(56)?,
        })
    }
}

impl HasKey for WorldConfig {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.world_config_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ZoneLoadingTips {
    pub id: i32,
    pub zoneid: i32,
    pub imagelocation: String,
    pub localize: bool,
    pub gate_version: String,
    pub loc_status: i32,
    pub weight: i32,
    pub target_version: Option<String>,
}

impl FromCdClient for ZoneLoadingTips {
    const TABLE: &'static str = "ZoneLoadingTips";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            zoneid: row.get(1)?,
            imagelocation: trim_to_string(row.get(2)?),
            localize: row.get(3)?,
            gate_version: trim_to_string(row.get(4)?),
            loc_status: row.get(5)?,
            weight: row.get(6)?,
            target_version: trim_and_nullify(row.get(7)?),
        })
    }
}

impl HasKey for ZoneLoadingTips {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ZoneSummary {
    pub zone_id: i32,
    pub r#type: i32,
    pub value: Option<i32>,
    pub unique_id: i32,
}

impl FromCdClient for ZoneSummary {
    const TABLE: &'static str = "ZoneSummary";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            zone_id: row.get(0)?,
            r#type: row.get(1)?,
            value: row.get(2)?,
            unique_id: row.get(3)?,
        })
    }
}

impl HasGroupKey for ZoneSummary {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.zone_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ZoneTable {
    pub zone_id: i32,
    pub loc_status: i32,
    pub zone_name: String,
    pub script_id: Option<i32>,
    pub ghostdistance_min: Option<f64>,
    pub ghostdistance: f64,
    pub population_soft_cap: i32,
    pub population_hard_cap: i32,
    pub display_description: Option<String>,
    pub map_folder: Option<String>,
    pub smashable_min_distance: Option<f64>,
    pub smashable_max_distance: Option<f64>,
    pub mixer_program: Option<String>,
    pub client_physics_framerate: Option<String>,
    pub server_physics_framerate: Option<String>,
    pub zone_control_template: Option<i32>,
    pub width_in_chunks: Option<i32>,
    pub height_in_chunks: Option<i32>,
    pub pets_allowed: bool,
    pub localize: bool,
    pub f_zone_weight: Option<f64>,
    pub thumbnail: Option<String>,
    pub player_lose_coins_on_death: bool,
    pub disable_save_loc: bool,
    pub team_radius: Option<f64>,
    pub gate_version: Option<String>,
    pub mounts_allowed: bool,
}

impl FromCdClient for ZoneTable {
    const TABLE: &'static str = "ZoneTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            zone_id: row.get(0)?,
            loc_status: row.get(1)?,
            zone_name: trim_to_string(row.get(2)?),
            script_id: row.get(3)?,
            ghostdistance_min: row.get(4)?,
            ghostdistance: row.get(5)?,
            population_soft_cap: row.get(6)?,
            population_hard_cap: row.get(7)?,
            display_description: trim_and_nullify(row.get(8)?),
            map_folder: trim_and_nullify(row.get(9)?),
            smashable_min_distance: row.get(10)?,
            smashable_max_distance: row.get(11)?,
            mixer_program: trim_and_nullify(row.get(12)?),
            client_physics_framerate: trim_and_nullify(row.get(13)?),
            server_physics_framerate: trim_and_nullify(row.get(14)?),
            zone_control_template: row.get(15)?,
            width_in_chunks: row.get(16)?,
            height_in_chunks: row.get(17)?,
            pets_allowed: row.get(18)?,
            localize: row.get(19)?,
            f_zone_weight: row.get(20)?,
            thumbnail: trim_and_nullify(row.get(21)?),
            player_lose_coins_on_death: row.get(22)?,
            disable_save_loc: row.get(23)?,
            team_radius: row.get(24)?,
            gate_version: trim_and_nullify(row.get(25)?),
            mounts_allowed: row.get(26)?,
        })
    }
}

impl HasKey for ZoneTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.zone_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BrickAttributes {
    pub id: i32,
    pub icon_asset: String,
    pub display_order: i32,
    pub loc_status: i32,
}

impl FromCdClient for BrickAttributes {
    const TABLE: &'static str = "brickAttributes";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            icon_asset: trim_to_string(row.get(1)?),
            display_order: row.get(2)?,
            loc_status: row.get(3)?,
        })
    }
}

impl HasKey for BrickAttributes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Dtproperties {
    pub id: i32,
    pub objectid: i32,
    pub property: String,
    pub value: String,
    pub uvalue: String,
    pub lvalue: String,
    pub version: i32,
}

impl FromCdClient for Dtproperties {
    const TABLE: &'static str = "dtproperties";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            objectid: row.get(1)?,
            property: trim_to_string(row.get(2)?),
            value: trim_to_string(row.get(3)?),
            uvalue: trim_to_string(row.get(4)?),
            lvalue: trim_to_string(row.get(5)?),
            version: row.get(6)?,
        })
    }
}

impl HasKey for Dtproperties {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MapAnimationPriorities {
    pub id: i32,
    pub name: String,
    pub priority: f64,
}

impl FromCdClient for MapAnimationPriorities {
    const TABLE: &'static str = "mapAnimationPriorities";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: trim_to_string(row.get(1)?),
            priority: row.get(2)?,
        })
    }
}

impl HasKey for MapAnimationPriorities {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MapAssetType {
    pub id: i32,
    pub label: String,
    pub pathdir: String,
    pub typelabel: String,
}

impl FromCdClient for MapAssetType {
    const TABLE: &'static str = "mapAssetType";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            label: trim_to_string(row.get(1)?),
            pathdir: trim_to_string(row.get(2)?),
            typelabel: trim_to_string(row.get(3)?),
        })
    }
}

impl HasKey for MapAssetType {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MapIcon {
    pub lot: i32,
    pub icon_id: i32,
    pub icon_state: i32,
}

impl FromCdClient for MapIcon {
    const TABLE: &'static str = "mapIcon";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            lot: row.get(0)?,
            icon_id: row.get(1)?,
            icon_state: row.get(2)?,
        })
    }
}

impl HasGroupKey for MapIcon {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.lot
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MapItemTypes {
    pub id: i32,
    pub description: String,
    pub equip_location: Option<String>,
}

impl FromCdClient for MapItemTypes {
    const TABLE: &'static str = "mapItemTypes";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            description: trim_to_string(row.get(1)?),
            equip_location: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for MapItemTypes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MapRenderEffects {
    pub id: i32,
    pub game_id: i32,
    pub description: String,
}

impl FromCdClient for MapRenderEffects {
    const TABLE: &'static str = "mapRenderEffects";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            game_id: row.get(1)?,
            description: trim_to_string(row.get(2)?),
        })
    }
}

impl HasKey for MapRenderEffects {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MapShaders {
    pub id: i32,
    pub label: String,
    pub game_value: i32,
    pub priority: Option<i32>,
}

impl FromCdClient for MapShaders {
    const TABLE: &'static str = "mapShaders";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            label: trim_to_string(row.get(1)?),
            game_value: row.get(2)?,
            priority: row.get(3)?,
        })
    }
}

impl HasKey for MapShaders {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MapTextureResource {
    pub id: i32,
    pub texturepath: String,
    pub surface_type: i32,
}

impl FromCdClient for MapTextureResource {
    const TABLE: &'static str = "mapTextureResource";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            texturepath: trim_to_string(row.get(1)?),
            surface_type: row.get(2)?,
        })
    }
}

impl HasKey for MapTextureResource {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MapBlueprintCategory {
    pub id: i32,
    pub description: String,
    pub enabled: bool,
}

impl FromCdClient for MapBlueprintCategory {
    const TABLE: &'static str = "map_BlueprintCategory";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            description: trim_to_string(row.get(1)?),
            enabled: row.get(2)?,
        })
    }
}

impl HasKey for MapBlueprintCategory {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Sysdiagrams {
    pub name: String,
    pub principal_id: i32,
    pub diagram_id: i32,
    pub version: i32,
    pub definition: String,
}

impl FromCdClient for Sysdiagrams {
    const TABLE: &'static str = "sysdiagrams";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            name: trim_to_string(row.get(0)?),
            principal_id: row.get(1)?,
            diagram_id: row.get(2)?,
            version: row.get(3)?,
            definition: trim_to_string(row.get(4)?),
        })
    }
}

impl HasKey for Sysdiagrams {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.name
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct PossessableComponent {
    pub id: i32,
    pub control_scheme_id: i32,
    pub minifig_attach_point: Option<String>,
    pub minifig_attach_animation: Option<String>,
    pub minifig_detach_animation: Option<String>,
    pub mount_attach_animation: Option<String>,
    pub mount_detach_animation: Option<String>,
    pub attach_offset_fwd: Option<f64>,
    pub attach_offset_right: Option<f64>,
    pub possession_type: i32,
    pub want_billboard: bool,
    pub billboard_offset_up: Option<f64>,
    pub depossess_on_hit: bool,
    pub hit_stun_time: Option<f64>,
    pub skill_set: Option<i32>,
    pub immune_on_possess_time: Option<f64>,
    pub immune_on_depossess_time: Option<f64>,
    pub disable_interactions: Option<bool>,
}

impl FromCdClient for PossessableComponent {
    const TABLE: &'static str = "PossessableComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            control_scheme_id: row.get(1)?,
            minifig_attach_point: trim_and_nullify(row.get(2)?),
            minifig_attach_animation: trim_and_nullify(row.get(3)?),
            minifig_detach_animation: trim_and_nullify(row.get(4)?),
            mount_attach_animation: trim_and_nullify(row.get(5)?),
            mount_detach_animation: trim_and_nullify(row.get(6)?),
            attach_offset_fwd: row.get(7)?,
            attach_offset_right: row.get(8)?,
            possession_type: row.get(9)?,
            want_billboard: row.get(10)?,
            billboard_offset_up: row.get(11)?,
            depossess_on_hit: row.get(12)?,
            hit_stun_time: row.get(13)?,
            skill_set: row.get(14)?,
            immune_on_possess_time: row.get(15)?,
            immune_on_depossess_time: row.get(16)?,
            disable_interactions: row.get(17)?,
        })
    }
}

impl HasKey for PossessableComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SkillSets {
    pub id: i32,
    pub slot1: Option<i32>,
    pub slot2: Option<i32>,
    pub slot3: Option<i32>,
    pub slot4: Option<i32>,
    pub priority: f64,
    pub description: Option<String>,
}

impl FromCdClient for SkillSets {
    const TABLE: &'static str = "SkillSets";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            slot1: row.get(1)?,
            slot2: row.get(2)?,
            slot3: row.get(3)?,
            slot4: row.get(4)?,
            priority: row.get(5)?,
            description: trim_and_nullify(row.get(6)?),
        })
    }
}

impl HasKey for SkillSets {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MapFacesAndHair {
    pub id: i32,
    pub eyes: i32,
    pub eyebrows: i32,
    pub mouths: i32,
    pub haircolor: i32,
    pub hairstyle: i32,
}

impl FromCdClient for MapFacesAndHair {
    const TABLE: &'static str = "mapFacesAndHair";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            eyes: row.get(1)?,
            eyebrows: row.get(2)?,
            mouths: row.get(3)?,
            haircolor: row.get(4)?,
            hairstyle: row.get(5)?,
        })
    }
}

impl HasKey for MapFacesAndHair {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}
