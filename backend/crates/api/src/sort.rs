use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct SortBy {
    pub field: SortField,
    pub direction: SortDirection,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SortDirection {
    Ascending,
    #[default]
    Descending,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SortField {
    Id,
    #[default]
    PublicationDate,
    SigningDate,
    Price,
}

impl SortField {
    pub fn to_meilisearch(&self) -> &'static str {
        match self {
            SortField::Id => "id",
            SortField::PublicationDate => "publicationDate",
            SortField::SigningDate => "signingDate",
            SortField::Price => "initialContractualPrice",
        }
    }

    pub fn to_meilisearch_all() -> Vec<&'static str> {
        vec![
            Self::Id.to_meilisearch(),
            Self::PublicationDate.to_meilisearch(),
            Self::SigningDate.to_meilisearch(),
            Self::Price.to_meilisearch(),
        ]
    }
}

impl SortBy {
    pub fn to_meilisearch(&self) -> &'static [&'static str] {
        use SortDirection::*;
        use SortField::*;

        match (&self.field, &self.direction) {
            (Id, Ascending) => &["id:asc"],
            (Id, Descending) => &["id:desc"],
            (PublicationDate, Ascending) => &["publicationDate:asc", "id:asc"],
            (PublicationDate, Descending) => &["publicationDate:desc", "id:desc"],
            (SigningDate, Ascending) => &["signingDate:asc", "id:asc"],
            (SigningDate, Descending) => &["signingDate:desc", "id:desc"],
            (Price, Ascending) => &["initialContractualPrice:asc"],
            (Price, Descending) => &["initialContractualPrice:desc"],
        }
    }
}
