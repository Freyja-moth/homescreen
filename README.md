# Royal homescreen

> Originally created by [tressley](https://github.com/tressley/_traichu)
> Modifications by me

## Compiling

### Code

> You will need to install dixous_cli in order to compile the frontend 

Compiling happens in three parts

- The backend server 
- The frontend html and wasm
- The frontend server

```bash
# Navigate to the backend...
cd crates/homescreen_backend
# ...and compile it normally
cargo build --release

# Navigate to the frontend server...
cd crates/homescreen_frontend/homescreen_server
# ...and compile it normally
cargo build --release 

# Navigate to the frontend webpage... 
cd crates/homescreen_frontend/homescreen_webpage
# ...and compile it using dioxus cli
dx build --release 
```

### Organising

After compiling you will need to copy the homescreen_server and homescreen_backend executables from your target directory (make sure you look in the root of the project, it will not appear in the homescreen_sever/homescreen_backend) to where ever it is you'd prefer to have them. 

Then you need to copy the dist folder from [the server crate](/crates/homescreen_frontend/homescreen_server/) (it's compiled to there to make it easy to make changes while working on the frontend) into the directory where you have the homescreen_server executable.

Once that's done create Config.toml in the directory where you have the homescreen_backend executable, it should look something like this

```toml
# This should be a MySql/MariaDB url, I am using sqlx so if you're struggling to get it to work check out their syntax
database_url=<your-database-url>
port=<server-port>
```

The directory should now look something like this

```
.
├── Config.toml
├── dist
│  └── ...
├── homescreen_backend
└── homescreen_server
```

### Database schema

There is only one table needed for the database

```
CREATE TABLE websites(website_name VARCHAR(255) PRIMARY KEY, website_link VARCHAR(255) UNIQUE NOT NULL, section ENUM('code', 'fun', 'editing') NOT NULL);
```

- website_name

The name of the website that you want to be displayed, it does not have to be accurate.

- website_link

The link used for the website, it should not include http:// or https:// as that would mess with the way the homepage loads the icons for sites (there is a check for this in the backend though so don't worry about messing it up on accident).

- section

The section that you want the website to be included in, currently hardedcoded though that may change if needed.


## Creating/deleting websites

Creating and deleting websites should be done through the backend. Currently there is no authentication so it is not recommended to run this anywhere that someone could send a request to it aside from you.

### Creating 

```
PUT /websites HTTP/1.1
HOST: <your-backend-url>
CONTENT-TYPE: x-www-form-urlencoded
```

The form should look like this

| website_name | website_link | section |
| ------------ | ------------ | ------- |
| name         | link         | section |

### Deleting

```
PUT /websites/{website_name} HTTP/1.1
HOST: <your-backend-url>
```

Where website_name is the name of the website you want to delete

# TODO's
- Figure out favicon
- Add authentication
- Do proper logging in browser console
- Unhardcode the sections (possibly)
- Create a website moniter for the frontend (possibly)
