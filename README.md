# Sol

Sol is an application that extracts information from Zuul via the REST API.

## Features

Following API endpoints are supported:

- [GET /api/tenant/{tenant_name}/buildsets](https://zuul-ci.org/docs/zuul/latest/rest-api.html#get--api-tenant-tenant_name-buildsets)
- [GET /api/tenant/{tenant_name}/autohold](https://zuul-ci.org/docs/zuul/latest/rest-api.html#get--api-tenant-tenant_name-autohold)
- [GET /api/tenant/{tenant_name}/builds](https://zuul-ci.org/docs/zuul/latest/rest-api.html#get--api-tenant-tenant_name-builds)

([Zuul's REST API](https://zuul-ci.org/docs/zuul/latest/rest-api.html))

When the REST API can't provide the needed information Sol `functions` are
created. Below are the current functions supported

- `sol function build-nodes`

### Functions

#### build-nodes

This function will inspect the inventory.yaml file from Zuul build and list the
nodes under the section `all.hosts`, along with their IPv4 address and label

## Output

Output can be formatted (default), or in JSON format using the global
`--output json` option to any Sol command.

## Configuration

By default Sol looks for the configuration file at
`$HOME/.config/sol/config.yml`, this can be overridden using the env variable
`SOL_CONFIG_PATH`.

If no configuration can be found Sol will create one in the default location.
This configuration will need to be updated.

## Debugging

Sol uses `env_logger` crate. So to run with debug set `RUST_LOG=debug`
before you run it.

For backtrace set `RUST_BACKTRACE=1`.
