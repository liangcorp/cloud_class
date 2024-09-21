#!/bin/sh

# setup directories
mkdir student_codes

# install docker
sudo dnf install docker

# setup redis container
docker pull redis
docker create redis

sudo mkdir -p  /var/redis/data
sudo mkdir $PWD/redis-data
sudo chmod 775 -R /var/redis/data
sudo chmod 775 -R $PWD/redis-data

docker run -d   --name redis_server   -v $PWD/redis-data:/var/redis/data    -p 6379:6379   redis --requirepass cikq5XxudvHKUzdPgbQWokCOOhfT8wGQKPsLhBx8Tlw=

# setup streamlit docker for sample student -- student1
cd docker
docker build -t streamlit .
docker run -p 8501:8501 -d --name student1 -it streamlit

