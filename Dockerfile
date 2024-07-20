ARG BASE_WEB_IMAGE=node
ARG BASE_WEB_VERSION=18

FROM ${BASE_WEB_IMAGE}:${BASE_WEB_VERSION} AS web_builder

WORKDIR /app
USER 0

COPY client/package*.json ./

RUN npm install

COPY client/ ./

RUN npm run build
