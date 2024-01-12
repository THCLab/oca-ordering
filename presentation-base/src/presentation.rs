use std::collections::BTreeMap;

use isolang::Language;
use said::sad::{SerializationFormats, SAD};
use serde::{Deserialize, Serialize};

use crate::page::Page;

#[derive(Debug, SAD, Serialize, Deserialize)]
pub struct Presentation {
    #[serde(rename = "bd")]
    pub bundle_digest: said::SelfAddressingIdentifier,
    #[said]
    #[serde(rename = "d")]
    pub said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "p")]
    pub pages: Vec<Page>,
    #[serde(rename = "po")]
    pub pages_order: Vec<String>,
    #[serde(rename = "pl")]
    pub pages_label: BTreeMap<Language, BTreeMap<String, String>>,
    #[serde(rename = "i")]
    pub interaction: Vec<Interaction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interaction {
    #[serde(rename = "m")]
    pub interaction_method: InteractionMethod,
    #[serde(rename = "c")]
    pub context: Context,
    #[serde(rename = "a")]
    pub attr_properties: BTreeMap<String, Properties>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    #[serde(rename = "t")]
    pub type_: AttrType,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AttrType {
    TextArea,
    Signature,
    File,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Context {
    Capture,
}
#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InteractionMethod {
    Web,
    Ai,
}

#[cfg(test)]
mod tests {
    use crate::page::PageElement;

    use super::*;

    #[test]
    fn test_presentation_base() {
        let page_y = Page {
            name: "pageY".to_string(),
            ao: vec![PageElement::Value("attr_1".to_string())],
        };
        let page_z = Page {
            name: "pageZ".to_string(),
            ao: vec![
                PageElement::Value("attr_3".to_string()),
                PageElement::Value("attr_2".to_string()),
            ],
        };
        let pages = vec![page_y, page_z];

        let mut pages_label = BTreeMap::new();
        let mut pages_label_en = BTreeMap::new();
        pages_label_en.insert("pageY".to_string(), "Page Y".to_string());
        pages_label_en.insert("pageZ".to_string(), "Page Z".to_string());
        pages_label.insert(Language::Eng, pages_label_en);

        let mut presentation_base = Presentation {
            bundle_digest: "EHp19U2U1sdOBmPzMmILM3DUI0PQph9tdN3KtmBrvNV7"
                .parse()
                .unwrap(),
            said: None,
            pages,
            pages_order: vec!["pageY".to_string(), "pageZ".to_string()],
            pages_label,
            interaction: vec![Interaction {
                interaction_method: InteractionMethod::Web,
                context: Context::Capture,
                attr_properties: vec![(
                    "attr_1".to_string(),
                    Properties {
                        type_: AttrType::TextArea,
                    },
                )]
                .into_iter()
                .collect(),
            }],
        };

        presentation_base.compute_digest();

        println!(
            "{}",
            serde_json::to_string_pretty(&presentation_base).unwrap()
        );
        assert_eq!(
            presentation_base.said.unwrap().to_string(),
            "EKqOPrmvf4GEp8ec9KKxig0TdFTGqNo0zAlFIjut6z7Z".to_string()
        );
    }
}
