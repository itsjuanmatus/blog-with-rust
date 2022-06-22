# Blog with rust

This is the first project I've built with Rust.

The goal of this project was to learn about templates and rust web servers - pages were not supposed to look fancy.

This should be live here: https://blog-with-rust.herokuapp.com/


### How to run the project:

Make sure you read the `.env.example` and define your own `.env` variables to connect to your database.

#### Docker commands:

`docker build -t web:latest . `

`docker run -d --name blog-with-rust -e "PORT=8765" -e "DEBUG=0" -p 8007:8765 web:latest`


#### Routes available

- GET: `/`
- GET: `/blog/${slug-name}` (ex. `/blog/second-from-postman`)
- POST: `/new-post`
