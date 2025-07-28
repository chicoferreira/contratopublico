use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
pub struct SortBy {
    #[serde(rename = "sortField")]
    pub field: SortField,
    #[serde(rename = "sortDirection")]
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
    pub fn to_meilisearch(&self) -> &'static str {
        match (&self.field, &self.direction) {
            (SortField::Id, SortDirection::Ascending) => "id:asc",
            (SortField::Id, SortDirection::Descending) => "id:desc",
            (SortField::PublicationDate, SortDirection::Ascending) => "publicationDate:asc",
            (SortField::PublicationDate, SortDirection::Descending) => "publicationDate:desc",
            (SortField::SigningDate, SortDirection::Ascending) => "signingDate:asc",
            (SortField::SigningDate, SortDirection::Descending) => "signingDate:desc",
            (SortField::Price, SortDirection::Ascending) => "initialContractualPrice:asc",
            (SortField::Price, SortDirection::Descending) => "initialContractualPrice:desc",
        }
    }
}
