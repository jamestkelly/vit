# Usage:
# make              # Runs all except shutdown
# make shutdown     # Shuts down the stack

.PHONY: all dev shutdown

all: update build_migration_container initialise_stack migrate_database clean_up exit

update:
	@echo Checking for latest application version.
	@git fetch
	@echo Pulling latest changes to local repository.
	@git pull
	@echo Completed update process.

build_migration_container:
	@echo Starting demonstration application stack.
	@echo Building migration container image.
	@echo NO MIGRATION IMAGE YET: docker build -t vit-migration-container -f docker/migration-dockerfile .

initialise_stack:
	@echo Initialising Vit application compose project.
	@docker compose up -d
	@echo Haast Reporter compose project initialised.

migrate_database:
	@echo Migrating demonstration environment database.
	@docker run --rm --env-file .env --network=vit_default vit-migration-container
	@echo Migration complete.

clean_up:
	@echo Removing migration containers.
	@docker rmi -f vit-migration-container || echo "Migration image not found."
	@echo Successfully removed migration containers.

start_app:
	@echo Starting Vit application.
	@cargo run

exit:
	@echo Completed initialisation of Haast Reporter demonstration stack.

shutdown:
	@echo Shutting down Haast Reporter demonstration stack.
	@docker compose -f demo-docker-compose.yml down
	@echo Successfully shut down Haast Reporter demonstration stack.
