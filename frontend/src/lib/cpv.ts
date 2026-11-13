import type { CpvList, MatchingRange } from "$lib/types/api";

export type CpvTree = {
  names: CpvList;
  children: Map<string, string[]>;
  parent: Map<string, string>;
  searchNames: Map<string, string>;
};

const CODE_SEARCH_LEVELS = 3;

function significantDigits(code: string) {
  const digits = code.slice(0, 8);
  return digits.slice(0, Math.max(2, digits.replace(/0+$/, "").length));
}

function depth(code: string) {
  return significantDigits(code).length;
}

function covers(ancestor: string, code: string) {
  return significantDigits(code).startsWith(significantDigits(ancestor));
}

function foldAccents(text: string) {
  return text
    .normalize("NFD")
    .replace(/\p{Diacritic}/gu, "")
    .toLowerCase();
}

export function buildCpvTree(names: CpvList): CpvTree {
  const codes = Object.keys(names).sort();
  const codeByDigits = new Map(codes.map((code) => [significantDigits(code), code]));
  const parent = new Map<string, string>();
  for (const code of codes) {
    let ancestor = significantDigits(code).slice(0, -1);
    while (ancestor.length >= 2 && !codeByDigits.has(ancestor)) ancestor = ancestor.slice(0, -1);
    parent.set(code, codeByDigits.get(ancestor) ?? "");
  }
  const children = Map.groupBy(codes, (code) => parent.get(code)!);
  const searchNames = new Map(codes.map((code) => [code, foldAccents(names[code])]));
  return { names, children, parent, searchNames };
}

function isCodeQuery(query: string) {
  return /^[\d\s-]+$/.test(query);
}

export function searchCpvs(tree: CpvTree, query: string) {
  const q = foldAccents(query.trim());
  if (!q) return [];
  if (isCodeQuery(q)) {
    const digits = q.replace(/[\s-]/g, "");
    const hits = [...tree.parent.keys()].filter((code) => code.replace("-", "").startsWith(digits));
    const topDepth = Math.min(...hits.map(depth));
    return hits.filter((code) => depth(code) < topDepth + CODE_SEARCH_LEVELS);
  }
  const words = q.split(/\s+/);
  const startsWithQuery = (code: string) => tree.searchNames.get(code)!.startsWith(words[0]);
  return [...tree.searchNames]
    .filter(([, name]) => words.every((word) => name.includes(word)))
    .map(([code]) => code)
    .sort(
      (a, b) =>
        Number(startsWithQuery(b)) - Number(startsWithQuery(a)) ||
        depth(a) - depth(b) ||
        a.localeCompare(b),
    );
}

export function cpvView(tree: CpvTree, hits: string[] | null, toggled: Set<string>) {
  const matched = new Set(hits);
  const onPathToHit = new Set<string>();
  for (const hit of hits ?? []) {
    for (let code = hit; code; code = tree.parent.get(code)!) onPathToHit.add(code);
  }
  const allChildren = (code: string) => tree.children.get(code) ?? [];
  const childrenOnPath = (code: string) =>
    allChildren(code).filter((child) => onPathToHit.has(child));
  const childrenOf = (code: string) => {
    const onPath = childrenOnPath(code);
    return onPath.length ? onPath : allChildren(code);
  };
  const isOpen = (code: string) => {
    if (!tree.children.has(code)) return false;
    const openBySearch = childrenOnPath(code).length > 0;
    return toggled.has(code) ? !openBySearch : openBySearch;
  };

  const rows: string[] = [];
  const walk = (codes: string[]) => {
    for (const code of codes) {
      rows.push(code);
      if (isOpen(code)) walk(childrenOf(code));
    }
  };
  const roots = hits ? childrenOnPath("") : allChildren("");
  walk(roots);
  return { rows, roots, childrenOf, isOpen, matched };
}

export function cpvState(code: string, selected: string[]) {
  if (selected.some((s) => s !== code && covers(s, code))) return "covered";
  if (selected.includes(code)) return "on";
  return selected.some((s) => covers(code, s)) ? "mixed" : "off";
}

export function toggleCpv(code: string, selected: string[], max: number) {
  const state = cpvState(code, selected);
  if (state === "covered") return selected;
  const rest = selected.filter((s) => !covers(code, s));
  if (state === "on") return rest;
  return rest.length < max ? [...rest, code].sort() : selected;
}

const encoder = new TextEncoder();

export function nameMatchRanges(text: string, query: string): MatchingRange[] {
  const words = foldAccents(query.trim()).split(/\s+/).filter(Boolean);
  if (!words.length || isCodeQuery(words.join(""))) return [];

  let folded = "";
  const byteStarts: number[] = [];
  const byteEnds: number[] = [];
  let offset = 0;
  for (const char of text) {
    const size = encoder.encode(char).length;
    for (const foldedChar of foldAccents(char)) {
      folded += foldedChar;
      byteStarts.push(offset);
      byteEnds.push(offset + size);
    }
    offset += size;
  }

  const hit: boolean[] = [];
  for (const word of words) {
    for (let i = folded.indexOf(word); i >= 0; i = folded.indexOf(word, i + word.length)) {
      for (let k = i; k < i + word.length; k++) hit[k] = true;
    }
  }

  const ranges: MatchingRange[] = [];
  for (let i = 0; i < folded.length; i++) {
    if (!hit[i]) continue;
    let j = i;
    while (hit[j + 1]) j++;
    const last = ranges.at(-1);
    if (last && last.end >= byteStarts[i]) last.end = byteEnds[j];
    else ranges.push({ start: byteStarts[i], end: byteEnds[j] });
    i = j;
  }
  return ranges;
}

export function codeMatchRanges(code: string, query: string): MatchingRange[] {
  const digits = query.replace(/[\s-]/g, "");
  if (!digits || !isCodeQuery(query.trim()) || !code.replace("-", "").startsWith(digits)) return [];
  return [{ start: 0, end: digits.length + (digits.length > 8 ? 1 : 0) }];
}

export function cpvPath(tree: CpvTree, code: string) {
  const path: string[] = [];
  for (let c = code; c; c = tree.parent.get(c) ?? "") path.unshift(c);
  return path;
}
