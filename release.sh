#!/usr/bin/env sh

docker build -t registry.gitlab.com/addapp/gymtracker . && docker push registry.gitlab.com/addapp/gymtracker

if [[ "$1" == "--deploy" ]]; then
	echo "deploying"
	ssh netcup "cd dockerfiles/gymtracker && docker-compose down && docker-compose pull web && docker-compose up -d"
fi
