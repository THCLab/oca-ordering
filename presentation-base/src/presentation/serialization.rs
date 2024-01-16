use super::Presentation;
use serde::{ser::SerializeStruct, Serialize};

impl Serialize for Presentation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // 8 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Presentation", 8)?;
        state.serialize_field("v", &self.version)?;
        state.serialize_field("bd", &self.bundle_digest)?;
        state.serialize_field("l", &self.languages)?;
        let said = opt_serialization::none_as_empty_string(&self.said);
        state.serialize_field("d", &said)?;
        state.serialize_field("p", &self.pages)?;
        state.serialize_field("po", &self.pages_order)?;
        state.serialize_field("pl", &self.pages_label)?;
        state.serialize_field("i", &self.interaction)?;
        state.end()
    }
}

pub mod opt_serialization {
    use std::fmt::Display;

    use said::SelfAddressingIdentifier;
    use serde::{de::Error, Deserialize, Deserializer};

    pub fn none_as_empty_string<T>(input: &Option<T>) -> String
    where
        T: Display,
    {
        input
            .as_ref()
            .map(|said| said.to_string())
            .unwrap_or_default()
    }

    pub fn empty_str_as_none<'de, D>(
        deserializer: D,
    ) -> Result<Option<SelfAddressingIdentifier>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.is_empty() {
            Ok(None)
        } else {
            let parsed = s.parse().map_err(D::Error::custom)?;
            Ok(Some(parsed))
        }
    }
}
