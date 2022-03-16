f=-d

up:
	docker-compose up ${f} --build --force-recreate --remove-orphans

up:
	docker-compose down --remove-orphans