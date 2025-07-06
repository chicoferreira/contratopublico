use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub id: usize,
    pub contracting_procedure_type: String,
    #[serde(
        deserialize_with = "deserialize_date",
        serialize_with = "serialize_date"
    )]
    pub publication_date: NaiveDate,
    #[serde(
        deserialize_with = "deserialize_optional_date",
        serialize_with = "serialize_optional_date"
    )]
    pub signing_date: Option<NaiveDate>,
    pub ccp: bool,
    pub contracted: String,
    pub contracting: String,
    pub object_brief_description: String,
    pub initial_contractual_price: String,
}

fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").map_err(serde::de::Error::custom)
}

fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: Option<String> = Deserialize::deserialize(deserializer)?;
    Ok(date_str
        .map(|date_str| {
            NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").map_err(serde::de::Error::custom)
        })
        .transpose()?)
}

fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format("%d-%m-%Y").to_string())
}

fn serialize_optional_date<S>(date: &Option<NaiveDate>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date {
        Some(d) => serializer.serialize_str(&d.format("%d-%m-%Y").to_string()),
        None => serializer.serialize_none(),
    }
}
