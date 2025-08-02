import { json } from "@sveltejs/kit";
import { env } from "$env/dynamic/private";

const BACKEND_URL = env.BACKEND_URL || "http://localhost:3000";

// Only used in SSR and development.
// In SSR the svelte fetch function will run this function automatically without doing any network calls.
// In production and client-side we use rpxy and client fetch requests will redirect /api to the backend
export async function POST({ request }) {
  const body = await request.json();

  const response = await fetch(`${BACKEND_URL}/api/search`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(body),
  });

  return json(await response.json());
}
