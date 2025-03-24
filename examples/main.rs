use std::io::Write;
use turnkey_api::{Client, Error::UnexpectedResponse, types::*};

#[tokio::main]
async fn main() {
    print!("Enter your organization ID: ");
    std::io::stdout().flush().unwrap();
    let mut organization_id = String::new();
    std::io::stdin().read_line(&mut organization_id).unwrap();

    let client = Client::new("https://api.turnkey.com");
    let resp = client
        .get_sub_org_ids(&GetSubOrgIdsRequest {
            organization_id: organization_id.trim().to_string(),
            filter_type: None,
            filter_value: None,
            pagination_options: None,
        })
        .await;

    match resp {
        Ok(resp) => {
            println!(
                "Sub organization IDs: {}, ",
                resp.organization_ids.join(", ")
            );
        }
        Err(UnexpectedResponse(e)) => {
            println!(
                "Received error response with status {:?}: {:?}",
                e.status(),
                e.text().await.unwrap()
            );
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
