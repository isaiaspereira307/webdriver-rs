
use fantoccini::{ClientBuilder, Locator, Client};
use tokio;
use serde_json::json;
use serde_json::Value;
use serde_json::Map;


fn create_capabilities(download_dir: &str) -> Map<String, Value> {
    let capabilities = json!({
        "moz:firefoxOptions": {
            // "args": ["-headless"] // if you want a headless browser
        },
        "timeouts": {
            "pageLoad": 1000*60*5,
            "implicit": 1000*60*5,
            "script": 1000*60*5,
        }
    });

    let map= capabilities.as_object().unwrap().to_owned();
    map
}

async fn go_to_site(site: String) -> Client {
    let client = ClientBuilder::native().connect("http://firefox:4444").await.expect("failed to connect to WebDriver");

    client.goto(&site).await.expect("failed to go to the site");
    let url = client.current_url().await.expect("failed to know currente url");
    assert_eq!(url.as_ref(), "https://cadprev.previdencia.gov.br/Cadprev/pages/index.xhtml");
    client
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let client = ClientBuilder::native().connect("http://firefox:4444").await.expect("failed to connect to WebDriver");

    client.goto("https://google.com").await.expect("failed to go to the site");
    let url = client.current_url().await.expect("failed to know currente url");
    assert_eq!(url.as_ref(), "https://cadprev.previdencia.gov.br/Cadprev/pages/index.xhtml");

    client.find(Locator::Id("mw-disambig")).await?.click().await?;


    client.close().await
}
