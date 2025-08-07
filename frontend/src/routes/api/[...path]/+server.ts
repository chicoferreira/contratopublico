import { env } from "$env/dynamic/private";
import type { RequestHandler } from "./$types";

const BACKEND_URL = env.BACKEND_URL || "http://localhost:3000";

// Catch-all API proxy for all routes starting with /api
// Only used in SSR and development.
// In SSR the svelte fetch function will run this function automatically without doing any network calls.
// In production and client-side we use rpxy and will redirect /api client fetch requests to the backend

async function proxyRequest(method: string, path: string, request: Request) {
  const url = `${BACKEND_URL}/api/${path}`;

  const options: RequestInit = {
    method,
    headers: request.headers,
  };

  if (method === "POST") {
    options.body = await request.text();
  }

  return await fetch(url, options);
}

export const GET: RequestHandler = async ({ params, request }) => {
  return await proxyRequest("GET", params.path, request);
};

export const POST: RequestHandler = async ({ params, request }) => {
  return await proxyRequest("POST", params.path, request);
};
