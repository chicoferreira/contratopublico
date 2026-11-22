//! Builds the tree of countries, regions and municipalities returned by `GET /api/locations`.

use common::location::{Country, ExecutionPlace, Municipality, Region};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LocationNode {
    id: ExecutionPlace,
    label: &'static str,
    children: Vec<LocationNode>,
}

pub fn location_tree() -> Vec<LocationNode> {
    Country::ALL.iter().copied().map(country_node).collect()
}

fn country_node(country: Country) -> LocationNode {
    LocationNode {
        id: ExecutionPlace::Country(country),
        label: country.label(),
        children: Region::ALL
            .iter()
            .copied()
            .filter(|region| region.country() == country)
            .map(region_node)
            .collect(),
    }
}

fn region_node(region: Region) -> LocationNode {
    LocationNode {
        id: ExecutionPlace::Region(region),
        label: region.label(),
        children: Municipality::ALL
            .iter()
            .copied()
            .filter(|municipality| municipality.region() == region)
            .map(municipality_node)
            .collect(),
    }
}

fn municipality_node(municipality: Municipality) -> LocationNode {
    LocationNode {
        id: ExecutionPlace::Municipality(municipality),
        label: municipality.label(),
        children: Vec::new(),
    }
}
