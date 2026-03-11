import type { HandleFetch } from "@sveltejs/kit";

const FORWARDED_HEADERS = [
  "cf-connecting-ip",
  "cf-ipcountry",
  "cf-ray",
  "x-forwarded-for",
  "user-agent",
];

export const handleFetch: HandleFetch = async ({ event, request, fetch }) => {
  if (new URL(request.url).pathname.startsWith("/api/")) {
    for (const header of FORWARDED_HEADERS) {
      const value = event.request.headers.get(header);
      if (value) request.headers.set(header, value);
    }
  }

  return fetch(request);
};
