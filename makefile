init-db:
	@psql omdb -f ./src/cores/database/data/ddl.sql
	@psql omdb -f ./src/cores/database/data/dml.sql