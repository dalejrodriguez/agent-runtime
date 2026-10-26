FROM node:24-alpine AS build
WORKDIR /src
COPY . .
RUN npm ci && npm run build

FROM node:24-alpine
COPY --from=build --chmod=755 /src/dist/main.js /readme-fetch
ENTRYPOINT ["/readme-fetch", "idle"]
