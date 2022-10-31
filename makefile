init-db:
	@psql omdb -f ./src/cores/database/data/ddl.sql
	@psql omdb -f ./src/cores/database/data/dml.sql

coverage:
	@bash coverage.sh

coverage-ci:
	@bash coverage.sh ci

install:
	@cp .env.sample .env
	@code .