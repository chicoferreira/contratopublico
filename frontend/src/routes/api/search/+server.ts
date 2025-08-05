import { env } from "$env/dynamic/private";

const BACKEND_URL = env.BACKEND_URL || "http://localhost:3000";

// Only used in SSR and development.
// In SSR the svelte fetch function will run this function automatically without doing any network calls.
// In production and client-side we use rpxy and will redirect /api client fetch requests to the backend
export async function POST({ request }) {
  return await fetch(`${BACKEND_URL}/api/search`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(await request.json()),
  });
}
