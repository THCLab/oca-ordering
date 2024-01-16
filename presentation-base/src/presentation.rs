use std::collections::BTreeMap;

use isolang::Language;
use said::{
    sad::{SerializationFormats, SAD},
    SelfAddressingIdentifier,
};
use serde::{Deserialize, Deserializer, Serialize};

use crate::page::Page;

#[derive(Debug, thiserror::Error)]
pub enum PresentationError {
    #[error("Said doesn't match presentation")]
    SaidDoesNotMatch,
    #[error("`d` field is empty")]
    MissingSaid,
}

#[derive(Debug, SAD, Serialize, Deserialize)]
pub struct Presentation {
    #[serde(rename = "v")]
    pub version: String,
    #[serde(rename = "bd")]
    pub bundle_digest: said::SelfAddressingIdentifier,
    #[said]
    #[serde(rename = "d")]
    #[serde(deserialize_with = "empty_string_is_none")]
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

impl Presentation {
    pub fn validate_digest(&self) -> Result<(), PresentationError> {
        let der_data = self.derivation_data();
        if self
            .said
            .as_ref()
            .ok_or(PresentationError::MissingSaid)?
            .verify_binding(&der_data)
        {
            Ok(())
        } else {
            Err(PresentationError::SaidDoesNotMatch)
        }
    }
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AttrType {
    TextArea,
    Signature,
    File,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Context {
    Capture,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InteractionMethod {
    Web,
    Ai,
}

fn empty_string_is_none<'de, D>(
    deserializer: D,
) -> Result<Option<SelfAddressingIdentifier>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.parse().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::page::PageElement;

    use super::*;

    #[test]
    fn test_presentation_base() {
        let page_y = Page {
            name: "pageY".to_string(),
            attribute_order: vec![PageElement::Value("attr_1".to_string())],
        };
        let page_z = Page {
            name: "pageZ".to_string(),
            attribute_order: vec![
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
            version: "1.0.0".to_string(),
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
        let der_data = presentation_base.derivation_data();
        let sai = presentation_base.said.unwrap();
        assert!(sai.verify_binding(&der_data));
        assert_eq!(
            sai.to_string(),
            "EEH1HiTzmFoX58oNnJpKSdKrQ4LQ6WsTzdV_eUz-tF1H".to_string()
        );
    }

    #[test]
    fn test_deserialize() {
        let input = r#"{
  "v":"1.0.0",
  "bd": "EHp19U2U1sdOBmPzMmILM3DUI0PQph9tdN3KtmBrvNV7",
  "d": "",
  "p": [
    {
      "n": "pageY",
      "ao": [
        "attr_1"
      ]
    },
    {
      "n": "pageZ",
      "ao": [
        "attr_3",
        "attr_2"
      ]
    }
  ],
  "po": [
    "pageY",
    "pageZ"
  ],
  "pl": {
    "eng": {
      "pageY": "Page Y",
      "pageZ": "Page Z"
    }
  },
  "i": [
    {
      "m": "web",
      "c": "capture",
      "a": {
        "attr_1": {
          "t": "textarea"
        }
      }
    }
  ]
}"#;

        let pres: Presentation = serde_json::from_str(input).unwrap();
        assert!(pres.said.is_none());
    }
}
