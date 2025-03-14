use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct AppAccessResponse {
    pub access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct AdsAccountsResponse {
    pub data: Vec<AdAccount>,
    pub paging: Paging,
}

#[derive(Deserialize, Debug)]
pub struct AdAccount {
    pub account_id: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Paging {
    pub cursors: Cursors,
}

#[derive(Deserialize, Debug)]
pub struct Cursors {
    pub before: String,
    pub after: String,
}

impl AdsAccountsResponse {
    pub fn display(&self) {
        println!("Displaying Ads Accounts:");

        for account in &self.data {
            println!("Ad Account ID: {}", account.account_id);
            println!("Ad Account Internal ID: {}", account.id);
        }
        println!("\nPaging Information:");
        println!("Before Cursor: {}", self.paging.cursors.before);
        println!("After Cursor: {}", self.paging.cursors.after);
    }
}

#[derive(Deserialize, Debug)]
pub struct CampaignsResponse {
    pub data: Vec<Campaign>,
    pub paging: Paging,
}

#[derive(Deserialize, Debug)]
pub struct Campaign {
    pub id: Option<String>,                                // Campaign's ID
    pub account_id: Option<String>, // ID of the ad account that owns this campaign
    pub adlabels: Option<Vec<AdLabel>>, // List of Ad Labels associated with this campaign
    pub bid_strategy: Option<String>, // Bid strategy
    pub boosted_object_id: Option<String>, // The Boosted Object this campaign has associated, if any
    pub brand_lift_studies: Option<Vec<AdStudy>>, // Automated Brand Lift V2 studies for this ad set
    pub budget_rebalance_flag: Option<bool>, // Whether to automatically rebalance budgets daily
    pub budget_remaining: Option<String>,  // Remaining budget
    pub buying_type: Option<String>,       // Buying type (e.g., AUCTION, RESERVED)
    pub campaign_group_active_time: Option<String>, // Internal campaign group active time
    pub can_create_brand_lift_study: Option<bool>, // If we can create a new brand lift study
    pub can_use_spend_cap: Option<bool>,   // Whether the campaign can set the spend cap
    pub configured_status: Option<String>, // Campaign status (ACTIVE, PAUSED, etc.)
    pub created_time: Option<String>,      // Created time
    pub daily_budget: Option<String>,      // Daily budget
    pub effective_status: Option<String>,  // Effective status (ACTIVE, PAUSED, etc.)
    pub has_secondary_skadnetwork_reporting: Option<bool>, // Secondary SKAdNetwork reporting
    pub is_budget_schedule_enabled: Option<bool>, // Whether budget scheduling is enabled
    pub is_skadnetwork_attribution: Option<bool>, // Whether the campaign includes SKAdNetwork attribution
    pub issues_info: Option<Vec<AdCampaignIssuesInfo>>, // Issues preventing campaign delivery
    pub last_budget_toggling_time: Option<String>, // Last budget toggling time
    pub lifetime_budget: Option<String>,          // Lifetime budget
    pub name: Option<String>,                     // Campaign's name
    pub objective: Option<String>,                // Campaign's objective
    pub pacing_type: Option<Vec<String>>,         // Defines pacing type (e.g., "standard")
    pub primary_attribution: Option<String>,      // Primary attribution
    pub promoted_object: Option<AdPromotedObject>, // The object this campaign is promoting
    pub smart_promotion_type: Option<String>,     // Smart promotion type
    pub source_campaign: Option<Box<Campaign>>,   // Source campaign (if copied)
    pub source_campaign_id: Option<String>,       // Source campaign ID (if copied)
    pub special_ad_categories: Option<Vec<String>>, // Special ad categories
    pub special_ad_category: Option<String>, // Campaign's special ad category (e.g., HOUSING, EMPLOYMENT)
    pub special_ad_category_country: Option<Vec<String>>, // Countries for special ad category
    pub spend_cap: Option<String>,           // Spend cap for the campaign
    pub start_time: Option<String>,          // Start time
    pub status: Option<String>,              // Status (ACTIVE, PAUSED, etc.)
    pub stop_time: Option<String>,           // Stop time
    pub topline_id: Option<String>,          // Topline ID
    pub updated_time: Option<String>,        // Updated time
}

#[derive(Deserialize, Debug)]
pub struct AdLabel {
    pub id: String,
    pub account: Option<AdAccount>,
    pub created_time: Option<String>,
    pub name: String,
    pub updated_time: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct AdStudy {
    pub id: String,
    pub business: Option<String>,
    pub canceled_time: Option<String>,
    pub cooldown_start_time: Option<String>,
    pub created_by: Option<String>,
    pub created_time: Option<String>,
    pub description: Option<String>,
    pub end_time: Option<String>,
    pub name: String,
    pub observation_end_time: Option<String>,
    pub results_first_available_date: Option<String>,
    pub start_time: Option<String>,
    pub study_type: Option<String>,
    pub updated_by: Option<String>,
    pub updated_time: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct AdCampaignIssuesInfo {
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
    pub error_summary: Option<String>,
    pub error_type: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AdPromotedObject {
    pub application_id: Option<String>,
    pub boosted_product_set_id: Option<String>,
    pub conversion_goal_id: Option<String>,
    pub custom_conversion_id: Option<String>,
    pub custom_event_str: Option<String>,
    pub custom_event_type: Option<String>,
    pub event_id: Option<String>,
    pub lead_ads_custom_event_str: Option<String>,
    pub lead_ads_custom_event_type: Option<String>,
    pub lead_ads_form_event_source_type: Option<String>,
    pub mcme_conversion_id: Option<String>,
    pub object_store_url: Option<String>,
    pub offer_id: Option<String>,
    pub offline_conversion_data_set_id: Option<String>,
    pub offsite_conversion_event_id: Option<String>,
    pub page_id: Option<String>,
    pub pixel_aggregation_rule: Option<String>,
    pub pixel_id: Option<String>,
    pub pixel_rule: Option<String>,
    pub place_page_set_id: Option<String>,
    pub product_catalog_id: Option<String>,
    pub product_set_id: Option<String>,
    pub retention_days: Option<String>,
    pub value_semantic_type: Option<String>,
    pub variation: Option<String>,
    pub whatsapp_phone_number: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct AdSetResponse {
    pub data: Vec<AdSet>,
    pub paging: Paging,
}

#[derive(Deserialize, Debug)]
pub struct AdSet {
    pub id: Option<String>,
    pub account_id: Option<String>,
    pub adlabels: Option<Vec<AdLabel>>,
    pub adset_schedule: Option<Vec<DayPart>>,
    pub asset_feed_id: Option<String>,
    pub attribution_spec: Option<Vec<String>>, // PLM DOCUMENTATIE
    pub bid_adjustments: Option<AdBidAdjustments>, //PLM DOCUMENTATIE
    pub bid_amount: Option<u32>,
    pub bid_constraints: Option<AdCampaignBidConstraint>, // PLM DOCUMENTATIE
    pub bid_info: Option<HashMap<String, u32>>,
    pub bid_strategy: Option<String>,
    pub billing_event: Option<String>,
    pub brand_safety_config: Option<String>, // PLM DOCUMENTATIE
    pub budget_remaining: Option<String>,
    pub campaign: Option<Campaign>, // PLM
    pub campaign_active_time: Option<String>,
    pub campaign_attribution: Option<String>,
    pub campaign_id: Option<String>,
    pub configured_status: Option<String>,
    pub contextual_bundling_spec: Option<String>, //PLM
    pub created_time: Option<String>,
    pub creative_sequence: Option<Vec<String>>,
    pub daily_budget: Option<String>,
    pub daily_min_spend_target: Option<String>,
    pub daily_spend_cap: Option<String>,
    pub destination_type: Option<String>,
    pub dsa_beneficiary: Option<String>,
    pub dsa_payor: Option<String>,
    pub effective_status: Option<String>,
    pub end_time: Option<String>,
    pub frequency_control_specs: Option<Vec<AdCampaignFrequencyControlSpecs>>,
    pub instagram_user_id: Option<String>,
    pub is_dynamic_creative: Option<bool>,
    pub issues_info: Option<Vec<AdCampaignIssuesInfo>>,
    pub learning_stage_info: Option<AdCampaignLearningStageInfo>,
    pub lifetime_budget: Option<String>,
    pub lifetime_imps: Option<i32>,
    pub lifetime_min_spend_target: Option<String>,
    pub lifetime_spend_cap: Option<String>,
    pub min_budget_spend_percentage: Option<String>,
    pub multi_optimization_goal_weight: Option<String>,
    pub name: Option<String>,
    pub optimization_goal: Option<String>,
    pub optimization_sub_event: Option<String>,
    pub pacing_type: Option<Vec<String>>,
    pub promoted_object: Option<AdPromotedObject>,
    pub recommendations: Option<Vec<AdRecommendation>>,
    pub recurring_budget_semantics: Option<bool>,
    pub regional_regulated_categories: Option<Vec<String>>,
    pub regional_regulation_identities: Option<String>, //PLM
    pub review_feedback: Option<String>,
    pub rf_prediction_id: Option<String>,
    pub source_adset: Option<Box<AdSet>>, //PLM
    pub source_adset_id: Option<String>,
    pub start_time: Option<String>,
    pub status: Option<String>,
    pub targeting: Option<Targeting>, //PLM
    pub targeting_optimization_types: Option<Vec<TargetingOptimizationTypes>>,
    pub time_based_ad_rotation_id_blocks: Option<Vec<Vec<i32>>>,
    pub time_based_ad_rotation_intervals: Option<Vec<u32>>,
    pub updated_time: Option<String>,
    pub use_new_app_click: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DayPart {
    pub start_minute: i32,
    pub end_minute: Option<i32>,
    pub days: Option<Vec<i32>>,
    pub timezone_type: Option<String>, // Optional: "user" or "advertizer"
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdBidAdjustments {
    pub age_range: HashMap<String, f64>,
    pub page_types: Option<String>, // NU SCRIE DOCUMENTATIE
    pub user_groups: Option<String>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AdCampaignBidConstraint;

#[derive(Debug, Deserialize)]
pub struct AdCampaignFrequencyControlSpecs {
    pub event: Option<String>,
    pub interval_days: Option<u32>,
    pub max_frequency: Option<u32>,
}
#[derive(Debug, Deserialize)]
pub struct AdCampaignLearningStageInfo {
    pub attribution_windows: Option<Vec<String>>,
    pub conversions: Option<u32>,
    pub last_sig_edit_ts: Option<i64>,
    pub status: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct AdRecommendation {
    pub recommendation_signature: String,
    pub r#type: String,
    pub object_ids: Option<Vec<String>>,
}
#[derive(Debug, Deserialize)]
pub struct Targeting {
    pub genders: Option<Vec<u8>>, // 1 = males, 2 = females
    pub age_min: Option<u8>, // Min 13, default 18
    pub age_max: Option<u8>,
    pub countries: Option<Vec<String>>, 
    pub regions: Option<Vec<LocationKey>>, 
    pub cities: Option<Vec<CityTargeting>>, 
    pub zips: Option<Vec<LocationKey>>, 
    pub places: Option<Vec<PlaceTargeting>>, 
    pub custom_locations: Option<Vec<CustomLocation>>, 
    pub geo_markets: Option<Vec<LocationKey>>, 
    pub electoral_districts: Option<Vec<LocationKey>>, 
    pub location_types: Option<Vec<String>>, 
    pub country_groups: Option<Vec<String>>, 
    pub interests: Option<Vec<InterestOrBehavior>>, 
    pub behaviors: Option<Vec<InterestOrBehavior>>, 
}

#[derive(Debug, Deserialize)]
pub struct LocationKey {
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct CityTargeting {
    pub key: String, 
    pub radius: u8, 
    pub distance_unit: String,
}

#[derive(Debug, Deserialize)]
pub struct PlaceTargeting {
    pub key: String, 
    pub name: String, 
    pub radius: u8, 
    pub distance_unit: String, 
}

#[derive(Debug, Deserialize)]
pub struct CustomLocation {
    pub latitude: f64, 
    pub longitude: f64, 
    pub name: Option<String>, 
    pub radius: f64, 
    pub distance_unit: Option<String>, 
    pub address_string: Option<String>, 
}

#[derive(Debug, Deserialize)]
pub struct InterestOrBehavior {
    pub id: u64, 
    pub name: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct TargetingOptimizationTypes {
    pub key: Option<String>,
    pub value : Option<i64>,
}

#[derive(Deserialize, Debug)]
pub struct AdResponse {
    pub data: Vec<AdSet>,
    pub paging: Paging,
}

#[derive(Debug,Deserialize)]
pub struct Ad {
    pub id: Option<String>, 
    pub account_id: Option<String>, 
    pub ad_active_time: Option<String>, 
    pub ad_review_feedback: Option<AdgroupReviewFeedback>, 
    pub ad_schedule_end_time: Option<String>, 
    pub ad_schedule_start_time: Option<String>, 
    pub adlabels: Option<Vec<AdLabel>>, 
    pub adset: Option<AdSet>, 
    pub adset_id: Option<String>, 
    pub bid_amount: Option<i32>, 
    pub campaign: Option<Campaign>, 
    pub campaign_id: Option<String>, 
    pub configured_status: Option<String>, // {ACTIVE, PAUSED, DELETED, ARCHIVED}
    pub conversion_domain: Option<String>, 
    pub created_time: Option<String>, 
   // pub creative: Option<AdCreative>, 
   // pub creative_asset_groups_spec: Option<AdCreativeAssetGroupsSpec>, 
    pub effective_status: Option<String>, // {ACTIVE, PAUSED, DELETED, PENDING_REVIEW, etc.}
    pub issues_info: Option<Vec<AdgroupIssuesInfo>>, 
    pub last_updated_by_app_id: Option<String>,
    pub name: Option<String>,
    pub preview_shareable_link: Option<String>,
    pub recommendations: Option<Vec<AdRecommendation>>, 
    pub source_ad: Option<Box<Ad>>, 
    pub source_ad_id: Option<String>, 
    pub status: Option<String>, // {ACTIVE, PAUSED, DELETED, ARCHIVED}
    pub tracking_specs: Option<Vec<ConversionActionQuery>>, 
    pub updated_time: Option<String>, 
}

#[derive(Debug, Deserialize)]
pub struct AdgroupReviewFeedback {
    pub global: Option<HashMap<String, String>>,
    pub placement_specific : Option<AdgroupPlacementSpecificReviewFeedback>,
}
#[derive(Debug, Deserialize)]
pub struct AdgroupPlacementSpecificReviewFeedback {
    pub account_admin: Option<HashMap<String, String>>,
    pub ad: Option<HashMap<String, String>>,
    pub ads_conversion_experiences: Option<HashMap<String, String>>,
    pub b2c: Option<HashMap<String, String>>,
    pub b2c_commerce_unified: Option<HashMap<String, String>>,
    pub bsg: Option<HashMap<String, String>>,
    pub city_community: Option<HashMap<String, String>>,
    pub commerce: Option<HashMap<String, String>>,
    pub compromise: Option<HashMap<String, String>>,
    pub daily_deals: Option<HashMap<String, String>>,
    pub daily_deals_legacy: Option<HashMap<String, String>>,
    pub dpa: Option<HashMap<String, String>>,
    pub dri_copyright: Option<HashMap<String, String>>,
    pub dri_counterfeit: Option<HashMap<String, String>>,
    pub facebook: Option<HashMap<String, String>>,
    pub facebook_pages_live_shopping: Option<HashMap<String, String>>,
    pub independent_work: Option<HashMap<String, String>>,
    pub instagram: Option<HashMap<String, String>>,
    pub instagram_shop: Option<HashMap<String, String>>,
    pub job_search: Option<HashMap<String, String>>,
    pub lead_gen_honeypot: Option<HashMap<String, String>>,
    pub marketplace: Option<HashMap<String, String>>,
    pub marketplace_home_rentals: Option<HashMap<String, String>>,
    pub marketplace_home_sales: Option<HashMap<String, String>>,
    pub marketplace_motors: Option<HashMap<String, String>>,
    pub marketplace_shops: Option<HashMap<String, String>>,
    pub neighborhoods: Option<HashMap<String, String>>,
    pub page_admin: Option<HashMap<String, String>>,
    pub product: Option<HashMap<String, String>>,
    pub product_service: Option<HashMap<String, String>>,
    pub profile: Option<HashMap<String, String>>,
    pub seller: Option<HashMap<String, String>>,
    pub shops: Option<HashMap<String, String>>,
    pub traffic_quality: Option<HashMap<String, String>>,
    pub unified_commerce_content: Option<HashMap<String, String>>,
    pub whatsapp: Option<HashMap<String, String>>,
}
//#[derive(Debug, Deserialize)]
// pub struct AdCreative {
//     pub id: Option<String>,
//     pub account_id: Option<String>,
//     pub actor_id: Option<String>,
//     pub ad_disclaimer_spec: Option<AdCreativeAdDisclaimer>,
//     pub adlabels: Option<Vec<AdLabel>>,
//     pub applink_treatment: Option<String>,
//     pub asset_feed_spec: Option<AdAssetFeedSpec>,
//     pub authorization_category: Option<String>,
//     pub body: Option<String>,
//     pub branded_content: Option<AdCreativeBrandedContentAds>,
//     pub branded_content_sponsor_page_id: Option<String>,
//     pub bundle_folder_id: Option<String>,
//     pub call_to_action_type: Option<String>,
//     pub categorization_criteria: Option<String>,
//     pub category_media_source: Option<String>,
//     pub collaborative_ads_lsb_image_bank_id: Option<String>,
//     pub contextual_multi_ads: Option<AdCreativeContextualMultiAds>,
//     pub creative_sourcing_spec: Option<AdCreativeSourcingSpec>,
//     pub degrees_of_freedom_spec: Option<AdCreativeDegreesOfFreedomSpec>,
//     pub destination_set_id: Option<String>,
//     pub dynamic_ad_voice: Option<String>,
//     pub effective_authorization_category: Option<String>,
//     pub effective_instagram_media_id: Option<String>,
//     pub effective_object_story_id: Option<String>,
//     pub enable_direct_install: Option<bool>,
//     pub enable_launch_instant_app: Option<bool>,
//     pub facebook_branded_content: Option<AdCreativeFacebookBrandedContent>,
//     pub image_crops: Option<AdsImageCrops>,
//     pub image_hash: Option<String>,
//     pub image_url: Option<String>,
//     pub instagram_permalink_url: Option<String>,
//     pub instagram_user_id: Option<String>,
//     pub interactive_components_spec: Option<AdCreativeInteractiveComponentsSpec>,
//     pub link_destination_display_url: Option<String>,
//     pub link_og_id: Option<String>,
//     pub link_url: Option<String>,
//     pub messenger_sponsored_message: Option<String>,
//     pub name: Option<String>,
//     pub object_id: Option<String>,
//     pub object_store_url: Option<String>,
//     pub object_story_id: Option<String>,
//     pub object_story_spec: Option<AdCreativeObjectStorySpec>,
//     pub object_type: Option<String>,
//     pub object_url: Option<String>,
//     pub page_welcome_message: Option<String>,
//     pub photo_album_source_object_story_id: Option<String>,
//     pub place_page_set_id: Option<String>,
//     pub platform_customizations: Option<AdCreativePlatformCustomization>,
//     pub playable_asset_id: Option<String>,
//     pub portrait_customizations: Option<AdCreativePortraitCustomizations>,
//     pub product_data: Option<Vec<AdCreativeProductData>>,
//     pub product_set_id: Option<String>,
//     pub recommender_settings: Option<AdCreativeRecommenderSettings>,
//     pub referral_id: Option<String>,
//     pub regional_regulation_disclaimer_spec: Option<AdCreativeRegionalRegulationDisclaimer>,
//     pub source_instagram_media_id: Option<String>,
//     pub status: Option<String>,
//     pub template_url: Option<String>,
//     pub template_url_spec: Option<AdCreativeTemplateURLSpec>,
//     pub thumbnail_id: Option<String>,
//     pub thumbnail_url: Option<String>,
//     pub title: Option<String>,
//     pub url_tags: Option<String>,
//     pub use_page_actor_override: Option<bool>,
//     pub video_id: Option<String>,
// }
// #[derive(Debug, Deserialize)]
// struct AdCreativeAdDisclaimer{
// pub text:Option<String>,
// pub title:Option<String>,
// pub url:Option<String>,
// }
// #[derive(Debug, Deserialize)]
// pub struct AdAssetFeedSpec {
//     pub ad_formats: Option<Vec<String>>,
//     pub additional_data: Option<AdAssetFeedAdditionalData>,
//     pub app_product_page_id: Option<String>,
//     pub asset_customization_rules: Option<Vec<AdAssetFeedSpecAssetCustomizationRule>>,
//     pub audios: Option<Vec<AdAssetAudios>>,
//     pub autotranslate: Option<Vec<String>>,
//     pub bodies: Option<Vec<AdAssetFeedSpecBody>>,
//     pub call_ads_configuration: Option<AdAssetCallAdsConfigurationFeedSpec>,
//     pub call_to_action_types: Option<Vec<String>>,
//     pub call_to_actions: Option<Vec<AdAssetFeedSpecCallToAction>>,
//     pub captions: Option<Vec<AdAssetFeedSpecCaption>>,
//     pub ctwa_consent_data: Option<Vec<AdAssetCtwaConsentData>>,
//     pub descriptions: Option<Vec<AdAssetFeedSpecDescription>>,
//     pub events: Option<Vec<AdAssetFeedSpecEvents>>,
//     pub groups: Option<Vec<AdAssetFeedSpecGroupRule>>,
//     pub images: Option<Vec<AdAssetFeedSpecImage>>,
//     pub link_urls: Option<Vec<AdAssetFeedSpecLinkURL>>,
//     pub message_extensions: Option<Vec<AdAssetMessageExtensions>>,
//     pub onsite_destinations: Option<Vec<AdAssetOnsiteDestinations>>,
//     pub optimization_type: Option<String>,
//     pub promotional_metadata: Option<AdAssetPromotionalMetadata>,
//     pub reasons_to_shop: Option<bool>,
//     pub shops_bundle: Option<bool>,
//     pub titles: Option<Vec<AdAssetFeedSpecTitle>>,
//     pub upcoming_events: Option<Vec<AdAssetUpcomingEvents>>,
//     pub videos: Option<Vec<AdAssetFeedSpecVideo>>,
// }

// #[derive(Debug, Deserialize)]
// pub struct AdAssetFeedAdditionalData {
//     pub automated_product_tags: Option<bool>,
//     pub brand_page_id: Option<String>,
//     pub is_click_to_message: Option<bool>,
//     pub multi_share_end_card: Option<bool>,
//     pub page_welcome_message: Option<String>,
//     pub partner_app_welcome_message_flow_id: Option<String>,
// }
// #[derive(Debug, Deserialize)]
// pub struct AdAssetFeedSpecAssetCustomizationRule {
//     pub body_label: Option<AdAssetFeedSpecAssetLabel>,
//     pub call_to_action_label: Option<AdAssetFeedSpecAssetLabel>,
//     pub call_to_action_type_label: Option<AdAssetFeedSpecAssetLabel>, // PLM CEA MAI PROASTA DOCUMENTATIE
//     pub caption_label: Option<AdAssetFeedSpecAssetLabel>,
//     pub carousel_label: Option<AdAssetFeedSpecAssetLabel>,
//     pub customization_spec: Option<AdAssetCustomizationRuleCustomizationSpec>,
//     pub description_label: Option<AdAssetFeedSpecAssetLabel>,
//     pub image_label: Option<AdAssetFeedSpecAssetLabel>,
//     pub is_default: Option<bool>,
//     pub link_url_label: Option<AdAssetFeedSpecAssetLabel>,
//     pub priority: Option<i32>,
//     pub title_label: Option<AdAssetFeedSpecAssetLabel>,
//     pub video_label: Option<AdAssetFeedSpecAssetLabel>,
// }

#[derive(Debug, Deserialize)]
pub struct AdgroupIssuesInfo {
    pub error_code: Option<i32>,
    pub error_message: Option<String>,
    pub error_summary: Option<String>,
    pub error_type: Option<String>, // HARD_ERROR / SOFT_ERROR
    pub level: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct ConversionActionQuery {
    pub action_type: Option<Vec<String>>,
    pub application: Option<Vec<String>>, // Poate conține fie liste de string-uri, fie ID-uri
    pub conversion_id: Option<Vec<String>>,
    pub creative: Option<Vec<String>>, // Poate conține fie liste de string-uri, fie ID-uri
    pub dataset: Option<Vec<String>>,
    pub event: Option<Vec<String>>,
    pub event_creator: Option<Vec<String>>,
    pub event_type: Option<Vec<String>>,
    pub fb_pixel: Option<Vec<String>>,
    pub fb_pixel_event: Option<Vec<String>>,
    pub leadgen: Option<Vec<String>>,
    pub object: Option<Vec<String>>,
    pub object_domain: Option<Vec<String>>,
    pub offer: Option<Vec<String>>,
    pub offer_creator: Option<Vec<String>>,
    pub offsite_pixel: Option<Vec<String>>,
    pub page: Option<Vec<String>>,
    pub page_parent: Option<Vec<String>>,
    pub post: Option<Vec<String>>,
    pub post_object: Option<Vec<String>>,
    pub post_object_wall: Option<Vec<String>>,
    pub post_wall: Option<Vec<String>>,
    pub question: Option<Vec<String>>,
    pub question_creator: Option<Vec<String>>,
    pub response: Option<Vec<String>>,
    pub subtype: Option<Vec<String>>,
}

impl CampaignsResponse {
    pub fn display(&self) {
        println!("Displaying Campaigns:");

        println!("Paging Information:");
        println!("Before Cursor: {}", self.paging.cursors.before);
        println!("After Cursor: {}", self.paging.cursors.after);
        println!("----------------------------------------");

        for campaign in &self.data {
            if let Some(id) = &campaign.id {
                println!("Campaign ID: {}", id);
            }

            if let Some(account_id) = &campaign.account_id {
                println!("Ad Account ID: {}", account_id);
            }

            if let Some(adlabels) = &campaign.adlabels {
                println!("Ad Labels:");
                for adlabel in adlabels {
                    println!("  ID: {}", adlabel.id);
                    println!("  Name: {}", adlabel.name);

                    if let Some(account) = &adlabel.account {
                        println!("  Account ID: {}", account.account_id);
                        println!("  Internal ID: {}", account.id);
                    }

                    if let Some(created_time) = &adlabel.created_time {
                        println!("  Created Time: {}", created_time);
                    }

                    if let Some(updated_time) = &adlabel.updated_time {
                        println!("  Updated Time: {}", updated_time);
                    }
                }
            }

            if let Some(brand_lift_studies) = &campaign.brand_lift_studies {
                println!("Brand Lift Studies:");
                for study in brand_lift_studies {
                    println!("  ID: {}", study.id);
                    if let Some(business) = &study.business {
                        println!("  Business: {}", business);
                    }
                    if let Some(canceled_time) = &study.canceled_time {
                        println!("  Canceled Time: {}", canceled_time);
                    }
                    if let Some(cooldown_start_time) = &study.cooldown_start_time {
                        println!("  Cooldown Start Time: {}", cooldown_start_time);
                    }
                    if let Some(created_by) = &study.created_by {
                        println!("  Created By: {}", created_by);
                    }
                    if let Some(created_time) = &study.created_time {
                        println!("  Created Time: {}", created_time);
                    }
                    if let Some(description) = &study.description {
                        println!("  Description: {}", description);
                    }
                    if let Some(end_time) = &study.end_time {
                        println!("  End Time: {}", end_time);
                    }
                    println!("  Name: {}", study.name);
                    if let Some(observation_end_time) = &study.observation_end_time {
                        println!("  Observation End Time: {}", observation_end_time);
                    }
                    if let Some(results_first_available_date) = &study.results_first_available_date
                    {
                        println!(
                            "  Results First Available Date: {}",
                            results_first_available_date
                        );
                    }
                    if let Some(start_time) = &study.start_time {
                        println!("  Start Time: {}", start_time);
                    }
                    if let Some(study_type) = &study.study_type {
                        println!("  Type: {}", study_type);
                    }
                    if let Some(updated_by) = &study.updated_by {
                        println!("  Updated By: {}", updated_by);
                    }
                    if let Some(updated_time) = &study.updated_time {
                        println!("  Updated Time: {}", updated_time);
                    }
                    println!("----------------------------------------");
                }
            }
            if let Some(bid_strategy) = &campaign.bid_strategy {
                println!("Bid Strategy: {}", bid_strategy);
            }

            if let Some(boosted_object_id) = &campaign.boosted_object_id {
                println!("Boosted Object ID: {}", boosted_object_id);
            }

            if let Some(budget_rebalance_flag) = &campaign.budget_rebalance_flag {
                println!("Budget Rebalance Flag: {}", budget_rebalance_flag);
            }

            if let Some(budget_remaining) = &campaign.budget_remaining {
                println!("Budget Remaining: {}", budget_remaining);
            }
            if let Some(buying_type) = &campaign.buying_type {
                println!("Buying Type: {}", buying_type);
            }

            if let Some(campaign_group_active_time) = &campaign.campaign_group_active_time {
                println!("Campaign Group Active Time: {}", campaign_group_active_time);
            }

            if let Some(can_create_brand_lift_study) = &campaign.can_create_brand_lift_study {
                println!(
                    "Can Create Brand Lift Study: {}",
                    can_create_brand_lift_study
                );
            }

            if let Some(can_use_spend_cap) = &campaign.can_use_spend_cap {
                println!("Can Use Spend Cap: {}", can_use_spend_cap);
            }

            if let Some(configured_status) = &campaign.configured_status {
                println!("Configured Status: {}", configured_status);
            }

            if let Some(created_time) = &campaign.created_time {
                println!("Created Time: {}", created_time);
            }

            if let Some(daily_budget) = &campaign.daily_budget {
                println!("Daily Budget: {}", daily_budget);
            }

            if let Some(effective_status) = &campaign.effective_status {
                println!("Effective Status: {}", effective_status);
            }

            if let Some(has_secondary_skadnetwork_reporting) =
                &campaign.has_secondary_skadnetwork_reporting
            {
                println!(
                    "Has Secondary SKAdNetwork Reporting: {}",
                    has_secondary_skadnetwork_reporting
                );
            }

            if let Some(is_budget_schedule_enabled) = &campaign.is_budget_schedule_enabled {
                println!("Is Budget Schedule Enabled: {}", is_budget_schedule_enabled);
            }

            if let Some(is_skadnetwork_attribution) = &campaign.is_skadnetwork_attribution {
                println!("Is SKAdNetwork Attribution: {}", is_skadnetwork_attribution);
            }

            if let Some(issues_info) = &campaign.issues_info {
                println!("Campaign Issues:");
                for issue in issues_info {
                    if let Some(error_code) = issue.error_code {
                        println!("  Error Code: {}", error_code);
                    }
                    if let Some(error_message) = &issue.error_message {
                        println!("  Error Message: {}", error_message);
                    }
                    if let Some(error_summary) = &issue.error_summary {
                        println!("  Error Summary: {}", error_summary);
                    }
                    if let Some(error_type) = &issue.error_type {
                        println!("  Error Type: {}", error_type);
                    }
                    if let Some(level) = &issue.level {
                        println!("  Level: {}", level);
                    }
                    println!("----------------------------------------");
                }
            }

            if let Some(last_budget_toggling_time) = &campaign.last_budget_toggling_time {
                println!("Last Budget Toggling Time: {}", last_budget_toggling_time);
            }

            if let Some(lifetime_budget) = &campaign.lifetime_budget {
                println!("Lifetime Budget: {}", lifetime_budget);
            }

            if let Some(name) = &campaign.name {
                println!("Campaign Name: {}", name);
            }

            if let Some(objective) = &campaign.objective {
                println!("Campaign Objective: {}", objective);
            }
            if let Some(pacing_type) = &campaign.pacing_type {
                println!("Pacing Type: {:?}", pacing_type);
            }

            if let Some(primary_attribution) = &campaign.primary_attribution {
                println!("Primary Attribution: {}", primary_attribution);
            }

            if let Some(promoted_object) = &campaign.promoted_object {
                println!("Promoted Object:");
                if let Some(application_id) = &promoted_object.application_id {
                    println!("  Application ID: {}", application_id);
                }
                if let Some(boosted_product_set_id) = &promoted_object.boosted_product_set_id {
                    println!("  Boosted Product Set ID: {}", boosted_product_set_id);
                }
                if let Some(conversion_goal_id) = &promoted_object.conversion_goal_id {
                    println!("  Conversion Goal ID: {}", conversion_goal_id);
                }
                if let Some(custom_conversion_id) = &promoted_object.custom_conversion_id {
                    println!("  Custom Conversion ID: {}", custom_conversion_id);
                }
                if let Some(custom_event_str) = &promoted_object.custom_event_str {
                    println!("  Custom Event String: {}", custom_event_str);
                }
                if let Some(custom_event_type) = &promoted_object.custom_event_type {
                    println!("  Custom Event Type: {}", custom_event_type);
                }
                if let Some(event_id) = &promoted_object.event_id {
                    println!("  Event ID: {}", event_id);
                }
                if let Some(lead_ads_custom_event_str) = &promoted_object.lead_ads_custom_event_str
                {
                    println!(
                        "  Lead Ads Custom Event String: {}",
                        lead_ads_custom_event_str
                    );
                }
                if let Some(lead_ads_custom_event_type) =
                    &promoted_object.lead_ads_custom_event_type
                {
                    println!(
                        "  Lead Ads Custom Event Type: {}",
                        lead_ads_custom_event_type
                    );
                }
                if let Some(lead_ads_form_event_source_type) =
                    &promoted_object.lead_ads_form_event_source_type
                {
                    println!(
                        "  Lead Ads Form Event Source Type: {}",
                        lead_ads_form_event_source_type
                    );
                }
                if let Some(mcme_conversion_id) = &promoted_object.mcme_conversion_id {
                    println!("  MCME Conversion ID: {}", mcme_conversion_id);
                }
                if let Some(object_store_url) = &promoted_object.object_store_url {
                    println!("  Object Store URL: {}", object_store_url);
                }
                if let Some(offer_id) = &promoted_object.offer_id {
                    println!("  Offer ID: {}", offer_id);
                }
                if let Some(offline_conversion_data_set_id) =
                    &promoted_object.offline_conversion_data_set_id
                {
                    println!(
                        "  Offline Conversion Data Set ID: {}",
                        offline_conversion_data_set_id
                    );
                }
                if let Some(offsite_conversion_event_id) =
                    &promoted_object.offsite_conversion_event_id
                {
                    println!(
                        "  Offsite Conversion Event ID: {}",
                        offsite_conversion_event_id
                    );
                }
                if let Some(page_id) = &promoted_object.page_id {
                    println!("  Page ID: {}", page_id);
                }
                if let Some(pixel_aggregation_rule) = &promoted_object.pixel_aggregation_rule {
                    println!("  Pixel Aggregation Rule: {}", pixel_aggregation_rule);
                }
                if let Some(pixel_id) = &promoted_object.pixel_id {
                    println!("  Pixel ID: {}", pixel_id);
                }
                if let Some(pixel_rule) = &promoted_object.pixel_rule {
                    println!("  Pixel Rule: {}", pixel_rule);
                }
                if let Some(place_page_set_id) = &promoted_object.place_page_set_id {
                    println!("  Place Page Set ID: {}", place_page_set_id);
                }
                if let Some(product_catalog_id) = &promoted_object.product_catalog_id {
                    println!("  Product Catalog ID: {}", product_catalog_id);
                }
                if let Some(product_set_id) = &promoted_object.product_set_id {
                    println!("  Product Set ID: {}", product_set_id);
                }
                if let Some(retention_days) = &promoted_object.retention_days {
                    println!("  Retention Days: {}", retention_days);
                }
                if let Some(value_semantic_type) = &promoted_object.value_semantic_type {
                    println!("  Value Semantic Type: {}", value_semantic_type);
                }
                if let Some(variation) = &promoted_object.variation {
                    println!("  Variation: {}", variation);
                }
                if let Some(whatsapp_phone_number) = &promoted_object.whatsapp_phone_number {
                    println!("  WhatsApp Phone Number: {}", whatsapp_phone_number);
                }
            }

            if let Some(smart_promotion_type) = &campaign.smart_promotion_type {
                println!("Smart Promotion Type: {}", smart_promotion_type);
            }

            if let Some(source_campaign) = &campaign.source_campaign {
                println!("Source Campaign: ", /*source_campaign*/);
            }

            if let Some(source_campaign_id) = &campaign.source_campaign_id {
                println!("Source Campaign ID: {}", source_campaign_id);
            }

            if let Some(special_ad_categories) = &campaign.special_ad_categories {
                println!("Special Ad Categories: {:?}", special_ad_categories);
            }

            if let Some(special_ad_category) = &campaign.special_ad_category {
                println!("Special Ad Category: {}", special_ad_category);
            }

            if let Some(special_ad_category_country) = &campaign.special_ad_category_country {
                println!(
                    "Special Ad Category Countries: {:?}",
                    special_ad_category_country
                );
            }

            if let Some(spend_cap) = &campaign.spend_cap {
                println!("Spend Cap: {}", spend_cap);
            }
            if let Some(start_time) = &campaign.start_time {
                println!("Start Time: {}", start_time);
            }
            if let Some(status) = &campaign.status {
                println!("Status: {}", status);
            }
            if let Some(stop_time) = &campaign.stop_time {
                println!("Stop Time: {}", stop_time);
            }

            if let Some(topline_id) = &campaign.topline_id {
                println!("Topline ID: {}", topline_id);
            }
            if let Some(updated_time) = &campaign.updated_time {
                println!("Updated Time: {}", updated_time);
            }

            println!("----------------------------------------");
        }
    }
}

impl Campaign {
    pub fn display(&self) {
        if let Some(id) = &self.id {
            println!("Campaign ID: {}", id);
        }

        if let Some(account_id) = &self.account_id {
            println!("Ad Account ID: {}", account_id);
        }

        if let Some(adlabels) = &self.adlabels {
            println!("Ad Labels:");
            for adlabel in adlabels {
                println!("  ID: {}", adlabel.id);
                println!("  Name: {}", adlabel.name);

                if let Some(account) = &adlabel.account {
                    println!("  Account ID: {}", account.account_id);
                    println!("  Internal ID: {}", account.id);
                }

                if let Some(created_time) = &adlabel.created_time {
                    println!("  Created Time: {}", created_time);
                }

                if let Some(updated_time) = &adlabel.updated_time {
                    println!("  Updated Time: {}", updated_time);
                }
            }
        }

        if let Some(brand_lift_studies) = &self.brand_lift_studies {
            println!("Brand Lift Studies:");
            for study in brand_lift_studies {
                println!("  ID: {}", study.id);
                if let Some(business) = &study.business {
                    println!("  Business: {}", business);
                }
                if let Some(canceled_time) = &study.canceled_time {
                    println!("  Canceled Time: {}", canceled_time);
                }
                if let Some(cooldown_start_time) = &study.cooldown_start_time {
                    println!("  Cooldown Start Time: {}", cooldown_start_time);
                }
                if let Some(created_by) = &study.created_by {
                    println!("  Created By: {}", created_by);
                }
                if let Some(created_time) = &study.created_time {
                    println!("  Created Time: {}", created_time);
                }
                if let Some(description) = &study.description {
                    println!("  Description: {}", description);
                }
                if let Some(end_time) = &study.end_time {
                    println!("  End Time: {}", end_time);
                }
                println!("  Name: {}", study.name);
                if let Some(observation_end_time) = &study.observation_end_time {
                    println!("  Observation End Time: {}", observation_end_time);
                }
                if let Some(results_first_available_date) = &study.results_first_available_date
                {
                    println!(
                        "  Results First Available Date: {}",
                        results_first_available_date
                    );
                }
                if let Some(start_time) = &study.start_time {
                    println!("  Start Time: {}", start_time);
                }
                if let Some(study_type) = &study.study_type {
                    println!("  Type: {}", study_type);
                }
                if let Some(updated_by) = &study.updated_by {
                    println!("  Updated By: {}", updated_by);
                }
                if let Some(updated_time) = &study.updated_time {
                    println!("  Updated Time: {}", updated_time);
                }
                println!("----------------------------------------");
            }
        }
        if let Some(bid_strategy) = &self.bid_strategy {
            println!("Bid Strategy: {}", bid_strategy);
        }

        if let Some(boosted_object_id) = &self.boosted_object_id {
            println!("Boosted Object ID: {}", boosted_object_id);
        }

        if let Some(budget_rebalance_flag) = &self.budget_rebalance_flag {
            println!("Budget Rebalance Flag: {}", budget_rebalance_flag);
        }

        if let Some(budget_remaining) = &self.budget_remaining {
            println!("Budget Remaining: {}", budget_remaining);
        }
        if let Some(buying_type) = &self.buying_type {
            println!("Buying Type: {}", buying_type);
        }

        if let Some(campaign_group_active_time) = &self.campaign_group_active_time {
            println!("Campaign Group Active Time: {}", campaign_group_active_time);
        }

        if let Some(can_create_brand_lift_study) = &self.can_create_brand_lift_study {
            println!(
                "Can Create Brand Lift Study: {}",
                can_create_brand_lift_study
            );
        }

        if let Some(can_use_spend_cap) = &self.can_use_spend_cap {
            println!("Can Use Spend Cap: {}", can_use_spend_cap);
        }

        if let Some(configured_status) = &self.configured_status {
            println!("Configured Status: {}", configured_status);
        }

        if let Some(created_time) = &self.created_time {
            println!("Created Time: {}", created_time);
        }

        if let Some(daily_budget) = &self.daily_budget {
            println!("Daily Budget: {}", daily_budget);
        }

        if let Some(effective_status) = &self.effective_status {
            println!("Effective Status: {}", effective_status);
        }

        if let Some(has_secondary_skadnetwork_reporting) =
            &self.has_secondary_skadnetwork_reporting
        {
            println!(
                "Has Secondary SKAdNetwork Reporting: {}",
                has_secondary_skadnetwork_reporting
            );
        }

        if let Some(is_budget_schedule_enabled) = &self.is_budget_schedule_enabled {
            println!("Is Budget Schedule Enabled: {}", is_budget_schedule_enabled);
        }

        if let Some(is_skadnetwork_attribution) = &self.is_skadnetwork_attribution {
            println!("Is SKAdNetwork Attribution: {}", is_skadnetwork_attribution);
        }

        if let Some(issues_info) = &self.issues_info {
            println!("Campaign Issues:");
            for issue in issues_info {
                if let Some(error_code) = issue.error_code {
                    println!("  Error Code: {}", error_code);
                }
                if let Some(error_message) = &issue.error_message {
                    println!("  Error Message: {}", error_message);
                }
                if let Some(error_summary) = &issue.error_summary {
                    println!("  Error Summary: {}", error_summary);
                }
                if let Some(error_type) = &issue.error_type {
                    println!("  Error Type: {}", error_type);
                }
                if let Some(level) = &issue.level {
                    println!("  Level: {}", level);
                }
                println!("----------------------------------------");
            }
        }

        if let Some(last_budget_toggling_time) = &self.last_budget_toggling_time {
            println!("Last Budget Toggling Time: {}", last_budget_toggling_time);
        }

        if let Some(lifetime_budget) = &self.lifetime_budget {
            println!("Lifetime Budget: {}", lifetime_budget);
        }

        if let Some(name) = &self.name {
            println!("Campaign Name: {}", name);
        }

        if let Some(objective) = &self.objective {
            println!("Campaign Objective: {}", objective);
        }
        if let Some(pacing_type) = &self.pacing_type {
            println!("Pacing Type: {:?}", pacing_type);
        }

        if let Some(primary_attribution) = &self.primary_attribution {
            println!("Primary Attribution: {}", primary_attribution);
        }

        if let Some(promoted_object) = &self.promoted_object {
            println!("Promoted Object:");
            if let Some(application_id) = &promoted_object.application_id {
                println!("  Application ID: {}", application_id);
            }
            if let Some(boosted_product_set_id) = &promoted_object.boosted_product_set_id {
                println!("  Boosted Product Set ID: {}", boosted_product_set_id);
            }
            if let Some(conversion_goal_id) = &promoted_object.conversion_goal_id {
                println!("  Conversion Goal ID: {}", conversion_goal_id);
            }
            if let Some(custom_conversion_id) = &promoted_object.custom_conversion_id {
                println!("  Custom Conversion ID: {}", custom_conversion_id);
            }
            if let Some(custom_event_str) = &promoted_object.custom_event_str {
                println!("  Custom Event String: {}", custom_event_str);
            }
            if let Some(custom_event_type) = &promoted_object.custom_event_type {
                println!("  Custom Event Type: {}", custom_event_type);
            }
            if let Some(event_id) = &promoted_object.event_id {
                println!("  Event ID: {}", event_id);
            }
            if let Some(lead_ads_custom_event_str) = &promoted_object.lead_ads_custom_event_str
            {
                println!(
                    "  Lead Ads Custom Event String: {}",
                    lead_ads_custom_event_str
                );
            }
            if let Some(lead_ads_custom_event_type) =
                &promoted_object.lead_ads_custom_event_type
            {
                println!(
                    "  Lead Ads Custom Event Type: {}",
                    lead_ads_custom_event_type
                );
            }
            if let Some(lead_ads_form_event_source_type) =
                &promoted_object.lead_ads_form_event_source_type
            {
                println!(
                    "  Lead Ads Form Event Source Type: {}",
                    lead_ads_form_event_source_type
                );
            }
            if let Some(mcme_conversion_id) = &promoted_object.mcme_conversion_id {
                println!("  MCME Conversion ID: {}", mcme_conversion_id);
            }
            if let Some(object_store_url) = &promoted_object.object_store_url {
                println!("  Object Store URL: {}", object_store_url);
            }
            if let Some(offer_id) = &promoted_object.offer_id {
                println!("  Offer ID: {}", offer_id);
            }
            if let Some(offline_conversion_data_set_id) =
                &promoted_object.offline_conversion_data_set_id
            {
                println!(
                    "  Offline Conversion Data Set ID: {}",
                    offline_conversion_data_set_id
                );
            }
            if let Some(offsite_conversion_event_id) =
                &promoted_object.offsite_conversion_event_id
            {
                println!(
                    "  Offsite Conversion Event ID: {}",
                    offsite_conversion_event_id
                );
            }
            if let Some(page_id) = &promoted_object.page_id {
                println!("  Page ID: {}", page_id);
            }
            if let Some(pixel_aggregation_rule) = &promoted_object.pixel_aggregation_rule {
                println!("  Pixel Aggregation Rule: {}", pixel_aggregation_rule);
            }
            if let Some(pixel_id) = &promoted_object.pixel_id {
                println!("  Pixel ID: {}", pixel_id);
            }
            if let Some(pixel_rule) = &promoted_object.pixel_rule {
                println!("  Pixel Rule: {}", pixel_rule);
            }
            if let Some(place_page_set_id) = &promoted_object.place_page_set_id {
                println!("  Place Page Set ID: {}", place_page_set_id);
            }
            if let Some(product_catalog_id) = &promoted_object.product_catalog_id {
                println!("  Product Catalog ID: {}", product_catalog_id);
            }
            if let Some(product_set_id) = &promoted_object.product_set_id {
                println!("  Product Set ID: {}", product_set_id);
            }
            if let Some(retention_days) = &promoted_object.retention_days {
                println!("  Retention Days: {}", retention_days);
            }
            if let Some(value_semantic_type) = &promoted_object.value_semantic_type {
                println!("  Value Semantic Type: {}", value_semantic_type);
            }
            if let Some(variation) = &promoted_object.variation {
                println!("  Variation: {}", variation);
            }
            if let Some(whatsapp_phone_number) = &promoted_object.whatsapp_phone_number {
                println!("  WhatsApp Phone Number: {}", whatsapp_phone_number);
            }
        }

        if let Some(smart_promotion_type) = &self.smart_promotion_type {
            println!("Smart Promotion Type: {}", smart_promotion_type);
        }

        if let Some(source_campaign) = &self.source_campaign {
            println!("Source Campaign: ", /*source_campaign*/);
        }

        if let Some(source_campaign_id) = &self.source_campaign_id {
            println!("Source Campaign ID: {}", source_campaign_id);
        }

        if let Some(special_ad_categories) = &self.special_ad_categories {
            println!("Special Ad Categories: {:?}", special_ad_categories);
        }

        if let Some(special_ad_category) = &self.special_ad_category {
            println!("Special Ad Category: {}", special_ad_category);
        }

        if let Some(special_ad_category_country) = &self.special_ad_category_country {
            println!(
                "Special Ad Category Countries: {:?}",
                special_ad_category_country
            );
        }

        if let Some(spend_cap) = &self.spend_cap {
            println!("Spend Cap: {}", spend_cap);
        }
        if let Some(start_time) = &self.start_time {
            println!("Start Time: {}", start_time);
        }
        if let Some(status) = &self.status {
            println!("Status: {}", status);
        }
        if let Some(stop_time) = &self.stop_time {
            println!("Stop Time: {}", stop_time);
        }

        if let Some(topline_id) = &self.topline_id {
            println!("Topline ID: {}", topline_id);
        }
        if let Some(updated_time) = &self.updated_time {
            println!("Updated Time: {}", updated_time);
        }

    }
}

impl AdSetResponse {
    pub fn display(&self) {}
}
