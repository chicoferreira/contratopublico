import { fetchCpvs } from "$lib";
import {
  addNode,
  foldAccents,
  nameMatchRanges,
  searchByName,
  type Tree,
  type TreeNode,
} from "$lib/tree";
import type { CpvList, MatchingRange } from "$lib/types/api";

let cpvTree: Promise<Tree | undefined> | undefined;

export function loadCpvTree() {
  cpvTree ??= fetchCpvs().then((result) => {
    if (result.ok) return buildCpvTree(result.data);
    console.error("Failed to load CPVs:", result.message);
    cpvTree = undefined;
  });
  return cpvTree;
}

const CODE_DIGITS = 8;
const DIVISION_DIGITS = 2;
const CODE_SEARCH_LEVELS = 3;

function significantDigits(code: string) {
  const withoutTrailingZeros = code.slice(0, CODE_DIGITS).replace(/0+$/, "");
  return withoutTrailingZeros.padEnd(DIVISION_DIGITS, "0");
}

function closestParent(nodeByDigits: Map<string, TreeNode>, digits: string) {
  for (let length = digits.length - 1; length >= DIVISION_DIGITS; length--) {
    const parent = nodeByDigits.get(digits.slice(0, length));
    if (parent) return parent;
  }
}

export function buildCpvTree(designations: CpvList): Tree {
  const tree: Tree = { nodes: new Map(), roots: [] };
  const nodeByDigits = new Map<string, TreeNode>();
  // Sorted, so parents are added before their children
  for (const code of Object.keys(designations).sort()) {
    const digits = significantDigits(code);
    const parent = closestParent(nodeByDigits, digits);
    nodeByDigits.set(digits, addNode(tree, code, designations[code], digits.length, parent));
  }
  return tree;
}

function isCodeQuery(query: string) {
  return /^[\d\s-]+$/.test(query);
}

export function searchCpvs(tree: Tree, query: string) {
  const text = foldAccents(query.trim());
  if (!isCodeQuery(text)) return searchByName(tree, text);

  const digits = text.replace(/[\s-]/g, "");
  const matches = [...tree.nodes.values()].filter((node) =>
    node.id.replace("-", "").startsWith(digits),
  );
  const topDepth = Math.min(...matches.map((node) => node.depth));
  return matches.filter((node) => node.depth < topDepth + CODE_SEARCH_LEVELS);
}

export function designationMatchRanges(designation: string, query: string) {
  return isCodeQuery(foldAccents(query.trim())) ? [] : nameMatchRanges(designation, query);
}

export function codeMatchRanges(code: string, query: string): MatchingRange[] {
  if (!isCodeQuery(query.trim())) return [];
  const digits = query.replace(/[\s-]/g, "");
  if (!digits || !code.replace("-", "").startsWith(digits)) return [];

  const includesHyphen = digits.length > CODE_DIGITS;
  return [{ start: 0, end: includesHyphen ? digits.length + 1 : digits.length }];
}
