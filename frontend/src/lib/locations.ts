import { fetchLocations } from "$lib";
import { addNode, type Tree, type TreeNode } from "$lib/tree";
import type { LocationNode } from "$lib/types/api";

export const PORTUGAL = "portugal";

let locationTree: Promise<Tree | undefined> | undefined;

export function loadLocationTree() {
  locationTree ??= fetchLocations().then((result) => {
    if (result.ok) return buildLocationTree(result.data);
    console.error("Failed to load locations:", result.message);
    locationTree = undefined;
  });
  return locationTree;
}

function buildLocationTree(countries: LocationNode[]): Tree {
  const tree: Tree = { nodes: new Map(), roots: [] };

  const addPlace = (place: LocationNode, depth: number, parent?: TreeNode) => {
    const node = addNode(tree, place.id, place.label, depth, parent);
    for (const child of place.children) addPlace(child, depth + 1, node);
  };

  const portugal = countries.filter((country) => country.id === PORTUGAL);
  const others = countries.filter((country) => country.id !== PORTUGAL);
  for (const country of [...portugal, ...others]) addPlace(country, 1);

  return tree;
}

export function isDistrictId(id: string) {
  return id.endsWith("District");
}

export function placeKind(node: TreeNode) {
  if (isDistrictId(node.id)) return "(distrito)";
  if (node.parent?.name === node.name) return "(concelho)";
}

export function placeDetail(node: TreeNode) {
  const kind = placeKind(node);
  if (kind) return kind;

  const isMunicipality = node.parent?.parent !== undefined;
  if (isMunicipality) return `· ${node.parent?.name}`;
}
