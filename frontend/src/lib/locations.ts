import { fetchLocations } from "$lib";
import type { LocationNode } from "$lib/types/api";

let locationTree: Promise<LocationNode[] | undefined> | undefined;

export function loadLocationTree() {
  locationTree ??= fetchLocations().then((result) => {
    if (result.ok) return result.data;
    console.error("Failed to load locations:", result.message);
    locationTree = undefined;
  });
  return locationTree;
}
