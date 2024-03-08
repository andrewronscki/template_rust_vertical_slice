#!make
.PHONY: up down rmv swag

infra:
	docker compose -f ./docker-compose.yml up -d
	sleep 5
	docker exec -it postgres-rust-project psql -U postgres -d rustProject -c 'CREATE EXTENSION IF NOT EXISTS "uuid-ossp";'

up:
	docker compose -f ./docker-compose.yml up -d

down:
	docker compose -f ./docker-compose.yml down

rmv:
	docker volume rm $$(docker volume ls -q)

run:
	RUST_LOG=debug cargo watch -x run 