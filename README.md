# Turnkey API Rust Client

A rust client for the [Turnkey](https://www.turnkey.com/) API, generated from the OpenAPI spec using [Progenitor](https://github.com/oxidecomputer/progenitor).

## Installation

Add the following dependency to your `Cargo.toml`:

```toml
turnkey_api = { git = "https://github.com/serdnad/turnkey-api-rs.git" }
```

## Usage

First, [create an API keypair](https://docs.turnkey.com/getting-started/quickstart) from Turnkey, and export the **private** key as an environment variable:

```sh
export TURNKEY_API_PRIVATE_KEY="<PRIVATE_KEY>"
```

Then, create a new client and use as follows:

```rust
use turnkey_api::{Client, types::*};

#[tokio::main]
async fn main() {
    let client = Client::new("https://api.turnkey.com");
    let resp = client
        .get_sub_org_ids(&GetSubOrgIdsRequest {
            organization_id: "<MY_ORGANIZATION_ID>".to_string(),
            filter_type: None,
            filter_value: None,
            pagination_options: None,
        })
        .await
        .unwrap();

    println!("{}, ", resp.organization_ids.join(", "));
}

```

All routes, requests, and responses are typed and documented courtesy of the OpenAPI spec.

## Development

The OpenAPI spec used to generate this client can be found [here](https://github.com/tkhq/docs/blob/main-legacy/api/public_api.swagger.json).

Because Progenitor requires OpenAPI 3.0 specs, the spec was first converted from 2.0 to 3.0 using [swagger2openapi](https://github.com/Mermade/oas-kit/blob/main/packages/swagger2openapi/README.md).
