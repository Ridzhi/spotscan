#!make
ENV_FILE = .env
DC_CLI = docker compose --env-file=$(ENV_FILE) --profile dev
#
## to share env here
include $(ENV_FILE)
#
PG_DSN = postgres://$(SPOTSCAN_PG_USER):$(SPOTSCAN_PG_PASSWORD)@localhost:$(SPOTSCAN_PG_PORT)/$(SPOTSCAN_PG_DBNAME)?sslmode=disable
## to share env in program
export $(shell sed 's/=.*//' $(ENV_FILE))
export DB_URI=$(PG_DSN)

default:
$(info Commands: up,down,migrate)

up:
	$(DC_CLI) up -d

down:
	$(DC_CLI) down

migrate:
	refinery migrate -e DB_URI -p ./migrations

api:
	openapi-generator generate -i docs/openapi.json -g typescript-axios -o web/src/utils/openapi

cargo-build:
	cargo build --bin api --release
	cargo build --bin bot --release
	cargo build --bin spot --release