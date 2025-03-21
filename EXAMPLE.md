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
cargo run human-user add --file-path examples/add_user.json
```

## Add an organization (Service users can't create orgs so it won't work if you logged in using PAT/client credentials)

```sh
cargo run org add  --file-path examples/add_organization.json
```

## Add a project

```sh
cargo run project add --file-path examples/add_project.json
```

## Add an identity provider

### Google

```sh
cargo run idp add google --file-path examples/add_google_idp.json
```
