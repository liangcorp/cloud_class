#!/bin/sh

# setup directories
mkdir student_codes

# install podman
sudo dnf install podman

# setup redis container
podman pull redis
podman create redis

sudo mkdir -p  /var/redis/data
sudo mkdir $PWD/redis-data
sudo chmod 775 -R /var/redis/data
sudo chmod 775 -R $PWD/redis-data

podman run -d   --name redis_server   -v $PWD/redis-data:/var/redis/data    -p 6379:6379   redis --requirepass cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=

# setup streamlit docker for sample student -- student1
cd docker
podman build -t streamlit .
podman run -p 8501:8501 -d --name student1 -it streamlit

