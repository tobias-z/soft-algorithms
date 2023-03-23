#!/usr/bin/env bash
DB_PATH="postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@${POSTGRES_HOST}:${POSTGRES_PORT}/${POSTGRES_DATABASE}" refinery migrate -e DB_PATH -p /migrations
graph-server
