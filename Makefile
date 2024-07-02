$PHONY: up, down

up:
	migrate -path migrations/ -database "postgresql://$(DB_USER):$(DB_PASSWORD)@localhost:5432/word_counter?sslmode=disable" -verbose up