# Zitadel Resources CLI

This is a command-line interface (CLI) tool designed to interact with the Zitadel API, allowing users to manage resources via the CLI.
If you receive an unauthorized response, it is recommended to log in again to refresh your session and regain access.

The CLI doesn't support `x-zitadel-orgid` yet, the organization targeted is the default one.
The required fields and their types are validated, but the actual values haven't been verified yet. For instance, we check if the 'gender' field is a string, but we don't validate whether it's one of the supported values (e.g., GENDER_UNSPECIFIED, GENDER_FEMALE, GENDER_MALE, GENDER_DIVERSE). Please be cautious when providing payloads.

Right now you can authenticate using different flows:

- [Client credentials](#client-credentials-flow-for-service-users)
- [Authorization](#authorization-flow)
- [Personal access token](#personal-access-token-pat-for-service-users)

## Installation

To get started, follow these steps:

1. **Install Rust and Cargo**  
   If you haven't already, install Rust and Cargo from [here](https://rustup.rs/).

2. **Set up the environment variables**  
   Define the [environment variables](#environment-variables) (possibly in a `.env` file) to configure your environment.

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
  Currently calls [this](https://zitadel.com/docs/apis/resources/org_service_v2/organization-service-add-organization) endpoint

A minimal payload that works can be found in `add_organization.json`.

- **`add-human-user`**  
  Adds a new human user. This command checks if the CLI is logged in, then interacts with the Zitadel API.
  Currently calls [this](https://zitadel.com/docs/apis/resources/user_service_v2/user-service-add-human-user) endpoint

  **Options:**

  - `--file-path <path>` - Specifies the path to the JSON file containing the new organization's details.

A minimal payload that works can be found in `add_user.json`.

- **`add-project`**  
  Adds a new project. This command checks if the CLI is logged in, then interacts with the Zitadel API.
  Currently calls [this](https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project) endpoint

  **Options:**

  - `--file-path <path>` - Specifies the path to the JSON file containing the new organization's details.

A minimal payload that works can be found in `add_project.json`.

- **`help`**  
  Displays all available commands and options.

## Environment Variables

### Client credentials flow for service users

- `CONFIG_FILE_PATH` - The path where the login informations are saved

- `ISSUER` - The URL of the Zitadel instance.

- `CLIENT_ID` - Your client ID for authentication

- `CLIENT_SECRET` - Your client secret

- `SCOPES` - The scopes required. `urn:zitadel:iam:org:project:id:zitadel:aud` is necessary for managing resources. See [Scopes Documentation](https://zitadel.com/docs/apis/openidoauth/scopes) for more information.

### Authorization flow

Ensure you set up the following environment variables:

- `CONFIG_FILE_PATH` - The path where the login informations are saved

- `ISSUER` - The URL of the Zitadel instance.

- `CLIENT_ID` - Your client ID for authentication.

- `CALLBACK_SERVER_ADDRESS` - The URL of the callback server that Zitadel calls once the user is authenticated (Usually called redirect_uri).

- `SCOPES` - The scopes required. `urn:zitadel:iam:org:project:id:zitadel:aud` is necessary for managing resources. See [Scopes Documentation](https://zitadel.com/docs/apis/openidoauth/scopes) for more information.

### Personal Access Token PAT for service users

- `CONFIG_FILE_PATH` - The path where the login informations are saved

- `ISSUER` - The URL of the Zitadel instance.

- `PERSONAL_ACCESS_TOKEN` - The personal access token as described [here](https://zitadel.com/blog/new-personal-access-token#how-to-set-up-pats-on-the-zitadel-console)

- `SCOPES` - The scopes required. `urn:zitadel:iam:org:project:id:zitadel:aud` is necessary for managing resources. See [Scopes Documentation](https://zitadel.com/docs/apis/openidoauth/scopes) for more information.

## To Do

- [x] Authentication of service accounts via CLI (also PAT ok)
- [x] Create Organizations
- [x] Create Projects
- [ ] Create Applications (5 client types)
- [x] Create users (human)
- [ ] Add identity provider (all templates)

- [ ] Add more unit tests

### Bonus:

- [ ] Enforce more strict type validations (e.g., < 200 characters/fields with fixed values)
- [ ] Add option to interact with the grpc APIs

### Questions:

- Where is the projects endpoint located? I could only find the endpoint for the V1

- The payload displayed at for creating a human user doesn't seem to work out of the box (https://zitadel.com/docs/apis/resources/user_service_v2/user-service-add-human-user):

```
2025-03-06T21:20:35.912734Z ERROR zitadel_cli: Failed to create human user: Invalid response: An unexpected error occured: {"code":3,"message":"proto: (line 1:109): error parsing \"orgDomain\", oneof zitadel.object.v2.Organization.org is already set"}
```

- The payload displayed at for creating an organization doesn't seem to work out of the box (https://zitadel.com/docs/apis/resources/org_service_v2/organization-service-add-organization):

```
2025-03-06T21:18:14.377730Z ERROR zitadel_cli: Failed to create organization: Invalid response: An unexpected error occured: {"code":3,"message":"proto: (line 1:48): error parsing \"human\", oneof zitadel.org.v2.AddOrganizationRequest.Admin.user_type is already set"}
```

- The create project API endpoint returns a status code 200, is it done on purpose? The V1: https://zitadel.com/docs/apis/resources/mgmt/management-service-add-project
