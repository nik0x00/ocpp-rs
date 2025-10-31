use super::data_types::DateTimeWrapper;
// New type pattern to implement Arbitrary for DateTime

// Serializer for serde that forces to be in the format of ISO8601
pub(crate) mod iso8601_date_time {
    use alloc::{format, string::String};
    use chrono::DateTime;
    use serde::{self, Deserialize, Serializer, Deserializer};
    use super::DateTimeWrapper;

    pub fn serialize<S>(
        date: &DateTimeWrapper,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.inner().format("%+"));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<DateTimeWrapper, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
        Ok(DateTimeWrapper::new(dt.to_utc()))
    }
}

pub(crate) mod iso8601_date_time_optional {
    use alloc::{format, string::String};
    use chrono::DateTime;
    use serde::{self, Deserialize, Serializer, Deserializer};
    use super::DateTimeWrapper;
    
    #[allow(clippy::ref_option)]
    pub fn serialize<S>(
        date: &Option<DateTimeWrapper>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let s = format!("{}", date.inner().format("%+"));
                serializer.serialize_str(&s)
            },
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTimeWrapper>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            Some(s) => {
                let dt = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
                Ok(Some(DateTimeWrapper::new(dt.to_utc())))
            },
            None => Ok(None),
        }       
    }
}
