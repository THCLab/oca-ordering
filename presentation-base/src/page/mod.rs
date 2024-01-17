use serde::{Deserialize, Serialize};
pub mod recursion_setup;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "ao")]
    pub attribute_order: Vec<PageElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PageElement {
    Value(String),
    Page {
        #[serde(rename = "n")]
        name: String,
        #[serde(rename = "ao")]
        attribute_order: Vec<PageElement>,
    },
}

#[test]
fn page_example() {
    let page = r#"{
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

    let page_deser: Page = serde_json::from_str(page).unwrap();
    let mut page_no_whitespace = page.to_string();
    page_no_whitespace.retain(|c| !c.is_whitespace());
    assert_eq!(
        page_no_whitespace,
        serde_json::to_string(&page_deser).unwrap()
    );
}
