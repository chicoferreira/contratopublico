import type { MatchingRange } from "$lib/types/api";

export type TreeNode = {
  id: string;
  name: string;
  searchName: string;
  depth: number;
  parent?: TreeNode;
  children: TreeNode[];
};

export type Tree = {
  nodes: Map<string, TreeNode>;
  roots: TreeNode[];
};

export function addNode(tree: Tree, id: string, name: string, depth: number, parent?: TreeNode) {
  const node: TreeNode = { id, name, searchName: foldAccents(name), depth, parent, children: [] };
  const siblings = parent ? parent.children : tree.roots;
  siblings.push(node);
  tree.nodes.set(id, node);
  return node;
}

export function treePath(node: TreeNode): TreeNode[] {
  return node.parent ? [...treePath(node.parent), node] : [node];
}

export function foldAccents(text: string) {
  return text
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();
}

export function searchByName(tree: Tree, query: string) {
  const text = foldAccents(query.trim());
  if (!text) return [];
  const words = text.split(/\s+/);

  const rank = (node: TreeNode) => {
    if (node.searchName === text) return 0;
    if (node.searchName.startsWith(words[0])) return 1;
    return 2;
  };

  return [...tree.nodes.values()]
    .filter((node) => words.every((word) => node.searchName.includes(word)))
    .sort((a, b) => rank(a) - rank(b) || a.depth - b.depth);
}

export function treeView(tree: Tree, matches: TreeNode[] | undefined, toggled: Set<string>) {
  const matched = new Set(matches);
  const onPathToMatch = new Set(matches?.flatMap((match) => treePath(match)));

  const childrenOf = (node: TreeNode) => {
    const onPath = node.children.filter((child) => onPathToMatch.has(child));
    return onPath.length > 0 ? onPath : node.children;
  };
  const isOpen = (node: TreeNode) => {
    if (node.children.length === 0) return false;
    const openBySearch = node.children.some((child) => onPathToMatch.has(child));
    return toggled.has(node.id) ? !openBySearch : openBySearch;
  };

  const rows: TreeNode[] = [];
  const addRows = (nodes: TreeNode[]) => {
    for (const node of nodes) {
      rows.push(node);
      if (isOpen(node)) addRows(childrenOf(node));
    }
  };

  const roots = matches ? tree.roots.filter((root) => onPathToMatch.has(root)) : tree.roots;
  addRows(roots);
  return { rows, roots, matched, childrenOf, isOpen };
}

// These take ids, as the selection comes from the URL and can be there before the tree loads
function covers(tree: Tree | undefined, ancestorId: string, id: string) {
  if (ancestorId === id) return true;
  let parent = tree?.nodes.get(id)?.parent;
  while (parent) {
    if (parent.id === ancestorId) return true;
    parent = parent.parent;
  }
  return false;
}

export function selectionState(tree: Tree | undefined, id: string, selected: string[]) {
  if (selected.some((other) => other !== id && covers(tree, other, id))) return "covered";
  if (selected.includes(id)) return "selected";
  if (selected.some((other) => covers(tree, id, other))) return "partial";
  return "unselected";
}

export function toggleSelection(
  tree: Tree | undefined,
  id: string,
  selected: string[],
  max: number,
) {
  const state = selectionState(tree, id, selected);
  if (state === "covered") return selected;

  const remaining = selected.filter((other) => !covers(tree, id, other));
  if (state === "selected") return remaining;
  if (remaining.length >= max) return selected;
  return [...remaining, id].sort();
}

const encoder = new TextEncoder();

export function nameMatchRanges(name: string, query: string): MatchingRange[] {
  const words = foldAccents(query.trim()).split(/\s+/).filter(Boolean);
  if (words.length === 0) return [];

  let folded = "";
  const byteRanges: MatchingRange[] = [];
  let start = 0;
  for (const char of name) {
    const end = start + encoder.encode(char).length;
    for (const foldedChar of foldAccents(char)) {
      folded += foldedChar;
      byteRanges.push({ start, end });
    }
    start = end;
  }

  const matched = new Set<number>();
  for (const word of words) {
    let position = folded.indexOf(word);
    while (position !== -1) {
      for (let i = position; i < position + word.length; i++) matched.add(i);
      position = folded.indexOf(word, position + word.length);
    }
  }

  const ranges: MatchingRange[] = [];
  for (const [i, { start, end }] of byteRanges.entries()) {
    if (!matched.has(i)) continue;
    const previous = ranges.at(-1);
    if (previous && (matched.has(i - 1) || previous.end >= start)) previous.end = end;
    else ranges.push({ start, end });
  }
  return ranges;
}
