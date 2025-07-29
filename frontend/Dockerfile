FROM oven/bun:1 AS builder
WORKDIR /app
COPY package*.json bun.lock* ./
RUN bun install --frozen-lockfile
COPY . .
RUN bun --bun run build
RUN bun install --production --frozen-lockfile

FROM oven/bun:1
WORKDIR /app
COPY --from=builder /app/build build/
COPY --from=builder /app/node_modules node_modules/
COPY --from=builder /app/package.json .
ENV NODE_ENV=production
CMD ["bun", "--bun", "run", "build/index.js"]
