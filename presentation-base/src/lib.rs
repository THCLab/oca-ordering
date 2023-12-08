use std::collections::BTreeMap;

use isolang::Language;
use said::sad::{SerializationFormats, SAD};
use serde::{Deserialize, Serialize};

#[derive(Debug, SAD, Serialize, Deserialize)]
pub struct PresentationBase {
    #[serde(rename = "bd")]
    pub bundle_digest: said::SelfAddressingIdentifier,
    #[said]
    #[serde(rename = "d")]
    pub said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "o")]
    pub pages: BTreeMap<String, Vec<String>>,
    #[serde(rename = "po")]
    pub pages_order: Vec<String>,
    #[serde(rename = "pl")]
    pub pages_label: BTreeMap<Language, BTreeMap<String, String>>,
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

        let mut presentation_base = PresentationBase {
            bundle_digest: "EHp19U2U1sdOBmPzMmILM3DUI0PQph9tdN3KtmBrvNV7"
                .parse()
                .unwrap(),
            said: None,
            pages,
            pages_order: vec!["pageY".to_string(), "pageZ".to_string()],
            pages_label,
        };

        presentation_base.compute_digest();

        println!(
            "{}",
            serde_json::to_string_pretty(&presentation_base).unwrap()
        );
        assert_eq!(
            presentation_base.said.unwrap().to_string(),
            "EB17nP-YYp1eav0KHu6K90iUVn47sR1_z3_dKtoEBL8Z".to_string()
        );
    }
}
