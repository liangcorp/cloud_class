#!/bin/sh

# setup directories
mkdir student_codes

# setup redis container
podman pull redis
podman create redis

sudo mkdir -p  /var/redis/data
sudo mkdir $PWD/redis-data
sudo chmod 775 -R /var/redis/data
sudo chmod 775 -R $PWD/redis-data

podman run -d   --name redis_server   -v $PWD/redis-data:/var/redis/data    -p 6379:6379   redis --requirepass cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=
