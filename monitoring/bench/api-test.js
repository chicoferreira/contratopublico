// Import necessary modules
import { check } from "k6";
import http from "k6/http";

export const options = {
  // define thresholds
  thresholds: {
    http_req_failed: ["rate<0.01"], // http errors should be less than 1%
    http_req_duration: ["p(95)<1000"], // 95% of requests should be below 1s
  },
  scenarios: {
    search_load_test: {
      executor: "ramping-vus",
      stages: [
        { duration: "5s", target: 1250 },
        { duration: "50s", target: 1250 },
        { duration: "5s", target: 0 },
      ],
    },
  },
};

const searchQueries = [
  "construção",
  "serviços",
  "limpeza",
  "manutenção",
  "fornecimento",
  "consultoria",
  "projeto",
  "energia",
  "transporte",
  "segurança",
  "lda",
  "sa",
  "lisboa",
  "porto",
  "test",
  "",
];

function generateGibberish() {
  const chars = "abcdefghijklmnopqrstuvwxyz";
  const length = Math.floor(Math.random() * 10) + 3;
  let result = "";
  for (let i = 0; i < length; i++) {
    result += chars.charAt(Math.floor(Math.random() * chars.length));
  }
  return result;
}

const sortOptions = [
  { field: "publicationDate", direction: "descending" },
  { field: "publicationDate", direction: "ascending" },
  { field: "signingDate", direction: "descending" },
  { field: "price", direction: "descending" },
  { field: "id", direction: "ascending" },
];

export default function () {
  const baseUrl = "http://localhost/api/search";

  const query =
    Math.random() < 0.3
      ? generateGibberish()
      : searchQueries[Math.floor(Math.random() * searchQueries.length)];
  const sort =
    Math.random() > 0.5
      ? sortOptions[Math.floor(Math.random() * sortOptions.length)]
      : null;
  const page = Math.random() > 0.8 ? Math.floor(Math.random() * 3) + 1 : 1;

  const body = { query: query };

  if (sort) {
    body.sort = sort;
  }

  if (page > 1) {
    body.page = page;
  }

  const params = {
    headers: {
      "Content-Type": "application/json",
    },
  };

  const res = http.post(baseUrl, JSON.stringify(body), params);

  check(res, {
    "response code was 200": (res) => res.status === 200,
    "response time < 1000ms": (res) => res.timings.duration < 1000,
  });
}
