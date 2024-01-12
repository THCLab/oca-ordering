use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    #[serde(rename = "n")]
    pub name: String,
    pub ao: Vec<PageElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum PageElement {
    Value(String),
    Page(Page),
}

#[test]
fn page_example() {
    let pahe = r#"{
              "n": "page1",
              "ao": [
                "passed",
                "d",
                "sign",
                {
                  "n": "customer",
                  "ao": [
                    "name",
                    "surname",
                    {
                      "n": "building",
                      "ao": ["floors", "area", { "n": "address", "ao": ["city", "zip", "street"] }]
                    }
                  ]
                }
              ]
            }"#;

    let page_deser: Page = serde_json::from_str(pahe).unwrap();
    let mut page_no_whitespace = pahe.to_string();
    page_no_whitespace.retain(|c| !c.is_whitespace());
    assert_eq!(
        page_no_whitespace,
        serde_json::to_string(&page_deser).unwrap()
    );
}
