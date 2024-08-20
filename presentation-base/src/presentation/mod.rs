use std::collections::BTreeMap;

use isolang::Language;
use said::sad::{SerializationFormats, SAD};
use said::derivation::HashFunctionCode;
use serde::{Deserialize, Serialize, Serializer};
use serialization::opt_serialization;

use crate::page::Page;
use indexmap::IndexMap;
mod serialization;

#[derive(Debug, thiserror::Error)]
pub enum PresentationError {
    #[error("Said doesn't match presentation")]
    SaidDoesNotMatch,
    #[error("`d` field is empty")]
    MissingSaid,
}

#[derive(Debug, SAD, Deserialize)]
pub struct Presentation {
    #[serde(rename = "v")]
    pub version: String,
    #[serde(rename = "bd")]
    pub bundle_digest: said::SelfAddressingIdentifier,
    #[serde(rename = "l")]
    pub languages: Vec<Language>,
    #[said]
    #[serde(rename = "d")]
    #[serde(deserialize_with = "opt_serialization::empty_str_as_none")]
    pub said: Option<said::SelfAddressingIdentifier>,
    #[serde(rename = "p")]
    pub pages: Vec<Page>,
    #[serde(rename = "po")]
    pub pages_order: Vec<String>,
    #[serde(rename = "pl")]
    pub pages_label: IndexMap<Language, BTreeMap<String, String>>,
    #[serde(rename = "i")]
    pub interaction: Vec<Interaction>,
}

impl Presentation {
    pub fn validate_digest(&self) -> Result<(), PresentationError> {
        let code = HashFunctionCode::Blake3_256;
        let format = SerializationFormats::JSON;
        let der_data = self.derivation_data(&code, &format);
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
    pub attr_properties: IndexMap<String, AttrType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "t")]
pub enum AttrType {
    TextArea,
    Signature {
        #[serde(skip_serializing_if = "Option::is_none")]
        m: Option<SignatureMetadata>,
    },
    File,
    Radio {
        o: Orientation,
    },
    Time,
    DateTime,
    Date,
    #[serde(rename = "code_scanner")]
    CodeScanner,
    Select {
        va: Cardinality,
    },
    Number {
        r: Range,
        s: f32,
    },
    Question {
        answer: String,
        o: IndexMap<String, Vec<String>>,
    },
}

#[derive(Debug, Deserialize, Clone)]
pub struct Range([Option<f32>; 2]);
use serde::ser::SerializeSeq;
impl Serialize for Range {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize the Range as an array of two elements
        let mut seq = serializer.serialize_seq(Some(2))?;
        let first = &self.0[0];
        if first.map(|i| i.fract()) == Some(0.0) {
            seq.serialize_element(&(first.map(|i| i as i32)))?;
        } else {
            seq.serialize_element(&first)?;
        }
        let second = &self.0[1];
        if second.map(|i| i.fract()) == Some(0.0) {
            seq.serialize_element(&(second.map(|i| i as i32)))?;
        } else {
            seq.serialize_element(&second)?;
        }

        seq.end()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SignatureMetadata {
    canvas: String,
    geolocation: Geolocation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Geolocation {
    latitude: String,
    longitude: String,
    accuracy: String,
    timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Cardinality {
    Multiple,
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

        let mut pages_label = IndexMap::new();
        let mut pages_label_en = BTreeMap::new();
        pages_label_en.insert("pageY".to_string(), "Page Y".to_string());
        pages_label_en.insert("pageZ".to_string(), "Page Z".to_string());
        pages_label.insert(Language::Eng, pages_label_en);

        let mut presentation_base = Presentation {
            version: "1.0.0".to_string(),
            bundle_digest: "EHp19U2U1sdOBmPzMmILM3DUI0PQph9tdN3KtmBrvNV7"
                .parse()
                .unwrap(),
            languages: vec![Language::Eng, Language::Pol, Language::Deu],
            said: None,
            pages,
            pages_order: vec!["pageY".to_string(), "pageZ".to_string()],
            pages_label,
            interaction: vec![Interaction {
                interaction_method: InteractionMethod::Web,
                context: Context::Capture,
                attr_properties: vec![(
                    "attr_1".to_string(),
                    AttrType::Radio {
                        o: Orientation::Horizontal,
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
            "EOiPlSDMJlllCZHT4skyPLlpy0tOXsOJNxP2ifhexL4b".to_string()
        );
    }

    #[test]
    fn test_deserialize() {
        let input = r#"{
  "v":"1.0.0",
  "bd": "EHp19U2U1sdOBmPzMmILM3DUI0PQph9tdN3KtmBrvNV7",
  "l": ["eng", "pol", "deu"],
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

        let mut serialized = serde_json::to_string_pretty(&pres).unwrap();
        serialized.retain(|c| !c.is_whitespace());
        let mut expected = input.to_string();
        expected.retain(|c| !c.is_whitespace());

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_complex_deserialize() {
        let input = r#"{
  "v": "1.0.0",
  "bd": "EIRYpj7kwFW1nJ9AInPgMjsdC-DeX26eHlb7FzwzlkEh",
  "l": [
    "eng", 
    "pol", 
    "deu"
    ],
  "d": "",
  "p": [
    {
      "n": "page 2",
      "ao": [
        "select", 
        "i", 
        "img", 
        "num", 
        "date", 
        "time", 
        "nice_attr"
        ]
    },
    {
      "n": "page 1",
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
              "ao": [
                "floors", 
                "area", 
                { 
                  "n": "address", 
                  "ao": [
                    "city", 
                    "zip", 
                    "street"
                    ] 
                }
                ]
            }
          ]
        }
      ]
    },
    {
      "n": "page 3",
      "ao": [
        "list_text",
        "list_num",
        "list_bool",
        "list_date",
        {
          "n": "devices",
          "ao": [
            "name",
            "description",
            {
              "n": "manufacturer",
              "ao": [
                "name",
                { 
                  "n": "address", 
                  "ao": ["city", "zip"] 
                },
                { 
                  "n": "parts", 
                  "ao": ["name"] 
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "n": "page 4",
      "ao": [
        "text_attr1", 
        "radio1", 
        "text_attr2", 
        "radio2"
        ]
    }
  ],
  "po": [
    "page 1", 
    "page 2", 
    "page 3", 
    "page 4"
    ],
  "pl": {
    "eng": {
      "page 1": "First page",
      "page 2": "Second page",
      "page 3": "Third page",
      "page 4": "Radio/checkbox page"
    },
    "pol": {
      "page 1": "Pierwsza strona",
      "page 2": "Druga strona",
      "page 3": "Trzecia strona",
      "page 4": "Radio/checkbox strona"
    },
    "deu": {
      "page 1": "Erste Seite",
      "page 2": "Zweite Seite",
      "page 3": "Dritte Seite",
      "page 4": "Radio/checkbox Seite"
    }
  },
  "i": [
    {
      "m": "web",
      "c": "capture",
      "a": {
        "d": { 
          "t": "textarea" 
        },
        "img": { 
          "t": "file" 
        },
        "sign": { 
          "t": "signature" 
        },
        "radio1": { 
          "t": "radio", 
          "o": "vertical" 
        },
        "radio2": { 
          "t": "radio", 
          "o": "horizontal" 
        },
        "date": { 
          "t": "date" 
        },
        "time": { 
          "t": "time" 
        },
        "list_date": { 
          "t": "datetime" 
        },
        "customer.building.address.street": { 
          "t": "textarea" 
        },
        "question1": {
          "t": "question",
          "answer": "r",
          "o": { "no": ["on_no_what", "on_no_when"], "maybe": ["on_maybe"] }
        }
      }
    }
  ]
}
"#;

        let pres: Presentation = serde_json::from_str(input).unwrap();
        assert!(pres.said.is_none());

        let mut serialized = serde_json::to_string_pretty(&pres).unwrap();
        serialized.retain(|c| !c.is_whitespace());
        let mut expected = input.to_string();
        expected.retain(|c| !c.is_whitespace());

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_attribute() {
        let attr_str = r#"{
                "t": "question",
                "answer": "r",
                "o": { "no": ["on_no_what", "on_no_when"], "maybe": ["on_maybe"] }
              }"#;
        let attr: AttrType = serde_json::from_str(&attr_str).unwrap();
        assert!(matches!(attr, AttrType::Question { answer: _, o: _ }))
    }
}
