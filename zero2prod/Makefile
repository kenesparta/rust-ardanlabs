install-sqlx-cli:
	cargo install --version="~0.8" sqlx-cli \
      --no-default-features \
      --features rustls,postgres

up-db:
	./scripts/init_db.sh

migrate:
	export DATABASE_URL=postgres://app:secret@127.0.0.1:5432/newsletter
	sqlx migrate add create_subscriptions_table
