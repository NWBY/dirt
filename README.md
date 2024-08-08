# dirt

dirt (Deploy in Rust Time) is a deployment tool for deploying Laravel apps to production via SSH. dirt can setup your server with all the required
dependencies and then deploy to that server with zero downtime.

### Features

* Automatic installation of server dependencies (PHP-FPM, Caddy, etc.)
* Custom deployment script
* Zero downtime deployments
* Automatic SSL with Caddy
* Daemons

### Installation

Linux & MacOS:
```sh
curl https://dirt.samnewby.dev/install.sh | sh
```

You can confirm dirt is installed correctly by running: `dirt`

### Getting Started

1. In your Laravel app repo run: `dirt init` to create your `dirt.yaml` config file. dirt supports `yaml`, `json`, and `toml` config files as well.
2. Update your `dirt.yaml` to add your server IP address, path to your SSH key.
