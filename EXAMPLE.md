## Log in

```sh
cargo run login

cargo run login --flow client-credentials

cargo run login --flow authorization-code --open

cargo run login --flow personal-access-token
```

## Log out

```sh
cargo run logout
```

## Add a human user

```sh
cargo run add-human-user --file-path add_user.json
```

## Add an organization (Service users can't create orgs so it won't work if you logged in using PAT/client credentials)

```sh
cargo run add-org --file-path add_organization.json
```
