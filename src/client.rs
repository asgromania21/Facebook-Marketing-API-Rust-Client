use crate::errors::FacebookError;
use crate::types::*;
use reqwest::blocking::*; 

#[derive(Debug)]
pub struct FacebookClient {
    pub app_id: String,
    pub app_secret: String,
    pub user_token: String,
    pub app_access_token: Option<String>,
    pub http_client: Client,
    pub version: String,  
}

impl FacebookClient {
    /// Creates a new Facebook client and fetches the initial App Access Token.
    ///
    /// # Arguments
    ///
    /// * `app_id` - The Facebook App ID.
    /// * `app_secret` - The Facebook App Secret.
    /// * `user_token` - The user's access token.
    /// * `version` - The version of the Facebook Graph API.
    ///
    /// # Returns
    ///
    /// A `Result` containing a new instance of `FacebookClient` or a `FacebookError`.
    pub fn new(app_id: &str, app_secret: &str, user_token: &str, version: &str) -> Result<Self, FacebookError> {
        let mut client = FacebookClient {
            app_id: app_id.to_string(),
            app_secret: app_secret.to_string(),
            user_token: user_token.to_string(),
            app_access_token: None,
            http_client: Client::new(),
            version: version.to_string(), 
        };
        client.refresh_app_access_token()?;
        Ok(client)
    }

    /// Fetches or refreshes the App Access Token for the Facebook application.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or `FacebookError` if there's an issue.
    pub fn refresh_app_access_token(&mut self) -> Result<(), FacebookError> {
        let url = format!(
            "https://graph.facebook.com/{}/oauth/access_token?client_id={}&client_secret={}&grant_type=client_credentials",
            self.version, self.app_id, self.app_secret
        );

        let response = self
            .http_client
            .get(&url)
            .send() 
            .map_err(|e| FacebookError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response
            .text()
            .map_err(|e| FacebookError::Unexpected(format!("Failed to read response text: {}", e)))?;

        if status.is_success() {
            let token_response: AppAccessResponse = serde_json::from_str(&response_text)
                .map_err(|e| FacebookError::Unexpected(format!("Failed to parse JSON: {}", e)))?;
            self.app_access_token = Some(token_response.access_token);
            Ok(())
        } else if status == reqwest::StatusCode::UNAUTHORIZED {
            Err(FacebookError::Unauthorized)
        } else {
            Err(FacebookError::HttpError(format!(
                "Unexpected response status: {}. Response: {}",
                status, response_text
            )))
        }
    }

    /// Fetches ad accounts associated with the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing a list of ad accounts or a `FacebookError`.
    pub fn get_ad_accounts(&self) -> Result<AdsAccountsResponse, FacebookError> {
        let url = format!(
            "https://graph.facebook.com/{}/me/adaccounts?access_token={}",
            self.version, self.user_token
        );

        let response = self
            .http_client
            .get(&url)
            .send() 
            .map_err(|e| FacebookError::HttpError(e.to_string()))?; 

        let status = response.status();
        let response_text = response
            .text()
            .map_err(|e| FacebookError::Unexpected(format!("Failed to read response text: {}", e)))?;

        if status.is_client_error() || status.is_server_error() {
            return Err(FacebookError::Unexpected(response_text));
        }

        let ad_accounts_response: AdsAccountsResponse = serde_json::from_str(&response_text)
            .map_err(|e| FacebookError::Unexpected(format!("Failed to parse JSON: {}", e)))?;

        Ok(ad_accounts_response)
    }

    pub fn get_campaigns(&self, ad_account_id: &str) -> Result<CampaignsResponse, FacebookError> {

        let fields = vec![
            "id", "account_id", "adlabels", "bid_strategy", "boosted_object_id", "brand_lift_studies", 
            "budget_rebalance_flag", "budget_remaining", "buying_type", "campaign_group_active_time", 
            "can_create_brand_lift_study", "can_use_spend_cap", "configured_status", "created_time", 
            "daily_budget", "effective_status", "has_secondary_skadnetwork_reporting", 
            "is_budget_schedule_enabled", "is_skadnetwork_attribution", "issues_info", 
            "last_budget_toggling_time", "lifetime_budget", "name", "objective", "pacing_type", 
            "primary_attribution", "promoted_object", "smart_promotion_type", "source_campaign", 
            "source_campaign_id", "special_ad_categories", "special_ad_category", 
            "special_ad_category_country", "spend_cap", "start_time", "status", "stop_time", 
            "topline_id", "updated_time"// edges : "ad_studies", "adrules_governed", "ads", "adsets", 
            //"budget_schedules", "copies", "insights"
        ];

        let fields_param = fields.join(",");

        let url = format!(
            "https://graph.facebook.com/{}/act_{}/campaigns?access_token={}&fields={}",
            self.version, ad_account_id, self.user_token, fields_param
        );

        let response = self
            .http_client
            .get(&url)
            .send()
            .map_err(|e| FacebookError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response
            .text()
            .map_err(|e| FacebookError::Unexpected(format!("Failed to read response text: {}", e)))?;

        if status.is_success() {
            let campaigns_response: CampaignsResponse = serde_json::from_str(&response_text)
                .map_err(|e| FacebookError::Unexpected(format!("Failed to parse JSON: {}", e)))?;
            Ok(campaigns_response)
        } else {
            Err(FacebookError::HttpError(format!(
                "Error fetching campaigns: {}. Response: {}",
                status, response_text
            )))
        }
    }
    
    pub fn activate_campaign(&self, campaign_id: &str) -> Result<(), FacebookError> {
        let url = format!(
            "https://graph.facebook.com/{}/{}?access_token={}",
            self.version, campaign_id, self.user_token
        );

        let params = [("status", "ACTIVE")];

        let response = self
            .http_client
            .post(&url)
            .form(&params)
            .send()
            .map_err(|e| FacebookError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response
            .text()
            .map_err(|e| FacebookError::Unexpected(format!("Failed to read response text: {}", e)))?;

        if status.is_success() {
            Ok(())
        } else {
            Err(FacebookError::HttpError(format!(
                "Error activating campaign: {}. Response: {}",
                status, response_text
            )))
        }
    }

    pub fn delete_campaign(&self, campaign_id: &str) -> Result<(), FacebookError> {
        let url = format!(
            "https://graph.facebook.com/{}/{}?access_token={}",
            self.version, campaign_id, self.user_token
        );

        let response = self
            .http_client
            .delete(&url)
            .send()
            .map_err(|e| FacebookError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response
            .text()
            .map_err(|e| FacebookError::Unexpected(format!("Failed to read response text: {}", e)))?;

        if status.is_success() {
            Ok(())
        } else {
            Err(FacebookError::HttpError(format!(
                "Error deleting campaign: {}. Response: {}",
                status, response_text
            )))
        }
    }

    pub fn get_ad_sets(&self, campaign_id: &str) -> Result<AdSetResponse, FacebookError> {

        let fields = vec![
            "id", "account_id", "adlabels", "adset_schedule", "asset_feed_id", "attribution_spec", 
            "bid_adjustments", "bid_amount", "bid_constraints", "bid_info", "bid_strategy", "billing_event", 
            "brand_safety_config", "budget_remaining", "campaign", "campaign_active_time", "campaign_attribution", 
            "campaign_id", "configured_status",/* "contextual_bundling_spec"*/"created_time", "creative_sequence", 
            "daily_budget", "daily_min_spend_target", "daily_spend_cap", "destination_type", "dsa_beneficiary", 
            "dsa_payor", "effective_status", "end_time", "frequency_control_specs", "instagram_user_id", 
            "is_dynamic_creative", "issues_info", "learning_stage_info", "lifetime_budget", "lifetime_imps", 
            "lifetime_min_spend_target", "lifetime_spend_cap", "min_budget_spend_percentage", "multi_optimization_goal_weight", 
            "name", "optimization_goal", "optimization_sub_event", "pacing_type", "promoted_object", "recommendations", 
            "recurring_budget_semantics", "regional_regulated_categories", "regional_regulation_identities", "review_feedback", 
            "rf_prediction_id", "source_adset", "source_adset_id", "start_time", "status", "targeting", 
            "targeting_optimization_types", "time_based_ad_rotation_id_blocks", "time_based_ad_rotation_intervals", 
            "updated_time", "use_new_app_click"
        ];
        
    
        let fields_param = fields.join(",");
    
        let url = format!(
            "https://graph.facebook.com/{}/{}/adsets?fields={}&access_token={}",
            self.version, campaign_id, fields_param, self.user_token
        );
    
        let response = self
            .http_client
            .get(&url)
            .send()
            .map_err(|e| FacebookError::HttpError(e.to_string()))?;
    
        let status = response.status();
        let response_text = response
            .text()
            .map_err(|e| FacebookError::Unexpected(format!("Failed to read response text: {}", e)))?;
    
        if status.is_success() {
            let ad_set_response: AdSetResponse = serde_json::from_str(&response_text)
                .map_err(|e| FacebookError::Unexpected(format!("Failed to parse JSON: {}", e)))?;
            Ok(ad_set_response)
        } else {
            Err(FacebookError::HttpError(format!(
                "Error fetching ad sets: {}. Response: {}",
                status, response_text
            )))
        }
    }
    
    pub fn get_ads(&self, ad_set_id: &str) -> Result<AdResponse, FacebookError> {
        let fields = vec![
            "id", "account_id", "ad_active_time", "ad_review_feedback", "ad_schedule_end_time",
            "ad_schedule_start_time", "adlabels", "adset", "adset_id", "bid_amount", "campaign",
            "campaign_id", "configured_status", "conversion_domain", "created_time", 
            //"creative", "creative_asset_groups_spec", 
            "effective_status", "issues_info", "last_updated_by_app_id",
            "name", "preview_shareable_link", "recommendations", "source_ad", "source_ad_id",
            "status", "tracking_specs", "updated_time"
        ];
    
        let fields_param = fields.join(",");
    
        let url = format!(
            "https://graph.facebook.com/{}/{}/ads?fields={}&access_token={}",
            self.version, ad_set_id, fields_param, self.user_token
        );
    
        let response = self
            .http_client
            .get(&url)
            .send()
            .map_err(|e| FacebookError::HttpError(e.to_string()))?;
    
        let status = response.status();
        let response_text = response
            .text()
            .map_err(|e| FacebookError::Unexpected(format!("Failed to read response text: {}", e)))?;
    
        if status.is_success() {
            let ad_response: AdResponse = serde_json::from_str(&response_text)
                .map_err(|e| FacebookError::Unexpected(format!("Failed to parse JSON: {}", e)))?;
            Ok(ad_response)
        } else {
            Err(FacebookError::HttpError(format!(
                "Error fetching ads: {}. Response: {}",
                status, response_text
            )))
        }
    }
    
}
