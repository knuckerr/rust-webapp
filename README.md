# rust-webapp
This a basic rest-api build with Rust . This project for me is aiming to understand and learn Rust 
and gain the experience using Rust as a back end Server

# Running the DB
This app is using postgres 10 version  

to create the DB server enter 
```bash 
docker-compose up 
```

to create the tables for the database
```bash 
sh create_tables.sh
```


Adminer is on http://localhost:8000/      `username:root,pass:root`

# For the Rust server 
To run the application execute:

```bash
cargo run
```

Then to view it in your browser navigate to: [http://localhost:8088/clients](http://localhost:8088/clients)
