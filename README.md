# Scarabee

## Installation

- Install rustup (https://rustup.rs/). This is used to install and update cargo, which is used to compile and run Rust programs.
- Install diesel_cli (https://diesel.rs/guides/getting-started.html). This is used to manage the database for the app. If you have installed cargo, you can run `cargo install diesel_cli`, which will install this tool. See the guide if you need to know more.
- Install MySQL (https://www.mysql.com/) and MySQL Workbench if you want to use a GUI. This is the database software.

### Running the project

First, create a user for MySQL and the database the app will use. You can do this via a GUI or the command line for MySQL (MariaDB should also work). 

After creating the user, update the .env file and Rocket.toml to include the user and password, in the format: user:password (currently this is test:test, you can replace this).

Next, create the database, also via MySQL, with the following command.

```
CREATE DATABASE 3ways;
```

Open your terminal and navigate to the project folder. Run migrations on the database to fill it with tables.

```
diesel migration run
```

Almost done. Finally, run the project in your terminal from the same folder.

```
cargo run
```

You should see something like: "Rocket has launched from: http://127.0.0.1:8000". You can visit the from that address in your browser and start using the app.
