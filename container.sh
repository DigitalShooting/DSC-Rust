docker run --rm -it --name dsc-postgres -e POSTGRES_PASSWORD=pass -e POSTGRES_USER=user -p 5432:5432/tcp postgres

docker run --rm -it -p 8080:8080/tcp adminer
