mod client;
mod errors;
mod types;

use crate::client::FacebookClient;
use crate::errors::*;

fn main() {
    let app_id = "your_app_id_here";  // Replace with your actual app ID
    let app_secret = "your_app_secret_here";  // Replace with your actual app secret
    let user_token = "your_user_token_here";  // Replace with your actual user token
    let version = "v22.0"; 

    let client = match FacebookClient::new(app_id, app_secret, user_token, version) {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Error creating Facebook client: {}", e);
            return;
        }
    };

    match client.get_ad_accounts() {
        Ok(ad_accounts_response) => {
            ad_accounts_response.display();
        }
        Err(e) => {
            eprintln!("Error fetching ad accounts: {}", e);
        }
    }

    let campaign_id = "your_campaign_id_here";  // Replace with your actual campaign ID

    match client.activate_campaign(campaign_id) {
        Ok(()) => println!("The campaign with ID {} has been successfully activated!", campaign_id),
        Err(e) => match e {
            FacebookError::HttpError(err_msg) => println!("HTTP Error: {}", err_msg),
            FacebookError::Unauthorized => println!("Error: You don't have enough permissions to activate the campaign."),
            FacebookError::Unexpected(msg) => println!("Unexpected error: {}", msg),
        },
    }

    let campaign_id = "your_campaign_id_to_delete_here";  // Replace with the campaign ID you want to delete
    match client.delete_campaign(campaign_id) {
        Ok(()) => println!("The campaign with ID {} has been successfully deleted!", campaign_id),
        Err(e) => match e {
            FacebookError::HttpError(err_msg) => println!("HTTP Error: {}", err_msg),
            FacebookError::Unauthorized => println!("Error: You don't have enough permissions to delete the campaign."),
            FacebookError::Unexpected(msg) => println!("Unexpected error: {}", msg),
        },
    }
    
    let ad_account_id = "your_ad_account_id_here";  // Replace with your actual ad account ID

    match client.get_campaigns(ad_account_id) {
        Ok(campaigns_response) => {
            campaigns_response.display();
        }
        Err(e) => {
            eprintln!("Error fetching campaigns: {}", e);
        }
    }

    let campaign_id: &str = "your_campaign_id_here";  // Replace with the specific campaign ID you want to use

    match client.get_ad_sets(campaign_id) {
        Ok(ad_sets_response) => {
            ad_sets_response.display();
        }
        Err(e) => {
            eprintln!("Error fetching ad sets: {}", e);
        }
    }
}
