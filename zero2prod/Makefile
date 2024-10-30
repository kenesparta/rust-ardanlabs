install-sqlx-cli:
	cargo install --version="~0.8" sqlx-cli \
      --no-default-features \
      --features rustls,postgres

up-db:
	SKIP_DOCKER=true ./scripts/init_db.sh

create-migration:
	export DATABASE_URL=postgres://app:secret@127.0.0.1:5432/newsletter
	sqlx migrate add create_subscriptions_table

migrate:
	export DATABASE_URL=postgres://app:secret@127.0.0.1:5432/newsletter
	sqlx migrate run

sqlx-prepare:
	cargo sqlx prepare --database-url postgres://app:secret@localhost:5432/newsletter