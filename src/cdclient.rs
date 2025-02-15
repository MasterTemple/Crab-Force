use std::{
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
};

/// https://docs.lu-dev.net/en/latest/components.html
pub mod components {
    pub const CONTROLLABLE_PHYSICS_COMPONENT: i32 = 1;
    pub const RENDER_COMPONENT: i32 = 2;
    pub const SIMPLE_PHYSICS_COMPONENT: i32 = 3;
    pub const CHARACTER_COMPONENT: i32 = 4;
    pub const SCRIPT_COMPONENT: i32 = 5;
    pub const BOUNCER_COMPONENT: i32 = 6;
    pub const DESTROYABLE_COMPONENT: i32 = 7;
    pub const GHOST_COMPONENT: i32 = 8;
    pub const SKILL_COMPONENT: i32 = 9;
    pub const SPAWNER_COMPONENT: i32 = 10;
    pub const ITEM_COMPONENT: i32 = 11;
    pub const REBUILD_COMPONENT: i32 = 12;
    pub const REBUILD_START_COMPONENT: i32 = 13;
    pub const REBUILD_ACTIVATOR_COMPONENT: i32 = 14;
    pub const ICON_ONLY_COMPONENT: i32 = 15;
    pub const VENDOR_COMPONENT: i32 = 16;
    pub const INVENTORY_COMPONENT: i32 = 17;
    pub const PROJECTILE_PHYSICS_COMPONENT: i32 = 18;
    pub const SHOOTING_GALLERY_COMPONENT: i32 = 19;
    pub const RIGIDBODYPHANTOMPHYSICS_COMPONENT: i32 = 20;
    pub const DROP_EFFECT_COMPONENT: i32 = 21;
    pub const CHEST_COMPONENT: i32 = 22;
    pub const COLLECTIBLE_COMPONENT: i32 = 23;
    pub const BLUEPRINT_COMPONENT: i32 = 24;
    pub const MOVING_PLATFORM_COMPONENT: i32 = 25;
    pub const PET_COMPONENT: i32 = 26;
    pub const PLATFORM_BOUNDARY_COMPONENT: i32 = 27;
    pub const MODULE_COMPONENT: i32 = 28;
    pub const JET_PACK_PAD: i32 = 29;
    pub const VEHICLE_PHYSICS_COMPONENT: i32 = 30;
    pub const MOVEMENTAI_COMPONENT: i32 = 31;
    pub const EXHIBIT_COMPONENT: i32 = 32;
    pub const OVERHEADICON_COMPONENT: i32 = 33;
    pub const PET_CONTROL_COMPONENT: i32 = 34;
    pub const MINIFIG_COMPONENT: i32 = 35;
    pub const PROPERTY_COMPONENT: i32 = 36;
    pub const PET_CREATOR_COMPONENT: i32 = 37;
    pub const MODEL_BUILDER_COMPONENT: i32 = 38;
    pub const SCRIPTED_ACTIVITY_COMPONENT: i32 = 39;
    pub const PHANTOM_PHYSICS_COMPONENT: i32 = 40;
    pub const SPRINGPAD_COMPONENT: i32 = 41;
    pub const B3_BEHAVIORS_COMPONENT: i32 = 42;
    pub const PROPERTY_ENTRANCE_COMPONENT: i32 = 43;
    pub const FX_COMPONENT: i32 = 44;
    pub const PROPERTY_MANAGEMENT_COMPONENT: i32 = 45;
    pub const VEHICLE_PHYSICS_COMPONENT2: i32 = 46;
    pub const PHYSICS_SYSTEM_COMPONENT: i32 = 47;
    pub const QUICK_BUILD_COMPONENT: i32 = 48;
    pub const SWITCH_COMPONENT: i32 = 49;
    pub const MINIGAME_COMPONENT: i32 = 50;
    pub const CHANGLING_COMPONENT: i32 = 51;
    pub const CHOICE_BUILD_COMPONENT: i32 = 52;
    pub const PACKAGE_COMPONENT: i32 = 53;
    pub const SOUND_REPEATER_COMPONENT: i32 = 54;
    pub const SOUND_AMBIENT_2D_COMPONENT: i32 = 55;
    pub const SOUND_AMBIENT_3D_COMPONENT: i32 = 56;
    pub const PRECONDITION_COMPONENT: i32 = 57;
    pub const PLAYER_FLAGS_COMPONENT: i32 = 58;
    pub const CUSTOM_BUILD_ASSEMBLY_COMPONENT: i32 = 59;
    pub const BASE_COMBAT_AI_COMPONENT: i32 = 60;
    pub const MODULE_ASSEMBLY_COMPONENT: i32 = 61;
    pub const SHOWCASE_MODEL_HANDLER_COMPONENT: i32 = 62;
    pub const RACING_MODULE_COMPONENT: i32 = 63;
    pub const GENERIC_ACTIVATOR_COMPONENT: i32 = 64;
    pub const PROPERTY_VENDOR_COMPONENT: i32 = 65;
    pub const HFLIGHTDIRECTIONGADGET_COMPONENT: i32 = 66;
    pub const ROCKET_LAUNCH_COMPONENT: i32 = 67;
    pub const ROCKET_LANDING_COMPONENT: i32 = 68;
    pub const TRIGGER_COMPONENT: i32 = 69;
    pub const DROPPED_LOOT_COMPONENT: i32 = 70;
    pub const RACING_CONTROL_COMPONENT: i32 = 71;
    pub const FACTION_TRIGGER_COMPONENT: i32 = 72;
    pub const MISSION_OFFER_COMPONENT: i32 = 73;
    pub const RACING_STATS_COMPONENT: i32 = 74;
    pub const LUP_EXHIBIT_COMPONENT: i32 = 75;
    pub const BBB_COMPONENT: i32 = 76;
    pub const SOUND_TRIGGER_COMPONENT: i32 = 77;
    pub const PROXIMITY_MONITOR_COMPONENT: i32 = 78;
    pub const RACING_SOUND_TRIGGER_COMPONENT: i32 = 79;
    pub const CHAT_COMPONENT: i32 = 80;
    pub const FRIENDS_LIST_COMPONENT: i32 = 81;
    pub const GUILD_COMPONENT: i32 = 82;
    pub const LOCAL_SYSTEM_COMPONENT: i32 = 83;
    pub const MISSION_COMPONENT: i32 = 84;
    pub const MUTABLE_MODEL_BEHAVIORS_COMPONENT: i32 = 85;
    pub const PATHFINDING_CONTROL_COMPONENT: i32 = 86;
    pub const PET_TAMING_CONTROL_COMPONENT: i32 = 87;
    pub const PROPERTY_EDITOR_COMPONENT: i32 = 88;
    pub const SKINNED_RENDER_COMPONENT: i32 = 89;
    pub const SLASH_COMMAND_COMPONENT: i32 = 90;
    pub const STATUS_EFFECT_COMPONENT: i32 = 91;
    pub const TEAMS_COMPONENT: i32 = 92;
    pub const TEXT_EFFECT_COMPONENT: i32 = 93;
    pub const TRADE_COMPONENT: i32 = 94;
    pub const USER_CONTROL_COMPONENT: i32 = 95;
    pub const IGNORE_LIST_COMPONENT: i32 = 96;
    pub const LUP_LAUNCHPAD_COMPONENT: i32 = 97;
    pub const BUFF_COMPONENT: i32 = 98;
    pub const INTERACTION_MANAGER_COMPONENT: i32 = 99;
    pub const DONATION_VENDOR_COMPONENT: i32 = 100;
    pub const COMBAT_MEDIATOR_COMPONENT: i32 = 101;
    pub const PLAYER_FORCED_MOVEMENT_COMPONENT: i32 = 106;
    pub const BRICK_BY_BRICK_COMPONENT: i32 = 107;
    pub const LEVEL_PROGRESSION_COMPONENT: i32 = 109;
    pub const POSSESSION_CONTROL_COMPONENT: i32 = 110;
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
    // pub fn at_group_key(&self, key: &T::Key) -> Option<&[T]> {
    pub fn at_group_key(&self, key: &T::Key) -> Option<Vec<T>> {
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

        Some(self.0[min_idx..=max_idx].to_vec())
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
    pub ai_combat_roles: Vec<CdClientAiCombatRoles>,
    pub accessory_default_loc: Vec<CdClientAccessoryDefaultLoc>,
    pub activities: Vec<CdClientActivities>,
    pub activity_rewards: Vec<CdClientActivityRewards>,
    pub activity_text: Vec<CdClientActivityText>,
    pub animation_index: Vec<CdClientAnimationIndex>,
    pub animations: Vec<CdClientAnimations>,
    pub base_combat_aicomponent: Vec<CdClientBaseCombatAIComponent>,
    pub behavior_effect: Vec<CdClientBehaviorEffect>,
    pub behavior_parameter: Vec<CdClientBehaviorParameter>,
    pub behavior_template: Vec<CdClientBehaviorTemplate>,
    pub behavior_template_name: Vec<CdClientBehaviorTemplateName>,
    pub blueprints: Vec<CdClientBlueprints>,
    pub brick_colors: Vec<CdClientBrickColors>,
    pub brick_idtable: Vec<CdClientBrickIDTable>,
    pub buff_definitions: Vec<CdClientBuffDefinitions>,
    pub buff_parameters: Vec<CdClientBuffParameters>,
    pub camera: Vec<CdClientCamera>,
    pub celebration_parameters: Vec<CdClientCelebrationParameters>,
    pub choice_build_component: Vec<CdClientChoiceBuildComponent>,
    pub collectible_component: Vec<CdClientCollectibleComponent>,
    pub components_registry: Vec<CdClientComponentsRegistry>,
    pub control_schemes: Vec<CdClientControlSchemes>,
    pub currency_denominations: Vec<CdClientCurrencyDenominations>,
    pub currency_table: Vec<CdClientCurrencyTable>,
    pub db_exclude: Vec<CdClientDbExclude>,
    pub deletion_restrictions: Vec<CdClientDeletionRestrictions>,
    pub destructible_component: Vec<CdClientDestructibleComponent>,
    pub dev_model_behaviors: Vec<CdClientDevModelBehaviors>,
    pub emotes: Vec<CdClientEmotes>,
    pub event_gating: Vec<CdClientEventGating>,
    pub exhibit_component: Vec<CdClientExhibitComponent>,
    pub factions: Vec<CdClientFactions>,
    pub feature_gating: Vec<CdClientFeatureGating>,
    pub flair_table: Vec<CdClientFlairTable>,
    pub icons: Vec<CdClientIcons>,
    pub inventory_component: Vec<CdClientInventoryComponent>,
    pub item_component: Vec<CdClientItemComponent>,
    pub item_egg_data: Vec<CdClientItemEggData>,
    pub item_food_data: Vec<CdClientItemFoodData>,
    pub item_set_skills: Vec<CdClientItemSetSkills>,
    pub item_sets: Vec<CdClientItemSets>,
    pub jet_pack_pad_component: Vec<CdClientJetPackPadComponent>,
    pub lu_pexhibit_component: Vec<CdClientLuPExhibitComponent>,
    pub lu_pexhibit_model_data: Vec<CdClientLuPExhibitModelData>,
    pub lu_pzone_ids: Vec<CdClientLuPZoneIDs>,
    pub language_type: Vec<CdClientLanguageType>,
    pub level_progression_lookup: Vec<CdClientLevelProgressionLookup>,
    pub loot_matrix: Vec<CdClientLootMatrix>,
    pub loot_matrix_index: Vec<CdClientLootMatrixIndex>,
    pub loot_table: Vec<CdClientLootTable>,
    pub loot_table_index: Vec<CdClientLootTableIndex>,
    pub minifig_component: Vec<CdClientMinifigComponent>,
    pub minifig_decals_eyebrows: Vec<CdClientMinifigDecalsEyebrows>,
    pub minifig_decals_eyes: Vec<CdClientMinifigDecalsEyes>,
    pub minifig_decals_legs: Vec<CdClientMinifigDecalsLegs>,
    pub minifig_decals_mouths: Vec<CdClientMinifigDecalsMouths>,
    pub minifig_decals_torsos: Vec<CdClientMinifigDecalsTorsos>,
    pub mission_email: Vec<CdClientMissionEmail>,
    pub mission_npccomponent: Vec<CdClientMissionNPCComponent>,
    pub mission_tasks: Vec<CdClientMissionTasks>,
    pub mission_text: Vec<CdClientMissionText>,
    pub missions: Vec<CdClientMissions>,
    pub model_behavior: Vec<CdClientModelBehavior>,
    pub modular_build_component: Vec<CdClientModularBuildComponent>,
    pub module_component: Vec<CdClientModuleComponent>,
    pub motion_fx: Vec<CdClientMotionFX>,
    pub movement_aicomponent: Vec<CdClientMovementAIComponent>,
    pub moving_platforms: Vec<CdClientMovingPlatforms>,
    pub npc_icons: Vec<CdClientNpcIcons>,
    pub object_behavior_xref: Vec<CdClientObjectBehaviorXREF>,
    pub object_behaviors: Vec<CdClientObjectBehaviors>,
    pub object_skills: Vec<CdClientObjectSkills>,
    pub objects: Vec<CdClientObjects>,
    pub package_component: Vec<CdClientPackageComponent>,
    pub pet_abilities: Vec<CdClientPetAbilities>,
    pub pet_component: Vec<CdClientPetComponent>,
    pub pet_nest_component: Vec<CdClientPetNestComponent>,
    pub physics_component: Vec<CdClientPhysicsComponent>,
    pub player_flags: Vec<CdClientPlayerFlags>,
    pub player_statistics: Vec<CdClientPlayerStatistics>,
    pub preconditions: Vec<CdClientPreconditions>,
    pub property_entrance_component: Vec<CdClientPropertyEntranceComponent>,
    pub property_template: Vec<CdClientPropertyTemplate>,
    pub proximity_monitor_component: Vec<CdClientProximityMonitorComponent>,
    pub proximity_types: Vec<CdClientProximityTypes>,
    pub racing_module_component: Vec<CdClientRacingModuleComponent>,
    pub rail_activator_component: Vec<CdClientRailActivatorComponent>,
    pub rarity_table: Vec<CdClientRarityTable>,
    pub rarity_table_index: Vec<CdClientRarityTableIndex>,
    pub rebuild_component: Vec<CdClientRebuildComponent>,
    pub rebuild_sections: Vec<CdClientRebuildSections>,
    pub release_version: Vec<CdClientReleaseVersion>,
    pub render_component: Vec<CdClientRenderComponent>,
    pub render_component_flash: Vec<CdClientRenderComponentFlash>,
    pub render_component_wrapper: Vec<CdClientRenderComponentWrapper>,
    pub render_icon_assets: Vec<CdClientRenderIconAssets>,
    pub reputation_rewards: Vec<CdClientReputationRewards>,
    pub reward_codes: Vec<CdClientRewardCodes>,
    pub rewards: Vec<CdClientRewards>,
    pub rocket_launchpad_control_component: Vec<CdClientRocketLaunchpadControlComponent>,
    pub scene_table: Vec<CdClientSceneTable>,
    pub script_component: Vec<CdClientScriptComponent>,
    pub skill_behavior: Vec<CdClientSkillBehavior>,
    pub smashable_chain: Vec<CdClientSmashableChain>,
    pub smashable_chain_index: Vec<CdClientSmashableChainIndex>,
    pub smashable_component: Vec<CdClientSmashableComponent>,
    pub smashable_elements: Vec<CdClientSmashableElements>,
    pub speedchat_menu: Vec<CdClientSpeedchatMenu>,
    pub subscription_pricing: Vec<CdClientSubscriptionPricing>,
    pub surface_type: Vec<CdClientSurfaceType>,
    pub taming_build_puzzles: Vec<CdClientTamingBuildPuzzles>,
    pub text_description: Vec<CdClientTextDescription>,
    pub text_language: Vec<CdClientTextLanguage>,
    pub trail_effects: Vec<CdClientTrailEffects>,
    pub ug_behavior_sounds: Vec<CdClientUgBehaviorSounds>,
    pub vehicle_physics: Vec<CdClientVehiclePhysics>,
    pub vehicle_stat_map: Vec<CdClientVehicleStatMap>,
    pub vendor_component: Vec<CdClientVendorComponent>,
    pub whats_cool_item_spotlight: Vec<CdClientWhatsCoolItemSpotlight>,
    pub whats_cool_news_and_tips: Vec<CdClientWhatsCoolNewsAndTips>,
    pub world_config: Vec<CdClientWorldConfig>,
    pub zone_loading_tips: Vec<CdClientZoneLoadingTips>,
    pub zone_summary: Vec<CdClientZoneSummary>,
    pub zone_table: Vec<CdClientZoneTable>,
    pub brick_attributes: Vec<CdClientBrickAttributes>,
    pub dtproperties: Vec<CdClientDtproperties>,
    pub map_animation_priorities: Vec<CdClientMapAnimationPriorities>,
    pub map_asset_type: Vec<CdClientMapAssetType>,
    pub map_icon: Vec<CdClientMapIcon>,
    pub map_item_types: Vec<CdClientMapItemTypes>,
    pub map_render_effects: Vec<CdClientMapRenderEffects>,
    pub map_shaders: Vec<CdClientMapShaders>,
    pub map_texture_resource: Vec<CdClientMapTextureResource>,
    pub map_blueprint_category: Vec<CdClientMapBlueprintCategory>,
    pub sysdiagrams: Vec<CdClientSysdiagrams>,
    pub possessable_component: Vec<CdClientPossessableComponent>,
    pub skill_sets: Vec<CdClientSkillSets>,
    pub map_faces_and_hair: Vec<CdClientMapFacesAndHair>,
}

impl CdClientRows {
    pub fn load_sqlite(path: &Path) -> rusqlite::Result<Self> {
        let conn = rusqlite::Connection::open(path)?;
        Ok(Self {
            ai_combat_roles: CdClientAiCombatRoles::load(&conn)?,
            accessory_default_loc: CdClientAccessoryDefaultLoc::load(&conn)?,
            activities: CdClientActivities::load(&conn)?,
            activity_rewards: CdClientActivityRewards::load(&conn)?,
            activity_text: CdClientActivityText::load(&conn)?,
            animation_index: CdClientAnimationIndex::load(&conn)?,
            animations: CdClientAnimations::load(&conn)?,
            base_combat_aicomponent: CdClientBaseCombatAIComponent::load(&conn)?,
            behavior_effect: CdClientBehaviorEffect::load(&conn)?,
            behavior_parameter: CdClientBehaviorParameter::load(&conn)?,
            behavior_template: CdClientBehaviorTemplate::load(&conn)?,
            behavior_template_name: CdClientBehaviorTemplateName::load(&conn)?,
            blueprints: CdClientBlueprints::load(&conn)?,
            brick_colors: CdClientBrickColors::load(&conn)?,
            brick_idtable: CdClientBrickIDTable::load(&conn)?,
            buff_definitions: CdClientBuffDefinitions::load(&conn)?,
            buff_parameters: CdClientBuffParameters::load(&conn)?,
            camera: CdClientCamera::load(&conn)?,
            celebration_parameters: CdClientCelebrationParameters::load(&conn)?,
            choice_build_component: CdClientChoiceBuildComponent::load(&conn)?,
            collectible_component: CdClientCollectibleComponent::load(&conn)?,
            components_registry: CdClientComponentsRegistry::load(&conn)?,
            control_schemes: CdClientControlSchemes::load(&conn)?,
            currency_denominations: CdClientCurrencyDenominations::load(&conn)?,
            currency_table: CdClientCurrencyTable::load(&conn)?,
            db_exclude: CdClientDbExclude::load(&conn)?,
            deletion_restrictions: CdClientDeletionRestrictions::load(&conn)?,
            destructible_component: CdClientDestructibleComponent::load(&conn)?,
            dev_model_behaviors: CdClientDevModelBehaviors::load(&conn)?,
            emotes: CdClientEmotes::load(&conn)?,
            event_gating: CdClientEventGating::load(&conn)?,
            exhibit_component: CdClientExhibitComponent::load(&conn)?,
            factions: CdClientFactions::load(&conn)?,
            feature_gating: CdClientFeatureGating::load(&conn)?,
            flair_table: CdClientFlairTable::load(&conn)?,
            icons: CdClientIcons::load(&conn)?,
            inventory_component: CdClientInventoryComponent::load(&conn)?,
            item_component: CdClientItemComponent::load(&conn)?,
            item_egg_data: CdClientItemEggData::load(&conn)?,
            item_food_data: CdClientItemFoodData::load(&conn)?,
            item_set_skills: CdClientItemSetSkills::load(&conn)?,
            item_sets: CdClientItemSets::load(&conn)?,
            jet_pack_pad_component: CdClientJetPackPadComponent::load(&conn)?,
            lu_pexhibit_component: CdClientLuPExhibitComponent::load(&conn)?,
            lu_pexhibit_model_data: CdClientLuPExhibitModelData::load(&conn)?,
            lu_pzone_ids: CdClientLuPZoneIDs::load(&conn)?,
            language_type: CdClientLanguageType::load(&conn)?,
            level_progression_lookup: CdClientLevelProgressionLookup::load(&conn)?,
            loot_matrix: CdClientLootMatrix::load(&conn)?,
            loot_matrix_index: CdClientLootMatrixIndex::load(&conn)?,
            loot_table: CdClientLootTable::load(&conn)?,
            loot_table_index: CdClientLootTableIndex::load(&conn)?,
            minifig_component: CdClientMinifigComponent::load(&conn)?,
            minifig_decals_eyebrows: CdClientMinifigDecalsEyebrows::load(&conn)?,
            minifig_decals_eyes: CdClientMinifigDecalsEyes::load(&conn)?,
            minifig_decals_legs: CdClientMinifigDecalsLegs::load(&conn)?,
            minifig_decals_mouths: CdClientMinifigDecalsMouths::load(&conn)?,
            minifig_decals_torsos: CdClientMinifigDecalsTorsos::load(&conn)?,
            mission_email: CdClientMissionEmail::load(&conn)?,
            mission_npccomponent: CdClientMissionNPCComponent::load(&conn)?,
            mission_tasks: CdClientMissionTasks::load(&conn)?,
            mission_text: CdClientMissionText::load(&conn)?,
            missions: CdClientMissions::load(&conn)?,
            model_behavior: CdClientModelBehavior::load(&conn)?,
            modular_build_component: CdClientModularBuildComponent::load(&conn)?,
            module_component: CdClientModuleComponent::load(&conn)?,
            motion_fx: CdClientMotionFX::load(&conn)?,
            movement_aicomponent: CdClientMovementAIComponent::load(&conn)?,
            moving_platforms: CdClientMovingPlatforms::load(&conn)?,
            npc_icons: CdClientNpcIcons::load(&conn)?,
            object_behavior_xref: CdClientObjectBehaviorXREF::load(&conn)?,
            object_behaviors: CdClientObjectBehaviors::load(&conn)?,
            object_skills: CdClientObjectSkills::load(&conn)?,
            objects: CdClientObjects::load(&conn)?,
            package_component: CdClientPackageComponent::load(&conn)?,
            pet_abilities: CdClientPetAbilities::load(&conn)?,
            pet_component: CdClientPetComponent::load(&conn)?,
            pet_nest_component: CdClientPetNestComponent::load(&conn)?,
            physics_component: CdClientPhysicsComponent::load(&conn)?,
            player_flags: CdClientPlayerFlags::load(&conn)?,
            player_statistics: CdClientPlayerStatistics::load(&conn)?,
            preconditions: CdClientPreconditions::load(&conn)?,
            property_entrance_component: CdClientPropertyEntranceComponent::load(&conn)?,
            property_template: CdClientPropertyTemplate::load(&conn)?,
            proximity_monitor_component: CdClientProximityMonitorComponent::load(&conn)?,
            proximity_types: CdClientProximityTypes::load(&conn)?,
            racing_module_component: CdClientRacingModuleComponent::load(&conn)?,
            rail_activator_component: CdClientRailActivatorComponent::load(&conn)?,
            rarity_table: CdClientRarityTable::load(&conn)?,
            rarity_table_index: CdClientRarityTableIndex::load(&conn)?,
            rebuild_component: CdClientRebuildComponent::load(&conn)?,
            rebuild_sections: CdClientRebuildSections::load(&conn)?,
            release_version: CdClientReleaseVersion::load(&conn)?,
            render_component: CdClientRenderComponent::load(&conn)?,
            render_component_flash: CdClientRenderComponentFlash::load(&conn)?,
            render_component_wrapper: CdClientRenderComponentWrapper::load(&conn)?,
            render_icon_assets: CdClientRenderIconAssets::load(&conn)?,
            reputation_rewards: CdClientReputationRewards::load(&conn)?,
            reward_codes: CdClientRewardCodes::load(&conn)?,
            rewards: CdClientRewards::load(&conn)?,
            rocket_launchpad_control_component: CdClientRocketLaunchpadControlComponent::load(
                &conn,
            )?,
            scene_table: CdClientSceneTable::load(&conn)?,
            script_component: CdClientScriptComponent::load(&conn)?,
            skill_behavior: CdClientSkillBehavior::load(&conn)?,
            smashable_chain: CdClientSmashableChain::load(&conn)?,
            smashable_chain_index: CdClientSmashableChainIndex::load(&conn)?,
            smashable_component: CdClientSmashableComponent::load(&conn)?,
            smashable_elements: CdClientSmashableElements::load(&conn)?,
            speedchat_menu: CdClientSpeedchatMenu::load(&conn)?,
            subscription_pricing: CdClientSubscriptionPricing::load(&conn)?,
            surface_type: CdClientSurfaceType::load(&conn)?,
            taming_build_puzzles: CdClientTamingBuildPuzzles::load(&conn)?,
            text_description: CdClientTextDescription::load(&conn)?,
            text_language: CdClientTextLanguage::load(&conn)?,
            trail_effects: CdClientTrailEffects::load(&conn)?,
            ug_behavior_sounds: CdClientUgBehaviorSounds::load(&conn)?,
            vehicle_physics: CdClientVehiclePhysics::load(&conn)?,
            vehicle_stat_map: CdClientVehicleStatMap::load(&conn)?,
            vendor_component: CdClientVendorComponent::load(&conn)?,
            whats_cool_item_spotlight: CdClientWhatsCoolItemSpotlight::load(&conn)?,
            whats_cool_news_and_tips: CdClientWhatsCoolNewsAndTips::load(&conn)?,
            world_config: CdClientWorldConfig::load(&conn)?,
            zone_loading_tips: CdClientZoneLoadingTips::load(&conn)?,
            zone_summary: CdClientZoneSummary::load(&conn)?,
            zone_table: CdClientZoneTable::load(&conn)?,
            brick_attributes: CdClientBrickAttributes::load(&conn)?,
            dtproperties: CdClientDtproperties::load(&conn)?,
            map_animation_priorities: CdClientMapAnimationPriorities::load(&conn)?,
            map_asset_type: CdClientMapAssetType::load(&conn)?,
            map_icon: CdClientMapIcon::load(&conn)?,
            map_item_types: CdClientMapItemTypes::load(&conn)?,
            map_render_effects: CdClientMapRenderEffects::load(&conn)?,
            map_shaders: CdClientMapShaders::load(&conn)?,
            map_texture_resource: CdClientMapTextureResource::load(&conn)?,
            map_blueprint_category: CdClientMapBlueprintCategory::load(&conn)?,
            sysdiagrams: CdClientSysdiagrams::load(&conn)?,
            possessable_component: CdClientPossessableComponent::load(&conn)?,
            skill_sets: CdClientSkillSets::load(&conn)?,
            map_faces_and_hair: CdClientMapFacesAndHair::load(&conn)?,
        })
    }
}

#[allow(dead_code)]
pub struct CdClient {
    pub ai_combat_roles: KeyedVec<CdClientAiCombatRoles>,
    pub accessory_default_loc: KeyedVec<CdClientAccessoryDefaultLoc>,
    pub activities: KeyedVec<CdClientActivities>,
    pub activity_rewards: GroupKeyedVec<CdClientActivityRewards>,
    pub activity_text: GroupKeyedVec<CdClientActivityText>,
    pub animation_index: KeyedVec<CdClientAnimationIndex>,
    pub animations: GroupKeyedVec<CdClientAnimations>,
    pub base_combat_aicomponent: KeyedVec<CdClientBaseCombatAIComponent>,
    pub behavior_effect: GroupKeyedVec<CdClientBehaviorEffect>,
    pub behavior_parameter: GroupKeyedVec<CdClientBehaviorParameter>,
    pub behavior_template: GroupKeyedVec<CdClientBehaviorTemplate>,
    pub behavior_template_name: KeyedVec<CdClientBehaviorTemplateName>,
    pub blueprints: KeyedVec<CdClientBlueprints>,
    pub brick_colors: KeyedVec<CdClientBrickColors>,
    pub brick_idtable: KeyedVec<CdClientBrickIDTable>,
    pub buff_definitions: KeyedVec<CdClientBuffDefinitions>,
    pub buff_parameters: KeyedVec<CdClientBuffParameters>,
    pub camera: KeyedVec<CdClientCamera>,
    pub celebration_parameters: KeyedVec<CdClientCelebrationParameters>,
    pub choice_build_component: KeyedVec<CdClientChoiceBuildComponent>,
    pub collectible_component: KeyedVec<CdClientCollectibleComponent>,
    pub components_registry: GroupKeyedVec<CdClientComponentsRegistry>,
    pub control_schemes: KeyedVec<CdClientControlSchemes>,
    pub currency_denominations: KeyedVec<CdClientCurrencyDenominations>,
    pub currency_table: KeyedVec<CdClientCurrencyTable>,
    pub db_exclude: KeyedVec<CdClientDbExclude>,
    pub deletion_restrictions: KeyedVec<CdClientDeletionRestrictions>,
    pub destructible_component: KeyedVec<CdClientDestructibleComponent>,
    pub dev_model_behaviors: KeyedVec<CdClientDevModelBehaviors>,
    pub emotes: KeyedVec<CdClientEmotes>,
    pub event_gating: KeyedVec<CdClientEventGating>,
    pub exhibit_component: KeyedVec<CdClientExhibitComponent>,
    pub factions: KeyedVec<CdClientFactions>,
    pub feature_gating: KeyedVec<CdClientFeatureGating>,
    pub flair_table: KeyedVec<CdClientFlairTable>,
    pub icons: KeyedVec<CdClientIcons>,
    pub inventory_component: Vec<CdClientInventoryComponent>,
    pub item_component: KeyedVec<CdClientItemComponent>,
    pub item_egg_data: KeyedVec<CdClientItemEggData>,
    pub item_food_data: KeyedVec<CdClientItemFoodData>,
    pub item_set_skills: GroupKeyedVec<CdClientItemSetSkills>,
    pub item_sets: KeyedVec<CdClientItemSets>,
    pub jet_pack_pad_component: KeyedVec<CdClientJetPackPadComponent>,
    pub lu_pexhibit_component: KeyedVec<CdClientLuPExhibitComponent>,
    pub lu_pexhibit_model_data: KeyedVec<CdClientLuPExhibitModelData>,
    pub lu_pzone_ids: KeyedVec<CdClientLuPZoneIDs>,
    pub language_type: KeyedVec<CdClientLanguageType>,
    pub level_progression_lookup: KeyedVec<CdClientLevelProgressionLookup>,
    pub loot_matrix: GroupKeyedVec<CdClientLootMatrix>,
    pub loot_matrix_index: KeyedVec<CdClientLootMatrixIndex>,
    pub loot_table: GroupKeyedVec<CdClientLootTable>,
    pub loot_table_index: KeyedVec<CdClientLootTableIndex>,
    pub minifig_component: KeyedVec<CdClientMinifigComponent>,
    pub minifig_decals_eyebrows: KeyedVec<CdClientMinifigDecalsEyebrows>,
    pub minifig_decals_eyes: KeyedVec<CdClientMinifigDecalsEyes>,
    pub minifig_decals_legs: KeyedVec<CdClientMinifigDecalsLegs>,
    pub minifig_decals_mouths: KeyedVec<CdClientMinifigDecalsMouths>,
    pub minifig_decals_torsos: KeyedVec<CdClientMinifigDecalsTorsos>,
    pub mission_email: KeyedVec<CdClientMissionEmail>,
    pub mission_npccomponent: Vec<CdClientMissionNPCComponent>,
    pub mission_tasks: GroupKeyedVec<CdClientMissionTasks>,
    pub mission_text: KeyedVec<CdClientMissionText>,
    pub missions: KeyedVec<CdClientMissions>,
    pub model_behavior: KeyedVec<CdClientModelBehavior>,
    pub modular_build_component: KeyedVec<CdClientModularBuildComponent>,
    pub module_component: KeyedVec<CdClientModuleComponent>,
    pub motion_fx: KeyedVec<CdClientMotionFX>,
    pub movement_aicomponent: KeyedVec<CdClientMovementAIComponent>,
    pub moving_platforms: KeyedVec<CdClientMovingPlatforms>,
    pub npc_icons: KeyedVec<CdClientNpcIcons>,
    pub object_behavior_xref: KeyedVec<CdClientObjectBehaviorXREF>,
    pub object_behaviors: KeyedVec<CdClientObjectBehaviors>,
    pub object_skills: GroupKeyedVec<CdClientObjectSkills>,
    pub objects: KeyedVec<CdClientObjects>,
    pub package_component: KeyedVec<CdClientPackageComponent>,
    pub pet_abilities: KeyedVec<CdClientPetAbilities>,
    pub pet_component: KeyedVec<CdClientPetComponent>,
    pub pet_nest_component: KeyedVec<CdClientPetNestComponent>,
    pub physics_component: KeyedVec<CdClientPhysicsComponent>,
    pub player_flags: KeyedVec<CdClientPlayerFlags>,
    pub player_statistics: KeyedVec<CdClientPlayerStatistics>,
    pub preconditions: KeyedVec<CdClientPreconditions>,
    pub property_entrance_component: KeyedVec<CdClientPropertyEntranceComponent>,
    pub property_template: KeyedVec<CdClientPropertyTemplate>,
    pub proximity_monitor_component: KeyedVec<CdClientProximityMonitorComponent>,
    pub proximity_types: KeyedVec<CdClientProximityTypes>,
    pub racing_module_component: KeyedVec<CdClientRacingModuleComponent>,
    pub rail_activator_component: KeyedVec<CdClientRailActivatorComponent>,
    pub rarity_table: GroupKeyedVec<CdClientRarityTable>,
    pub rarity_table_index: KeyedVec<CdClientRarityTableIndex>,
    pub rebuild_component: KeyedVec<CdClientRebuildComponent>,
    pub rebuild_sections: KeyedVec<CdClientRebuildSections>,
    pub release_version: KeyedVec<CdClientReleaseVersion>,
    pub render_component: KeyedVec<CdClientRenderComponent>,
    pub render_component_flash: GroupKeyedVec<CdClientRenderComponentFlash>,
    pub render_component_wrapper: KeyedVec<CdClientRenderComponentWrapper>,
    pub render_icon_assets: KeyedVec<CdClientRenderIconAssets>,
    pub reputation_rewards: KeyedVec<CdClientReputationRewards>,
    pub reward_codes: KeyedVec<CdClientRewardCodes>,
    pub rewards: KeyedVec<CdClientRewards>,
    pub rocket_launchpad_control_component: KeyedVec<CdClientRocketLaunchpadControlComponent>,
    pub scene_table: KeyedVec<CdClientSceneTable>,
    pub script_component: KeyedVec<CdClientScriptComponent>,
    pub skill_behavior: KeyedVec<CdClientSkillBehavior>,
    pub smashable_chain: GroupKeyedVec<CdClientSmashableChain>,
    pub smashable_chain_index: KeyedVec<CdClientSmashableChainIndex>,
    pub smashable_component: KeyedVec<CdClientSmashableComponent>,
    pub smashable_elements: KeyedVec<CdClientSmashableElements>,
    pub speedchat_menu: KeyedVec<CdClientSpeedchatMenu>,
    pub subscription_pricing: KeyedVec<CdClientSubscriptionPricing>,
    pub surface_type: KeyedVec<CdClientSurfaceType>,
    pub taming_build_puzzles: KeyedVec<CdClientTamingBuildPuzzles>,
    pub text_description: KeyedVec<CdClientTextDescription>,
    pub text_language: KeyedVec<CdClientTextLanguage>,
    pub trail_effects: KeyedVec<CdClientTrailEffects>,
    pub ug_behavior_sounds: KeyedVec<CdClientUgBehaviorSounds>,
    pub vehicle_physics: KeyedVec<CdClientVehiclePhysics>,
    pub vehicle_stat_map: GroupKeyedVec<CdClientVehicleStatMap>,
    pub vendor_component: KeyedVec<CdClientVendorComponent>,
    pub whats_cool_item_spotlight: KeyedVec<CdClientWhatsCoolItemSpotlight>,
    pub whats_cool_news_and_tips: KeyedVec<CdClientWhatsCoolNewsAndTips>,
    pub world_config: KeyedVec<CdClientWorldConfig>,
    pub zone_loading_tips: KeyedVec<CdClientZoneLoadingTips>,
    pub zone_summary: GroupKeyedVec<CdClientZoneSummary>,
    pub zone_table: KeyedVec<CdClientZoneTable>,
    pub brick_attributes: KeyedVec<CdClientBrickAttributes>,
    pub dtproperties: KeyedVec<CdClientDtproperties>,
    pub map_animation_priorities: KeyedVec<CdClientMapAnimationPriorities>,
    pub map_asset_type: KeyedVec<CdClientMapAssetType>,
    pub map_icon: GroupKeyedVec<CdClientMapIcon>,
    pub map_item_types: KeyedVec<CdClientMapItemTypes>,
    pub map_render_effects: KeyedVec<CdClientMapRenderEffects>,
    pub map_shaders: KeyedVec<CdClientMapShaders>,
    pub map_texture_resource: KeyedVec<CdClientMapTextureResource>,
    pub map_blueprint_category: KeyedVec<CdClientMapBlueprintCategory>,
    pub sysdiagrams: KeyedVec<CdClientSysdiagrams>,
    pub possessable_component: KeyedVec<CdClientPossessableComponent>,
    pub skill_sets: KeyedVec<CdClientSkillSets>,
    pub map_faces_and_hair: KeyedVec<CdClientMapFacesAndHair>,
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
            loot_table: GroupKeyedVec::new(cdclient.loot_table),
            loot_table_index: KeyedVec::new(cdclient.loot_table_index),
            minifig_component: KeyedVec::new(cdclient.minifig_component),
            minifig_decals_eyebrows: KeyedVec::new(cdclient.minifig_decals_eyebrows),
            minifig_decals_eyes: KeyedVec::new(cdclient.minifig_decals_eyes),
            minifig_decals_legs: KeyedVec::new(cdclient.minifig_decals_legs),
            minifig_decals_mouths: KeyedVec::new(cdclient.minifig_decals_mouths),
            minifig_decals_torsos: KeyedVec::new(cdclient.minifig_decals_torsos),
            mission_email: KeyedVec::new(cdclient.mission_email),
            mission_npccomponent: cdclient.mission_npccomponent,
            mission_tasks: GroupKeyedVec::new(cdclient.mission_tasks),
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
            rarity_table: GroupKeyedVec::new(cdclient.rarity_table),
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
pub struct CdClientAiCombatRoles {
    pub id: i32,
    pub preferred_role: i32,
    pub specified_min_range_nouse: Option<f64>,
    pub specified_max_range_nouse: Option<f64>,
    pub specific_min_range: Option<f64>,
    pub specific_max_range: Option<f64>,
}

impl FromCdClient for CdClientAiCombatRoles {
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

impl HasKey for CdClientAiCombatRoles {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientAccessoryDefaultLoc {
    pub group_id: i32,
    pub description: String,
    pub pos_x: f64,
    pub pos_y: f64,
    pub pos_z: f64,
    pub rot_x: f64,
    pub rot_y: f64,
    pub rot_z: f64,
}

impl FromCdClient for CdClientAccessoryDefaultLoc {
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

impl HasKey for CdClientAccessoryDefaultLoc {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.group_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientActivities {
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

impl FromCdClient for CdClientActivities {
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

impl HasKey for CdClientActivities {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.activity_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientActivityRewards {
    pub object_template: i32,
    pub activity_reward_index: i32,
    pub activity_rating: i32,
    pub loot_matrix_index: Option<i32>,
    pub currency_index: Option<i32>,
    pub challenge_rating: i32,
    pub description: String,
}

impl FromCdClient for CdClientActivityRewards {
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

impl HasGroupKey for CdClientActivityRewards {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.object_template
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientActivityText {
    pub activity_id: i32,
    pub r#type: String,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for CdClientActivityText {
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

impl HasGroupKey for CdClientActivityText {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.activity_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientAnimationIndex {
    pub animation_group_id: i32,
    pub description: String,
    pub group_type: Option<String>,
}

impl FromCdClient for CdClientAnimationIndex {
    const TABLE: &'static str = "AnimationIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            animation_group_id: row.get(0)?,
            description: trim_to_string(row.get(1)?),
            group_type: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for CdClientAnimationIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.animation_group_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientAnimations {
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

impl FromCdClient for CdClientAnimations {
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

impl HasGroupKey for CdClientAnimations {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.animation_group_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBaseCombatAIComponent {
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

impl FromCdClient for CdClientBaseCombatAIComponent {
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

impl HasKey for CdClientBaseCombatAIComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBehaviorEffect {
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

impl FromCdClient for CdClientBehaviorEffect {
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

impl HasGroupKey for CdClientBehaviorEffect {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.effect_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBehaviorParameter {
    pub behavior_id: i32,
    pub parameter_id: String,
    pub value: f64,
}

impl FromCdClient for CdClientBehaviorParameter {
    const TABLE: &'static str = "BehaviorParameter";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            behavior_id: row.get(0)?,
            parameter_id: trim_to_string(row.get(1)?),
            value: row.get(2)?,
        })
    }
}

impl HasGroupKey for CdClientBehaviorParameter {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.behavior_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBehaviorTemplate {
    pub behavior_id: i32,
    pub template_id: i32,
    pub effect_id: i32,
    pub effect_handle: Option<String>,
}

impl FromCdClient for CdClientBehaviorTemplate {
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

impl HasGroupKey for CdClientBehaviorTemplate {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.behavior_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBehaviorTemplateName {
    pub template_id: i32,
    pub name: String,
}

impl FromCdClient for CdClientBehaviorTemplateName {
    const TABLE: &'static str = "BehaviorTemplateName";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            template_id: row.get(0)?,
            name: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientBehaviorTemplateName {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.template_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBlueprints {
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

impl FromCdClient for CdClientBlueprints {
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

impl HasKey for CdClientBlueprints {
    type Key = i64;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBrickColors {
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

impl FromCdClient for CdClientBrickColors {
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

impl HasKey for CdClientBrickColors {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBrickIDTable {
    pub ndobject_id: i32,
    pub legobrick_id: i32,
}

impl FromCdClient for CdClientBrickIDTable {
    const TABLE: &'static str = "BrickIDTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            ndobject_id: row.get(0)?,
            legobrick_id: row.get(1)?,
        })
    }
}

impl HasKey for CdClientBrickIDTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.ndobject_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBuffDefinitions {
    pub id: i32,
    pub priority: f64,
    pub uiicon: Option<String>,
}

impl FromCdClient for CdClientBuffDefinitions {
    const TABLE: &'static str = "BuffDefinitions";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            priority: row.get(1)?,
            uiicon: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for CdClientBuffDefinitions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBuffParameters {
    pub buff_id: i32,
    pub parameter_name: String,
    pub number_value: Option<f64>,
    pub string_value: Option<Vec<f64>>,
    pub effect_id: Option<i32>,
}

impl FromCdClient for CdClientBuffParameters {
    const TABLE: &'static str = "CdClientBuffParameters";

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

impl HasKey for CdClientBuffParameters {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.buff_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientCamera {
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

impl FromCdClient for CdClientCamera {
    const TABLE: &'static str = "CdClientCamera";

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

impl HasKey for CdClientCamera {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.camera_name
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientCelebrationParameters {
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

impl FromCdClient for CdClientCelebrationParameters {
    const TABLE: &'static str = "CdClientCelebrationParameters";

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

impl HasKey for CdClientCelebrationParameters {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientChoiceBuildComponent {
    pub id: i32,
    pub selections: Vec<i32>,
    pub imagination_override: Option<i32>,
}

impl FromCdClient for CdClientChoiceBuildComponent {
    const TABLE: &'static str = "CdClientChoiceBuildComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            selections: parse_required_comma_list(row.get(1)?),
            imagination_override: row.get(2)?,
        })
    }
}

impl HasKey for CdClientChoiceBuildComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientCollectibleComponent {
    pub id: i32,
    pub requirement_mission: Option<i32>,
}

impl FromCdClient for CdClientCollectibleComponent {
    const TABLE: &'static str = "CdClientCollectibleComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            requirement_mission: row.get(1)?,
        })
    }
}

impl HasKey for CdClientCollectibleComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientComponentsRegistry {
    pub id: i32,
    pub component_type: i32,
    pub component_id: i32,
}

impl FromCdClient for CdClientComponentsRegistry {
    const TABLE: &'static str = "ComponentsRegistry";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            component_type: row.get(1)?,
            component_id: row.get(2)?,
        })
    }
}

impl HasGroupKey for CdClientComponentsRegistry {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientControlSchemes {
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

impl FromCdClient for CdClientControlSchemes {
    const TABLE: &'static str = "CdClientControlSchemes";

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

impl HasKey for CdClientControlSchemes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.control_scheme
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientCurrencyDenominations {
    pub value: i32,
    pub objectid: i32,
}

impl FromCdClient for CdClientCurrencyDenominations {
    const TABLE: &'static str = "CdClientCurrencyDenominations";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            value: row.get(0)?,
            objectid: row.get(1)?,
        })
    }
}

impl HasKey for CdClientCurrencyDenominations {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.objectid
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientCurrencyTable {
    pub currency_index: i32,
    pub npcminlevel: i32,
    pub minvalue: i32,
    pub maxvalue: i32,
    pub id: i32,
}

impl FromCdClient for CdClientCurrencyTable {
    const TABLE: &'static str = "CdClientCurrencyTable";

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

impl HasKey for CdClientCurrencyTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientDbExclude {
    pub table: String,
    pub column: String,
}

impl FromCdClient for CdClientDbExclude {
    const TABLE: &'static str = "DBExclude";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            table: trim_to_string(row.get(0)?),
            column: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientDbExclude {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.table
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientDeletionRestrictions {
    pub id: i32,
    pub restricted: bool,
    pub ids: Option<Vec<i32>>,
    pub check_type: i32,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for CdClientDeletionRestrictions {
    const TABLE: &'static str = "CdClientDeletionRestrictions";

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

impl HasKey for CdClientDeletionRestrictions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientDestructibleComponent {
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

impl FromCdClient for CdClientDestructibleComponent {
    const TABLE: &'static str = "CdClientDestructibleComponent";

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

impl HasKey for CdClientDestructibleComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientDevModelBehaviors {
    pub model_id: i32,
    pub behavior_id: i32,
}

impl FromCdClient for CdClientDevModelBehaviors {
    const TABLE: &'static str = "CdClientDevModelBehaviors";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            model_id: row.get(0)?,
            behavior_id: row.get(1)?,
        })
    }
}

impl HasKey for CdClientDevModelBehaviors {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.model_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientEmotes {
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

impl FromCdClient for CdClientEmotes {
    const TABLE: &'static str = "CdClientEmotes";

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

impl HasKey for CdClientEmotes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientEventGating {
    pub event_name: String,
    pub date_start: i64,
    pub date_end: i64,
}

impl FromCdClient for CdClientEventGating {
    const TABLE: &'static str = "CdClientEventGating";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            event_name: trim_to_string(row.get(0)?),
            date_start: row.get(1)?,
            date_end: row.get(2)?,
        })
    }
}

impl HasKey for CdClientEventGating {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.event_name
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientExhibitComponent {
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

impl FromCdClient for CdClientExhibitComponent {
    const TABLE: &'static str = "CdClientExhibitComponent";

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

impl HasKey for CdClientExhibitComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientFactions {
    pub faction: i32,
    pub faction_list: Vec<i32>,
    pub faction_list_friendly: bool,
    pub friend_list: Option<Vec<i32>>,
    pub enemy_list: Option<Vec<i32>>,
}

impl FromCdClient for CdClientFactions {
    const TABLE: &'static str = "CdClientFactions";

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

impl HasKey for CdClientFactions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.faction
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientFeatureGating {
    pub feature_name: String,
    pub major: i32,
    pub current: i32,
    pub minor: i32,
    pub description: Option<String>,
}

impl FromCdClient for CdClientFeatureGating {
    const TABLE: &'static str = "CdClientFeatureGating";

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

impl HasKey for CdClientFeatureGating {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.feature_name
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientFlairTable {
    pub id: i32,
    pub asset: String,
}

impl FromCdClient for CdClientFlairTable {
    const TABLE: &'static str = "CdClientFlairTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            asset: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientFlairTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientIcons {
    pub icon_id: i32,
    pub icon_path: Option<String>,
    pub icon_name: Option<String>,
}

impl FromCdClient for CdClientIcons {
    const TABLE: &'static str = "CdClientIcons";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            icon_id: row.get(0)?,
            icon_path: trim_and_nullify(row.get(1)?),
            icon_name: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for CdClientIcons {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.icon_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientInventoryComponent {
    pub id: i32,
    pub itemid: i32,
    pub count: i32,
    pub equip: bool,
}

impl FromCdClient for CdClientInventoryComponent {
    const TABLE: &'static str = "CdClientInventoryComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            itemid: row.get(1)?,
            count: row.get(2)?,
            equip: row.get(3)?,
        })
    }
}

// impl HasKey for CdClientInventoryComponent {
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
pub struct CdClientItemComponent {
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

fn parse_currency_costs(input: Option<String>) -> Option<Vec<(i32, i32)>> {
    let mut elements = vec![];
    for pair in input?.split(',') {
        let (id, count) = pair.trim().split_once(':')?;
        elements.push((id.parse().ok()?, count.parse().ok()?))
    }
    Some(elements)
}

impl FromCdClient for CdClientItemComponent {
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

impl HasKey for CdClientItemComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientItemEggData {
    pub id: i32,
    pub chassie_type_id: i32,
}

impl FromCdClient for CdClientItemEggData {
    const TABLE: &'static str = "CdClientItemEggData";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            chassie_type_id: row.get(1)?,
        })
    }
}

impl HasKey for CdClientItemEggData {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientItemFoodData {
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

impl FromCdClient for CdClientItemFoodData {
    const TABLE: &'static str = "CdClientItemFoodData";

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

impl HasKey for CdClientItemFoodData {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientItemSetSkills {
    pub skill_set_id: i32,
    pub skill_id: i32,
    pub skill_cast_type: i32,
}

impl FromCdClient for CdClientItemSetSkills {
    const TABLE: &'static str = "CdClientItemSetSkills";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            skill_set_id: row.get(0)?,
            skill_id: row.get(1)?,
            skill_cast_type: row.get(2)?,
        })
    }
}

impl HasGroupKey for CdClientItemSetSkills {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.skill_set_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientItemSets {
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

impl FromCdClient for CdClientItemSets {
    const TABLE: &'static str = "CdClientItemSets";

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

impl HasKey for CdClientItemSets {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.set_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientJetPackPadComponent {
    pub id: i32,
    pub x_distance: f64,
    pub y_distance: f64,
    pub warn_distance: f64,
    pub lot_blocker: Option<i32>,
    pub lot_warning_volume: Option<i32>,
}

impl FromCdClient for CdClientJetPackPadComponent {
    const TABLE: &'static str = "CdClientJetPackPadComponent";

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

impl HasKey for CdClientJetPackPadComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientLuPExhibitComponent {
    pub id: i32,
    pub min_xz: f64,
    pub max_xz: f64,
    pub max_y: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub offset_z: f64,
}

impl FromCdClient for CdClientLuPExhibitComponent {
    const TABLE: &'static str = "LUPCdClientExhibitComponent";

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

impl HasKey for CdClientLuPExhibitComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientLuPExhibitModelData {
    pub lot: i32,
    pub min_xz: f64,
    pub max_xz: f64,
    pub max_y: f64,
    pub description: String,
    pub owner: String,
}

impl FromCdClient for CdClientLuPExhibitModelData {
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

impl HasKey for CdClientLuPExhibitModelData {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.lot
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientLuPZoneIDs {
    pub zone_id: i32,
}

impl FromCdClient for CdClientLuPZoneIDs {
    const TABLE: &'static str = "LUPZoneIDs";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            zone_id: row.get(0)?,
        })
    }
}

impl HasKey for CdClientLuPZoneIDs {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.zone_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientLanguageType {
    pub language_id: i32,
    pub language_description: String,
}

impl FromCdClient for CdClientLanguageType {
    const TABLE: &'static str = "CdClientLanguageType";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            language_id: row.get(0)?,
            language_description: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientLanguageType {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.language_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientLevelProgressionLookup {
    pub id: i32,
    pub required_uscore: i32,
    pub behavior_effect: Option<String>,
}

impl FromCdClient for CdClientLevelProgressionLookup {
    const TABLE: &'static str = "CdClientLevelProgressionLookup";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            required_uscore: row.get(1)?,
            behavior_effect: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for CdClientLevelProgressionLookup {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientLootMatrix {
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

impl FromCdClient for CdClientLootMatrix {
    const TABLE: &'static str = "CdClientLootMatrix";

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

impl HasGroupKey for CdClientLootMatrix {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.loot_matrix_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientLootMatrixIndex {
    pub loot_matrix_index: i32,
    pub in_npc_editor: bool,
}

impl FromCdClient for CdClientLootMatrixIndex {
    const TABLE: &'static str = "CdClientLootMatrixIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            loot_matrix_index: row.get(0)?,
            in_npc_editor: row.get(1)?,
        })
    }
}

impl HasKey for CdClientLootMatrixIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.loot_matrix_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientLootTable {
    pub itemid: i32,
    pub loot_table_index: i32,
    pub id: i32,
    pub mission_drop: bool,
    pub sort_priority: i32,
}

impl FromCdClient for CdClientLootTable {
    const TABLE: &'static str = "CdClientLootTable";

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

// multiple groupings, but i will do it by loot_table_index
impl HasGroupKey for CdClientLootTable {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.loot_table_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientLootTableIndex {
    pub loot_table_index: i32,
}

impl FromCdClient for CdClientLootTableIndex {
    const TABLE: &'static str = "CdClientLootTableIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            loot_table_index: row.get(0)?,
        })
    }
}

impl HasKey for CdClientLootTableIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.loot_table_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMinifigComponent {
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

impl FromCdClient for CdClientMinifigComponent {
    const TABLE: &'static str = "CdClientMinifigComponent";

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

impl HasKey for CdClientMinifigComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMinifigDecalsEyebrows {
    pub id: i32,
    pub high_path: String,
    pub low_path: String,
    pub character_create_valid: bool,
    pub male: bool,
    pub female: bool,
}

impl FromCdClient for CdClientMinifigDecalsEyebrows {
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

impl HasKey for CdClientMinifigDecalsEyebrows {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMinifigDecalsEyes {
    pub id: i32,
    pub high_path: String,
    pub low_path: String,
    pub character_create_valid: bool,
    pub male: bool,
    pub female: bool,
}

impl FromCdClient for CdClientMinifigDecalsEyes {
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

impl HasKey for CdClientMinifigDecalsEyes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMinifigDecalsLegs {
    pub id: i32,
    pub high_path: String,
}

impl FromCdClient for CdClientMinifigDecalsLegs {
    const TABLE: &'static str = "MinifigDecals_Legs";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            high_path: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientMinifigDecalsLegs {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMinifigDecalsMouths {
    pub id: i32,
    pub high_path: String,
    pub low_path: String,
    pub character_create_valid: bool,
    pub male: bool,
    pub female: bool,
}

impl FromCdClient for CdClientMinifigDecalsMouths {
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

impl HasKey for CdClientMinifigDecalsMouths {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMinifigDecalsTorsos {
    pub id: i32,
    pub high_path: String,
    pub character_create_valid: bool,
    pub male: bool,
    pub female: bool,
}

impl FromCdClient for CdClientMinifigDecalsTorsos {
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

impl HasKey for CdClientMinifigDecalsTorsos {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMissionEmail {
    pub id: i32,
    pub message_type: i32,
    pub notification_group: i32,
    pub mission_id: i32,
    pub attachment_lot: Option<i32>,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for CdClientMissionEmail {
    const TABLE: &'static str = "CdClientMissionEmail";

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

impl HasKey for CdClientMissionEmail {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMissionNPCComponent {
    pub id: i32,
    pub mission_id: i32,
    pub offers_mission: bool,
    pub accepts_mission: bool,
    pub gate_version: Option<String>,
}

impl FromCdClient for CdClientMissionNPCComponent {
    const TABLE: &'static str = "CdClientMissionNPCComponent";

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

// impl HasKey for CdClientMissionNPCComponent {
//     type Key = i32;
//
//     // multiple groupings
//     fn get_key(&self) -> &Self::Key {
//         &self.id
//     }
// }

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMissionTasks {
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

impl FromCdClient for CdClientMissionTasks {
    const TABLE: &'static str = "CdClientMissionTasks";

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

impl HasGroupKey for CdClientMissionTasks {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMissionText {
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

impl FromCdClient for CdClientMissionText {
    const TABLE: &'static str = "CdClientMissionText";

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

impl HasKey for CdClientMissionText {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMissions {
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

fn parse_mission_prereqs(input: Option<String>) -> Option<Vec<MissionPreReqType>> {
    let mut elements = vec![];
    for value in input?.split(',') {
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

impl FromCdClient for CdClientMissions {
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

impl HasKey for CdClientMissions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientModelBehavior {
    pub id: i32,
    pub definition_xmlfilename: String,
}

impl FromCdClient for CdClientModelBehavior {
    const TABLE: &'static str = "CdClientModelBehavior";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            definition_xmlfilename: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientModelBehavior {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientModularBuildComponent {
    pub id: i32,
    pub build_type: i32,
    pub xml: String,
    pub created_lot: i32,
    pub created_physics_id: i32,
    pub audio_event_guid_snap: String,
    pub audio_event_guid_complete: Option<String>,
    pub audio_event_guid_present: Option<String>,
}

impl FromCdClient for CdClientModularBuildComponent {
    const TABLE: &'static str = "CdClientModularBuildComponent";

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

impl HasKey for CdClientModularBuildComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientModuleComponent {
    pub id: i32,
    pub part_code: i32,
    pub build_type: i32,
    pub xml: String,
    pub primary_sound_guid: Option<String>,
    pub assembled_effect_id: Option<i32>,
}

impl FromCdClient for CdClientModuleComponent {
    const TABLE: &'static str = "CdClientModuleComponent";

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

impl HasKey for CdClientModuleComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMotionFX {
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

impl FromCdClient for CdClientMotionFX {
    const TABLE: &'static str = "CdClientMotionFX";

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

impl HasKey for CdClientMotionFX {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMovementAIComponent {
    pub id: i32,
    pub movement_type: String,
    pub wander_chance: f64,
    pub wander_delay_min: f64,
    pub wander_delay_max: f64,
    pub wander_speed: f64,
    pub wander_radius: f64,
    pub attached_path: Option<String>,
}

impl FromCdClient for CdClientMovementAIComponent {
    const TABLE: &'static str = "CdClientMovementAIComponent";

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

impl HasKey for CdClientMovementAIComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMovingPlatforms {
    pub id: i32,
    pub platform_is_simple_mover: bool,
    pub platform_move_x: f64,
    pub platform_move_y: f64,
    pub platform_move_z: f64,
    pub platform_move_time: f64,
    pub platform_start_at_end: bool,
    pub description: String,
}

impl FromCdClient for CdClientMovingPlatforms {
    const TABLE: &'static str = "CdClientMovingPlatforms";

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

impl HasKey for CdClientMovingPlatforms {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientNpcIcons {
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

impl FromCdClient for CdClientNpcIcons {
    const TABLE: &'static str = "CdClientNpcIcons";

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

impl HasKey for CdClientNpcIcons {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientObjectBehaviorXREF {
    pub lot: i32,
    pub behavior_id1: i64,
    pub behavior_id2: i64,
    pub behavior_id3: i64,
    pub behavior_id4: i64,
    pub behavior_id5: i64,
    pub r#type: i32,
}

impl FromCdClient for CdClientObjectBehaviorXREF {
    const TABLE: &'static str = "CdClientObjectBehaviorXREF";

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

impl HasKey for CdClientObjectBehaviorXREF {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.lot
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientObjectBehaviors {
    pub behavior_id: i64,
    pub xmldata: String,
}

impl FromCdClient for CdClientObjectBehaviors {
    const TABLE: &'static str = "CdClientObjectBehaviors";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            behavior_id: row.get(0)?,
            xmldata: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientObjectBehaviors {
    type Key = i64;

    fn get_key(&self) -> &Self::Key {
        &self.behavior_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientObjectSkills {
    pub object_template: i32,
    pub skill_id: i32,
    pub cast_on_type: Option<i32>,
    pub aicombat_weight: Option<i32>,
}

impl FromCdClient for CdClientObjectSkills {
    const TABLE: &'static str = "CdClientObjectSkills";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            object_template: row.get(0)?,
            skill_id: row.get(1)?,
            cast_on_type: row.get(2)?,
            aicombat_weight: row.get(3)?,
        })
    }
}

impl HasGroupKey for CdClientObjectSkills {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.object_template
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientObjects {
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

impl FromCdClient for CdClientObjects {
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
impl HasKey for CdClientObjects {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPackageComponent {
    pub id: i32,
    pub loot_matrix_index: i32,
    pub package_type: i32,
}

impl FromCdClient for CdClientPackageComponent {
    const TABLE: &'static str = "PackageComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            loot_matrix_index: row.get(1)?,
            package_type: row.get(2)?,
        })
    }
}

impl HasKey for CdClientPackageComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPetAbilities {
    pub id: i32,
    pub ability_name: String,
    pub imagination_cost: i32,
    pub loc_status: i32,
}

impl FromCdClient for CdClientPetAbilities {
    const TABLE: &'static str = "CdClientPetAbilities";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            ability_name: trim_to_string(row.get(1)?),
            imagination_cost: row.get(2)?,
            loc_status: row.get(3)?,
        })
    }
}

impl HasKey for CdClientPetAbilities {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPetComponent {
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

impl FromCdClient for CdClientPetComponent {
    const TABLE: &'static str = "CdClientPetComponent";

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

impl HasKey for CdClientPetComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPetNestComponent {
    pub id: i32,
    pub elemental_type: i32,
}

impl FromCdClient for CdClientPetNestComponent {
    const TABLE: &'static str = "CdClientPetNestComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            elemental_type: row.get(1)?,
        })
    }
}

impl HasKey for CdClientPetNestComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPhysicsComponent {
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

impl FromCdClient for CdClientPhysicsComponent {
    const TABLE: &'static str = "CdClientPhysicsComponent";

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

impl HasKey for CdClientPhysicsComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPlayerFlags {
    pub id: i32,
    pub session_only: bool,
    pub only_set_by_server: bool,
    pub session_zone_only: bool,
}

impl FromCdClient for CdClientPlayerFlags {
    const TABLE: &'static str = "CdClientPlayerFlags";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            session_only: row.get(1)?,
            only_set_by_server: row.get(2)?,
            session_zone_only: row.get(3)?,
        })
    }
}

impl HasKey for CdClientPlayerFlags {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPlayerStatistics {
    pub stat_id: i32,
    pub sort_order: Option<i32>,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for CdClientPlayerStatistics {
    const TABLE: &'static str = "CdClientPlayerStatistics";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            stat_id: row.get(0)?,
            sort_order: row.get(1)?,
            loc_status: row.get(2)?,
            gate_version: trim_and_nullify(row.get(3)?),
        })
    }
}

impl HasKey for CdClientPlayerStatistics {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.stat_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPreconditions {
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

impl FromCdClient for CdClientPreconditions {
    const TABLE: &'static str = "CdClientPreconditions";

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

impl HasKey for CdClientPreconditions {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPropertyEntranceComponent {
    pub id: i32,
    pub map_id: i32,
    pub property_name: String,
    pub is_on_property: bool,
    pub group_type: Option<String>,
}

impl FromCdClient for CdClientPropertyEntranceComponent {
    const TABLE: &'static str = "CdClientPropertyEntranceComponent";

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

impl HasKey for CdClientPropertyEntranceComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPropertyTemplate {
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

impl FromCdClient for CdClientPropertyTemplate {
    const TABLE: &'static str = "CdClientPropertyTemplate";

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

impl HasKey for CdClientPropertyTemplate {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientProximityMonitorComponent {
    pub id: i32,
    pub proximities: Vec<i32>,
    pub load_on_client: bool,
    pub load_on_server: bool,
}

impl FromCdClient for CdClientProximityMonitorComponent {
    const TABLE: &'static str = "CdClientProximityMonitorComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            proximities: parse_required_comma_list(row.get(1)?),
            load_on_client: row.get(2)?,
            load_on_server: row.get(3)?,
        })
    }
}

impl HasKey for CdClientProximityMonitorComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientProximityTypes {
    pub id: i32,
    pub name: String,
    pub radius: i32,
    pub collision_group: i32,
    pub passive_checks: bool,
    pub icon_id: i32,
    pub load_on_client: bool,
    pub load_on_server: bool,
}

impl FromCdClient for CdClientProximityTypes {
    const TABLE: &'static str = "CdClientProximityTypes";

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

impl HasKey for CdClientProximityTypes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRacingModuleComponent {
    pub id: i32,
    pub top_speed: Option<f64>,
    pub acceleration: Option<f64>,
    pub handling: Option<f64>,
    pub stability: Option<f64>,
    pub imagination: Option<f64>,
}

impl FromCdClient for CdClientRacingModuleComponent {
    const TABLE: &'static str = "CdClientRacingModuleComponent";

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

impl HasKey for CdClientRacingModuleComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRailActivatorComponent {
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

impl FromCdClient for CdClientRailActivatorComponent {
    const TABLE: &'static str = "CdClientRailActivatorComponent";

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

impl HasKey for CdClientRailActivatorComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRarityTable {
    pub id: i32,
    pub randmax: f64,
    pub rarity: i32,
    pub rarity_table_index: i32,
}

impl FromCdClient for CdClientRarityTable {
    const TABLE: &'static str = "CdClientRarityTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            randmax: row.get(1)?,
            rarity: row.get(2)?,
            rarity_table_index: row.get(3)?,
        })
    }
}

impl HasGroupKey for CdClientRarityTable {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.rarity_table_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRarityTableIndex {
    pub rarity_table_index: i32,
}

impl FromCdClient for CdClientRarityTableIndex {
    const TABLE: &'static str = "CdClientRarityTableIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            rarity_table_index: row.get(0)?,
        })
    }
}

impl HasKey for CdClientRarityTableIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.rarity_table_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRebuildComponent {
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

impl FromCdClient for CdClientRebuildComponent {
    const TABLE: &'static str = "CdClientRebuildComponent";

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

impl HasKey for CdClientRebuildComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRebuildSections {
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

impl FromCdClient for CdClientRebuildSections {
    const TABLE: &'static str = "CdClientRebuildSections";

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

impl HasKey for CdClientRebuildSections {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientReleaseVersion {
    pub release_version: String,
    pub release_date: i64,
}

impl FromCdClient for CdClientReleaseVersion {
    const TABLE: &'static str = "Release_Version";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            release_version: trim_to_string(row.get(0)?),
            release_date: row.get(1)?,
        })
    }
}

impl HasKey for CdClientReleaseVersion {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.release_version
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRenderComponent {
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

impl FromCdClient for CdClientRenderComponent {
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

impl HasKey for CdClientRenderComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRenderComponentFlash {
    pub id: i32,
    pub interactive: bool,
    pub animated: bool,
    pub node_name: String,
    pub flash_path: String,
    pub element_name: Option<String>,
    pub uid: i32,
}

impl FromCdClient for CdClientRenderComponentFlash {
    const TABLE: &'static str = "CdClientRenderComponentFlash";

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

impl HasGroupKey for CdClientRenderComponentFlash {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRenderComponentWrapper {
    pub id: i32,
    pub default_wrapper_asset: String,
}

impl FromCdClient for CdClientRenderComponentWrapper {
    const TABLE: &'static str = "CdClientRenderComponentWrapper";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            default_wrapper_asset: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientRenderComponentWrapper {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRenderIconAssets {
    pub id: i32,
    pub icon_asset: Option<String>,
    pub blank_column: Option<String>,
}

impl FromCdClient for CdClientRenderIconAssets {
    const TABLE: &'static str = "CdClientRenderIconAssets";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            icon_asset: trim_and_nullify(row.get(1)?),
            blank_column: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for CdClientRenderIconAssets {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientReputationRewards {
    pub rep_level: i32,
    pub sublevel: i32,
    pub reputation: f64,
}

impl FromCdClient for CdClientReputationRewards {
    const TABLE: &'static str = "CdClientReputationRewards";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            rep_level: row.get(0)?,
            sublevel: row.get(1)?,
            reputation: row.get(2)?,
        })
    }
}

impl HasKey for CdClientReputationRewards {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.rep_level
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRewardCodes {
    pub id: i32,
    pub code: String,
    pub attachment_lot: Option<i32>,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for CdClientRewardCodes {
    const TABLE: &'static str = "CdClientRewardCodes";

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

impl HasKey for CdClientRewardCodes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRewards {
    pub id: i32,
    pub level_id: i32,
    pub mission_id: Option<i32>,
    pub reward_type: i32,
    pub value: i32,
    pub count: Option<i32>,
}

impl FromCdClient for CdClientRewards {
    const TABLE: &'static str = "CdClientRewards";

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

impl HasKey for CdClientRewards {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientRocketLaunchpadControlComponent {
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

impl FromCdClient for CdClientRocketLaunchpadControlComponent {
    const TABLE: &'static str = "CdClientRocketLaunchpadControlComponent";

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

impl HasKey for CdClientRocketLaunchpadControlComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSceneTable {
    pub scene_id: i32,
    pub scene_name: String,
}

impl FromCdClient for CdClientSceneTable {
    const TABLE: &'static str = "CdClientSceneTable";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            scene_id: row.get(0)?,
            scene_name: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientSceneTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.scene_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientScriptComponent {
    pub id: i32,
    pub script_name: Option<String>,
    pub client_script_name: Option<String>,
}

impl FromCdClient for CdClientScriptComponent {
    const TABLE: &'static str = "CdClientScriptComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            script_name: trim_and_nullify(row.get(1)?),
            client_script_name: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for CdClientScriptComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSkillBehavior {
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

impl FromCdClient for CdClientSkillBehavior {
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

impl HasKey for CdClientSkillBehavior {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.skill_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSmashableChain {
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

impl FromCdClient for CdClientSmashableChain {
    const TABLE: &'static str = "CdClientSmashableChain";

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

impl HasGroupKey for CdClientSmashableChain {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.chain_index
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSmashableChainIndex {
    pub id: i32,
    pub target_group: String,
    pub description: String,
    pub continuous: i32,
}

impl FromCdClient for CdClientSmashableChainIndex {
    const TABLE: &'static str = "CdClientSmashableChainIndex";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            target_group: trim_to_string(row.get(1)?),
            description: trim_to_string(row.get(2)?),
            continuous: row.get(3)?,
        })
    }
}

impl HasKey for CdClientSmashableChainIndex {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSmashableComponent {
    pub id: i32,
    pub loot_matrix_index: i32,
}

impl FromCdClient for CdClientSmashableComponent {
    const TABLE: &'static str = "CdClientSmashableComponent";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            loot_matrix_index: row.get(1)?,
        })
    }
}

impl HasKey for CdClientSmashableComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSmashableElements {
    pub element_id: i32,
    pub drop_weight: i32,
}

impl FromCdClient for CdClientSmashableElements {
    const TABLE: &'static str = "CdClientSmashableElements";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            element_id: row.get(0)?,
            drop_weight: row.get(1)?,
        })
    }
}

impl HasKey for CdClientSmashableElements {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.element_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSpeedchatMenu {
    pub id: i32,
    pub parent_id: Option<i32>,
    pub emote_id: Option<i32>,
    pub image_name: Option<String>,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for CdClientSpeedchatMenu {
    const TABLE: &'static str = "CdClientSpeedchatMenu";

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

impl HasKey for CdClientSpeedchatMenu {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSubscriptionPricing {
    pub id: i32,
    pub country_code: String,
    pub monthly_fee_gold: String,
    pub monthly_fee_silver: String,
    pub monthly_fee_bronze: String,
    pub monetary_symbol: i32,
    pub symbol_is_appended: bool,
}

impl FromCdClient for CdClientSubscriptionPricing {
    const TABLE: &'static str = "CdClientSubscriptionPricing";

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

impl HasKey for CdClientSubscriptionPricing {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSurfaceType {
    pub surface_type: i32,
    pub footstep_ndaudio_meta_event_set_name: Option<String>,
}

impl FromCdClient for CdClientSurfaceType {
    const TABLE: &'static str = "CdClientSurfaceType";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            surface_type: row.get(0)?,
            footstep_ndaudio_meta_event_set_name: trim_and_nullify(row.get(1)?),
        })
    }
}

impl HasKey for CdClientSurfaceType {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.surface_type
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientTamingBuildPuzzles {
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

impl FromCdClient for CdClientTamingBuildPuzzles {
    const TABLE: &'static str = "CdClientTamingBuildPuzzles";

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

impl HasKey for CdClientTamingBuildPuzzles {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientTextDescription {
    pub text_id: i32,
    pub test_description: String,
}

impl FromCdClient for CdClientTextDescription {
    const TABLE: &'static str = "CdClientTextDescription";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            text_id: row.get(0)?,
            test_description: trim_to_string(row.get(1)?),
        })
    }
}

impl HasKey for CdClientTextDescription {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.text_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientTextLanguage {
    pub text_id: i32,
    pub language_id: i32,
    pub text: String,
}

impl FromCdClient for CdClientTextLanguage {
    const TABLE: &'static str = "CdClientTextLanguage";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            text_id: row.get(0)?,
            language_id: row.get(1)?,
            text: trim_to_string(row.get(2)?),
        })
    }
}

impl HasKey for CdClientTextLanguage {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.text_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientTrailEffects {
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

impl FromCdClient for CdClientTrailEffects {
    const TABLE: &'static str = "CdClientTrailEffects";

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

impl HasKey for CdClientTrailEffects {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.trail_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientUgBehaviorSounds {
    pub id: i32,
    pub guid: String,
    pub localize: bool,
    pub loc_status: i32,
    pub gate_version: Option<String>,
}

impl FromCdClient for CdClientUgBehaviorSounds {
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

impl HasKey for CdClientUgBehaviorSounds {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientVehiclePhysics {
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

impl FromCdClient for CdClientVehiclePhysics {
    const TABLE: &'static str = "CdClientVehiclePhysics";

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

impl HasKey for CdClientVehiclePhysics {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientVehicleStatMap {
    pub id: i32,
    pub module_stat: String,
    pub havok_stat: String,
    pub havok_change_per_module_stat: f64,
}

impl FromCdClient for CdClientVehicleStatMap {
    const TABLE: &'static str = "CdClientVehicleStatMap";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            module_stat: trim_to_string(row.get(1)?),
            havok_stat: trim_to_string(row.get(2)?),
            havok_change_per_module_stat: row.get(3)?,
        })
    }
}

impl HasGroupKey for CdClientVehicleStatMap {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientVendorComponent {
    pub id: i32,
    pub buy_scalar: f64,
    pub sell_scalar: f64,
    pub refresh_time_seconds: f64,
    pub loot_matrix_index: i32,
}

impl FromCdClient for CdClientVendorComponent {
    const TABLE: &'static str = "CdClientVendorComponent";

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

impl HasKey for CdClientVendorComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientWhatsCoolItemSpotlight {
    pub id: i32,
    pub item_id: i32,
    pub localize: bool,
    pub gate_version: Option<String>,
    pub loc_status: i32,
}

impl FromCdClient for CdClientWhatsCoolItemSpotlight {
    const TABLE: &'static str = "CdClientWhatsCoolItemSpotlight";

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

impl HasKey for CdClientWhatsCoolItemSpotlight {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientWhatsCoolNewsAndTips {
    pub id: i32,
    pub icon_id: Option<i32>,
    pub r#type: i32,
    pub localize: bool,
    pub gate_version: Option<String>,
    pub loc_status: i32,
}

impl FromCdClient for CdClientWhatsCoolNewsAndTips {
    const TABLE: &'static str = "CdClientWhatsCoolNewsAndTips";

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

impl HasKey for CdClientWhatsCoolNewsAndTips {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientWorldConfig {
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

impl FromCdClient for CdClientWorldConfig {
    const TABLE: &'static str = "CdClientWorldConfig";

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

impl HasKey for CdClientWorldConfig {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.world_config_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientZoneLoadingTips {
    pub id: i32,
    pub zoneid: i32,
    pub imagelocation: String,
    pub localize: bool,
    pub gate_version: String,
    pub loc_status: i32,
    pub weight: i32,
    pub target_version: Option<String>,
}

impl FromCdClient for CdClientZoneLoadingTips {
    const TABLE: &'static str = "CdClientZoneLoadingTips";

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

impl HasKey for CdClientZoneLoadingTips {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientZoneSummary {
    pub zone_id: i32,
    pub r#type: i32,
    pub value: Option<i32>,
    pub unique_id: i32,
}

impl FromCdClient for CdClientZoneSummary {
    const TABLE: &'static str = "CdClientZoneSummary";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            zone_id: row.get(0)?,
            r#type: row.get(1)?,
            value: row.get(2)?,
            unique_id: row.get(3)?,
        })
    }
}

impl HasGroupKey for CdClientZoneSummary {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.zone_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientZoneTable {
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

impl FromCdClient for CdClientZoneTable {
    const TABLE: &'static str = "CdClientZoneTable";

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

impl HasKey for CdClientZoneTable {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.zone_id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientBrickAttributes {
    pub id: i32,
    pub icon_asset: String,
    pub display_order: i32,
    pub loc_status: i32,
}

impl FromCdClient for CdClientBrickAttributes {
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

impl HasKey for CdClientBrickAttributes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientDtproperties {
    pub id: i32,
    pub objectid: i32,
    pub property: String,
    pub value: String,
    pub uvalue: String,
    pub lvalue: String,
    pub version: i32,
}

impl FromCdClient for CdClientDtproperties {
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

impl HasKey for CdClientDtproperties {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMapAnimationPriorities {
    pub id: i32,
    pub name: String,
    pub priority: f64,
}

impl FromCdClient for CdClientMapAnimationPriorities {
    const TABLE: &'static str = "mapAnimationPriorities";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            name: trim_to_string(row.get(1)?),
            priority: row.get(2)?,
        })
    }
}

impl HasKey for CdClientMapAnimationPriorities {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMapAssetType {
    pub id: i32,
    pub label: String,
    pub pathdir: String,
    pub typelabel: String,
}

impl FromCdClient for CdClientMapAssetType {
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

impl HasKey for CdClientMapAssetType {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMapIcon {
    pub lot: i32,
    pub icon_id: i32,
    pub icon_state: i32,
}

impl FromCdClient for CdClientMapIcon {
    const TABLE: &'static str = "mapIcon";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            lot: row.get(0)?,
            icon_id: row.get(1)?,
            icon_state: row.get(2)?,
        })
    }
}

impl HasGroupKey for CdClientMapIcon {
    type Key = i32;

    fn get_group_key(&self) -> &Self::Key {
        &self.lot
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMapItemTypes {
    pub id: i32,
    pub description: String,
    pub equip_location: Option<String>,
}

impl FromCdClient for CdClientMapItemTypes {
    const TABLE: &'static str = "mapItemTypes";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            description: trim_to_string(row.get(1)?),
            equip_location: trim_and_nullify(row.get(2)?),
        })
    }
}

impl HasKey for CdClientMapItemTypes {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMapRenderEffects {
    pub id: i32,
    pub game_id: i32,
    pub description: String,
}

impl FromCdClient for CdClientMapRenderEffects {
    const TABLE: &'static str = "mapRenderEffects";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            game_id: row.get(1)?,
            description: trim_to_string(row.get(2)?),
        })
    }
}

impl HasKey for CdClientMapRenderEffects {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMapShaders {
    pub id: i32,
    pub label: String,
    pub game_value: i32,
    pub priority: Option<i32>,
}

impl FromCdClient for CdClientMapShaders {
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

impl HasKey for CdClientMapShaders {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMapTextureResource {
    pub id: i32,
    pub texturepath: String,
    pub surface_type: i32,
}

impl FromCdClient for CdClientMapTextureResource {
    const TABLE: &'static str = "mapTextureResource";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            texturepath: trim_to_string(row.get(1)?),
            surface_type: row.get(2)?,
        })
    }
}

impl HasKey for CdClientMapTextureResource {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMapBlueprintCategory {
    pub id: i32,
    pub description: String,
    pub enabled: bool,
}

impl FromCdClient for CdClientMapBlueprintCategory {
    const TABLE: &'static str = "map_BlueprintCategory";

    fn query_map(row: &rusqlite::Row<'_>) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get(0)?,
            description: trim_to_string(row.get(1)?),
            enabled: row.get(2)?,
        })
    }
}

impl HasKey for CdClientMapBlueprintCategory {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSysdiagrams {
    pub name: String,
    pub principal_id: i32,
    pub diagram_id: i32,
    pub version: i32,
    pub definition: String,
}

impl FromCdClient for CdClientSysdiagrams {
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

impl HasKey for CdClientSysdiagrams {
    type Key = String;

    fn get_key(&self) -> &Self::Key {
        &self.name
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientPossessableComponent {
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

impl FromCdClient for CdClientPossessableComponent {
    const TABLE: &'static str = "CdClientPossessableComponent";

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

impl HasKey for CdClientPossessableComponent {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientSkillSets {
    pub id: i32,
    pub slot1: Option<i32>,
    pub slot2: Option<i32>,
    pub slot3: Option<i32>,
    pub slot4: Option<i32>,
    pub priority: f64,
    pub description: Option<String>,
}

impl FromCdClient for CdClientSkillSets {
    const TABLE: &'static str = "CdClientSkillSets";

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

impl HasKey for CdClientSkillSets {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CdClientMapFacesAndHair {
    pub id: i32,
    pub eyes: i32,
    pub eyebrows: i32,
    pub mouths: i32,
    pub haircolor: i32,
    pub hairstyle: i32,
}

impl FromCdClient for CdClientMapFacesAndHair {
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

impl HasKey for CdClientMapFacesAndHair {
    type Key = i32;

    fn get_key(&self) -> &Self::Key {
        &self.id
    }
}
