# Small Makefile helpers to control services defined in docker-compose.yml

COMPOSE := docker compose
COMPOSE_FILE := docker-compose.yml

.PHONY: help run stop ps
.DEFAULT_GOAL := help

help:
	@echo "Usage: make <target> [service...]"
	@echo
	@echo "Targets:"
	@echo "  run <service...>    Start one or more services (e.g. make run rocksdb)"
	@echo "  stop <service...>   Stop one or more services (e.g. make stop rocksdb)"
	@echo "  ps [service...]     Show docker compose ps for the project or specific service(s)"

# Start one or more services: `make run rocksdb` or `make run rocksdb other`
run:
	@services="$(filter-out $@,$(MAKECMDGOALS))"; \
	if [ -z "$$services" ]; then \
		echo "Specify service(s): make run rocksdb"; exit 1; \
	fi; \
	for svc in $$services; do \
		echo "Starting $$svc..."; \
		$(COMPOSE) -f $(COMPOSE_FILE) up -d $$svc; \
	done

# Stop one or more services: `make stop rocksdb`
stop:
	@services="$(filter-out $@,$(MAKECMDGOALS))"; \
	if [ -z "$$services" ]; then \
		echo "Specify service(s): make stop rocksdb"; exit 1; \
	fi; \
	for svc in $$services; do \
		echo "Stopping $$svc..."; \
		$(COMPOSE) -f $(COMPOSE_FILE) stop $$svc; \
	done

# Show status
ps:
	@services="$(filter-out $@,$(MAKECMDGOALS))"; \
	if [ -z "$$services" ]; then \
		$(COMPOSE) -f $(COMPOSE_FILE) ps; \
	else \
		$(COMPOSE) -f $(COMPOSE_FILE) ps $$services; \
	fi

