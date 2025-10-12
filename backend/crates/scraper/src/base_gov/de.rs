use chrono::NaiveDate;
use common::Currency;
use serde::{Deserialize, Deserializer};

use crate::base_gov::BaseGovCpv;

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: String = Deserialize::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&date_str, "%d-%m-%Y").map_err(serde::de::Error::custom)
}

pub fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: Option<String> = Deserialize::deserialize(deserializer)?;
    date_str
        .map(|s| NaiveDate::parse_from_str(&s, "%d-%m-%Y").map_err(serde::de::Error::custom))
        .transpose()
}

pub fn deserialize_euros<'de, D>(deserializer: D) -> Result<Currency, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // 5.611,10 €
    let euros_str: String = Deserialize::deserialize(deserializer)?;
    let euros_str = euros_str
        .trim()
        .trim_end_matches(" €")
        .replace(".", "")
        .replace(",", "");

    let big_int: isize = euros_str.parse().map_err(serde::de::Error::custom)?;

    Ok(Currency(big_int))
}

pub fn deserialize_optional_euros<'de, D>(deserializer: D) -> Result<Option<Currency>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    // 5.611,10 €
    let euros_str: Option<String> = Deserialize::deserialize(deserializer)?;
    euros_str
        .map(|euros_str| {
            let processed = euros_str
                .trim()
                .trim_end_matches(" €")
                .replace(".", "")
                .replace(",", "");
            let big_int: isize = processed.parse().map_err(serde::de::Error::custom)?;
            Ok(Currency(big_int))
        })
        .transpose()
}

pub fn deserialize_execution_deadline<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let date_str: String = Deserialize::deserialize(deserializer)?;
    let date_str = date_str.trim().trim_end_matches(" dias");

    let days: usize = date_str.parse().map_err(serde::de::Error::custom)?;

    Ok(days)
}

pub fn deserialize_announcement_id<'de, D>(deserializer: D) -> Result<Option<usize>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let id_str: isize = Deserialize::deserialize(deserializer)?;
    if id_str >= 0 {
        Ok(Some(id_str as usize))
    } else {
        Ok(None)
    }
}

pub fn empty_vec_if_null<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::<Vec<T>>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

pub fn deserialize_cpv_none_if_empty<'de, D>(
    deserializer: D,
) -> Result<Option<BaseGovCpv>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct CpvFields {
        #[serde(rename = "cpvs")]
        code: Option<String>,
        #[serde(rename = "cpvsDesignation")]
        designation: Option<String>,
    }

    let fields: CpvFields = Deserialize::deserialize(deserializer)?;

    match (fields.code, fields.designation) {
        (Some(code), Some(designation)) if !code.is_empty() && !designation.is_empty() => {
            Ok(Some(BaseGovCpv {
                code: code.to_string(),
                designation: designation.to_string(),
            }))
        }
        _ => Ok(None),
    }
}
