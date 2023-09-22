# Contactbook API in Rust

## Setup

1. Install [Docker](https://docker.com) and docker-compose
2. create `.env` file with:

    ``` env
    DATABASE_URL=postgres://postgres:postgrespw@localhost:5432
    POSTGRES_PASSWORD=postgrespw
    ```

3. Build Dockerimage by running

    ```bash
    docker build -t rust-api .
    ```

## Run locally using docker

1. Run following command in your terminal

    ```bash
    docker compose up 
    or
    docker-compose up 
    ```

2. Open up your broser or API tester like [Postman](https://postman.com) and head to `localhost:3000`
