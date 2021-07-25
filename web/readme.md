# Rust url shortener

This app is a simple url shortening service, written in rust, it has frontend built using [yew](http://yew.rs) and backend using [rocket](http://rocket.rs), with postgresql database.

## Installation

Clone this repository, rename .env.example of the dockerfile to .env then to build and start app run

```bash
docker compose up -d
```

It might take a while for the app to build

## Usage

Frontend should be accessible at [localhost:8080](http://localhost:8080), and backend on [localhost:8000](http://localhost:8000).

To test API you can use localhost:8000/swagger-ui, each controller is in its own definition because of the way swagger is implemented in the app.

On [authenticate](http://localhost:8000/account/authentication) route, you can authenticate with username admin and password of your choice which will then be set as your admin password, repeat the request to authenticate and get JWToken.

## License
This project doesn't currently have a license
