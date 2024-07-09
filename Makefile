$PHONY: up, down

up:
	migrate -path migrations/ -database "postgresql://$(DB_USER):$(DB_PASSWORD)@$(DB_HOST):5432/word_counter?sslmode=disable" -verbose up

down:
	migrate -path migrations/ -database "postgresql://$(DB_USER):$(DB_PASSWORD)@$(DB_HOST):5432/word_counter?sslmode=disable" -verbose down --all

