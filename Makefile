dev:
	cargo watch \
		--exec "check --features=ssr" \
		--exec "check --features=hydrate" \
		--exec "test --features=ssr" \
		--clear

dev-server:
	killall warehouse --wait || true
	cargo leptos watch

lint:
	cargo clippy -- -D warnings

fmt:
	cargo fmt -- --check
	leptosfmt --check ./**/*.rs

dep:
	cargo deny check

dev-db:
	docker run \
		-p 5432:5432 \
		-e POSTGRES_PASSWORD=mysecretpassword \
		-e POSTGRES_DB=warehouse \
		--restart=unless-stopped \
		--volume /opt/warehouse/dev/pg-data:/var/lib/postgresql \
		-d \
		postgres:18

migration:
	diesel migration generate --diff-schema $(MIGRATION_NAME)

apply-migrations:
	diesel migration run

test:
	TEST_LOG=true cargo test $(TEST_NAME) | bunyan
