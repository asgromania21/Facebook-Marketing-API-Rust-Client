# Facebook API Rust Client

## Description

This project is a Rust client for interacting with the Facebook API. It allows users to authenticate and make API requests to retrieve information such as ad accounts.

## Installation

To use this project, ensure you have Rust installed. Then, clone this repository and build the project:

```sh
git clone <repository-url>
cd <project-directory>
cargo build --release
```

## Usage

To create a new Facebook client and fetch the initial App Access Token:

```rust
mod client;
mod errors;
mod types;

use crate::client::FacebookClient;
use crate::errors::*;

fn main() {
    let app_id = "your_app_id";
    let app_secret = "your_app_secret";
    let user_token = "your_user_token";
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

    let campaign_id = "your_campaign_id";

    match client.activate_campaign(campaign_id) {
        Ok(()) => println!("Campaign with ID {} has been successfully activated!", campaign_id),
        Err(e) => match e {
            FacebookError::HttpError(err_msg) => println!("HTTP Error: {}", err_msg),
            FacebookError::Unauthorized => println!("Error: Insufficient permissions to activate the campaign."),
            FacebookError::Unexpected(msg) => println!("Unexpected error: {}", msg),
        },
    }

    let campaign_id = "your_campaign_id";
    match client.delete_campaign(campaign_id) {
        Ok(()) => println!("Campaign with ID {} has been successfully deleted!", campaign_id),
        Err(e) => match e {
            FacebookError::HttpError(err_msg) => println!("HTTP Error: {}", err_msg),
            FacebookError::Unauthorized => println!("Error: Insufficient permissions to delete the campaign."),
            FacebookError::Unexpected(msg) => println!("Unexpected error: {}", msg),
        },
    }
    let ad_account_id = "your_ad_account_id";

    match client.get_campaigns(ad_account_id) {
        Ok(campaigns_response) => {
            campaigns_response.display();
        }
        Err(e) => {
            eprintln!("Error fetching campaigns: {}", e);
        }
    }
    let campaign_id: &str = "your_campaign_id";

    match client.get_ad_sets(campaign_id) {
        Ok(ad_sets_response) => {
            ad_sets_response.display();
        }
        Err(e) => {
            eprintln!("Error fetching ad sets: {}", e);
        }
    }
} 
```

## Known Issues

- **Display Function Issue**: The `display` function references a `source_campaign` structure, which is a copied version of the current campaign. However, it does not function correctly in the display process and is currently commented out in the code.
- **AdCreative Structure**: The `AdCreative` structure was not implemented due to poor Facebook documentation. The structure of objects is commented out in the code for future reference.
- **Ad Set Issue**: The field `contextual_bundling_spec` is commented out because it requires additional permissions beyond the standard ones and causes an error in the HTTP request.

## Future Updates

- Fix the issue with `source_campaign` in the `display` function to ensure correct visualization of copied campaigns.
- Improve error handling for API request failures.
- Optimize API calls to reduce response time.
- Implement `AdCreative` once better documentation or alternative solutions are available.
- Investigate and handle permission issues related to `contextual_bundling_spec` in Ad Sets.

## Error Handling

The project defines several possible errors:

- `HttpError(String)`: Occurs when an HTTP request fails.
- `Unauthorized`: Triggered when access is denied.
- `Unexpected(String)`: Covers any other unexpected errors.

Handle errors using Rust's `Result` type:

```rust
match some_function() {
    Ok(result) => println!("Success: {:?}", result),
    Err(FacebookError::HttpError(e)) => println!("HTTP Error: {}", e),
    Err(FacebookError::Unauthorized) => println!("Unauthorized access"),
    Err(FacebookError::Unexpected(e)) => println!("Unexpected error: {}", e),
}
```

## Contributing

If you'd like to contribute, feel free to submit a pull request!

## License

This project is licensed under the MIT License.

