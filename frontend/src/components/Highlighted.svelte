<script lang="ts">
  import type { MatchingRange } from "$lib/types/api";
  import { cn, type WithElementRef } from "$lib/utils.js";

  const {
    content,
    ranges,
    index,
    class: className,
  }: {
    content?: string;
    ranges?: MatchingRange[];
    index?: number;
    class?: string;
  } = $props();

  const encoder = new TextEncoder();
  const decoder = new TextDecoder();

  // meilisearch returns the matched ranges as byte positions not character positions
  function sliceByBytes(str: string, start: number, end?: number): string {
    const bytes = encoder.encode(str);
    return decoder.decode(bytes.subarray(start, end));
  }

  type TextSegment = {
    text: string;
    highlighted: boolean;
  };

  function createHighlightedSegments(text: string, matchRanges: MatchingRange[]): TextSegment[] {
    if (!matchRanges || matchRanges.length === 0) {
      return [{ text, highlighted: false }];
    }

    const segments: TextSegment[] = [];
    let currentIndex = 0;

    for (const range of matchRanges) {
      if (index !== undefined && range.indices && !range.indices.includes(index)) {
        // is list and this range is not for the current index
        continue;
      }

      const beforeText = sliceByBytes(text, currentIndex, range.start);
      segments.push({ text: beforeText, highlighted: false });

      const highlightedText = sliceByBytes(text, range.start, range.end);
      segments.push({ text: highlightedText, highlighted: true });

      currentIndex = range.end;
    }

    const remainingText = sliceByBytes(text, currentIndex);
    segments.push({ text: remainingText, highlighted: false });

    return segments;
  }

  const highlightedSegments = $derived(
    content ? createHighlightedSegments(content, ranges || []) : [],
  );
</script>

{#each highlightedSegments as segment}
  {#if segment.highlighted}
    <mark class={cn("rounded bg-yellow-200 px-0", className)}>{segment.text}</mark>
  {:else}
    <span class={className}>{segment.text}</span>
  {/if}
{/each}
