

.PHONY: migrate
migrate:
	sqlx migrate run
