FROM node:16.20.0-alpine3.17 as builder
WORKDIR /build
COPY . .
RUN npm install
RUN VITE_GRAPH_SERVER=https://graph-api.tobiasz.dk npm run build

FROM nginx:alpine
WORKDIR /app
COPY --from=builder /build/dist .
COPY ./nginx.conf /etc/nginx/nginx.conf
