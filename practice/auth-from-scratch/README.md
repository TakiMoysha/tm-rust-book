## 

- Register client for testing - https://oauth.com/playground/client-registrion.html
- Write it in .env file like:
```init
OAUTH_CLIENT_ID=<YOUR_CLIENT_ID>
OAUTH_CLIENT_SECRET=<YOUR_CLIENT_SECRET>
OAUTH_LOGIN=<YOUR_LOGIN>
OAUTH_PASSWORD=<YOUR_PASSWORD>
```
- Run with `just` (just load the .env file) or copy command from justfile and run manually with appropriate `env` variables.


## 

### Session-based token authentication

Session have an id and secret. 
