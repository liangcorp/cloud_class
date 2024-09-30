#!/bin/sh

# install podman
sudo dnf install podman podman-docker

sudo mkdir -p  /var/redis/data
mkdir -p $HOME/containers/redis-data

sudo chmod 775 -R /var/redis/data
chmod 775 -R $HOME/containers/redis-data

# setup redis container
docker pull dockerproxy.cn/redis
docker create redis

docker run -d --name redis_server -v $HOME/containers/redis-data:/var/redis/data -p 6379:6379 redis --requirepass cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=

# setup streamlit docker for students
cd containers/docker
docker build -t streamlit .
docker run -p 8501:8501 -d --name $1 -it streamlit

