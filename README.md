# Zitadel Resources CLI

This is a command-line interface (CLI) tool designed to interact with the Zitadel API, allowing users to manage resources via the CLI.
If you receive an unauthorized response, it is recommended to log in again to refresh your session and regain access.

The CLI doesn't support `x-zitadel-orgid` yet, the organization targeted is the default one.
No strict type validation on the data is done yet, so be careful with the payloads you provide.

Right now you can authenticate using different flows:

- [Client credentials](#client-credentials-flow)
- [Authorization](#authorization-flow)
- [Personal access token](#personal-access-token)

## Installation

To get started, follow these steps:

1. **Install Rust and Cargo**  
   If you haven't already, install Rust and Cargo from [here](https://rustup.rs/).

2. **Set up the environment variables**  
   Follow the instructions in the [environment variables](#environment-variables) section to configure your environment.

## Usage

Detailed usage examples are present in the `README.md` file.

To run the CLI:

```sh
cargo run <COMMAND>
```

The CLI currently supports the following commands:

- **`login`**  
  Logs the user in and writes the access token to a file. The default is flow is the `Authorization flow`

  **Options:**

  - `--open` - Opens the browser for authentication.

- **`logout`**  
  Logs the user out by removing the access token file.

- **`add-org`**  
  Adds a new organization. This command checks if the CLI is logged in, then interacts with the Zitadel API.

A minimal payload that works can be found in `add_organization.json`.

- **`add-human-user`**  
  Adds a new human user. This command checks if the CLI is logged in, then interacts with the Zitadel API.

  **Options:**

  - `--file-path <path>` - Specifies the path to the JSON file containing the new organization's details.

A minimal payload that works can be found in `add_user.json`.

- **`help`**  
  Displays all available commands and options.

## Environment Variables

### Client credentials flow (service users)

- `ISSUER` - The URL of the Zitadel instance.

- `CLIENT_ID` - Your client ID for authentication

- `CLIENT_SECRET` - Your client secret

- `SCOPES` - The scopes required. `urn:zitadel:iam:org:project:id:zitadel:aud` is necessary for managing resources. See [Scopes Documentation](https://zitadel.com/docs/apis/openidoauth/scopes) for more information.

### Authorization flow

Ensure you set up the following environment variables:

- `ISSUER` - The URL of the Zitadel instance.

- `CLIENT_ID` - Your client ID for authentication.

- `CALLBACK_SERVER_ADDRESS` - The URL of the callback server that Zitadel calls once the user is authenticated (Usually called redirect_uri).

- `SCOPES` - The scopes required. `urn:zitadel:iam:org:project:id:zitadel:aud` is necessary for managing resources. See [Scopes Documentation](https://zitadel.com/docs/apis/openidoauth/scopes) for more information.

### Personal Access Token PAT(service users)

- `ISSUER` - The URL of the Zitadel instance.

- `PERSONAL_ACCESS_TOKEN` - The personal access token as described [here](https://zitadel.com/blog/new-personal-access-token#how-to-set-up-pats-on-the-zitadel-console)

- `SCOPES` - The scopes required. `urn:zitadel:iam:org:project:id:zitadel:aud` is necessary for managing resources. See [Scopes Documentation](https://zitadel.com/docs/apis/openidoauth/scopes) for more information.

## To Do

- [x] Authentication of service accounts via CLI (also PAT ok)
- [x] Create Organizations
- [ ] Create Projects
- [ ] Create Applications (5 client types)
- [x] Create users (human)
- [ ] Add identity provider (all templates)

### Bonus:

- [ ] Enforce more strict type validations (e.g., < 200 characters/fields with fixed values)
