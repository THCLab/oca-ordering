use std::collections::BTreeMap;

use isolang::Language;
use said::sad::{SerializationFormats, SAD};
use serde::{Deserialize, Serialize};

#[derive(Debug, SAD, Serialize, Deserialize)]
pub struct Presentation {
    #[serde(rename = "bd")]
    pub bundle_digest: said::SelfAddressingIdentifier,
    #[said]
    #[serde(rename = "d")]
    pub said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "p")]
    pub pages: BTreeMap<String, Vec<String>>,
    #[serde(rename = "po")]
    pub pages_order: Vec<String>,
    #[serde(rename = "pl")]
    pub pages_label: BTreeMap<Language, BTreeMap<String, String>>,
    #[serde(rename = "i")]
    pub interfaces: Vec<Interface>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interface {
    #[serde(rename = "m")]
    pub mode: ModeType,
    #[serde(rename = "c")]
    purpose: Purpose,
    #[serde(rename = "a")]
    attr_properties: BTreeMap<String, Properties>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Properties {
    #[serde(rename = "t")]
    pub type_: AttrType,
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AttrType {
    TextArea
}

#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone)]
enum Purpose {
    Capture,
}
#[serde(rename_all = "lowercase")]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ModeType {
    Web,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_base() {
        let mut pages = BTreeMap::new();
        pages.insert("pageY".to_string(), vec!["attr_1".to_string()]);
        pages.insert(
            "pageZ".to_string(),
            vec!["attr_3".to_string(), "attr_2".to_string()],
        );

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
            interfaces: vec![Interface {
                mode: ModeType::Web,
                purpose: Purpose::Capture,
                attr_properties: vec![(
                    "attr_1".to_string(),
                    Properties {
                        type_: AttrType::TextArea,
                    },
                )]
                .into_iter()
                .collect(),
            }]
        };

        presentation_base.compute_digest();

        println!(
            "{}",
            serde_json::to_string_pretty(&presentation_base).unwrap()
        );
        assert_eq!(
            presentation_base.said.unwrap().to_string(),
            "EBDKqlEFqzmW00Q6PzsqXb0rPTXeCqYcHS1kwhg76LiF".to_string()
        );
    }
}
