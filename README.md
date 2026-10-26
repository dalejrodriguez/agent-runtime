# agent-runtime
This is a short lab to interact with Edera's Agent Runtime environment. <br/>
Understand all of the features announced as part of **[Project Lunchbox](https://edera.dev/stories/project-lunchbox-freeze-it-fork-it-let-it-run)**.

## Demo Link
https://ndouglas-edera.github.io/agent-runtime/

## Demo Workflow
Same readme fetcher in ```Rust```, ```Python``` and ```TypeScript```. <br/>
The container idles until you exec `/readme-fetch get <repo>` into it, then it prints the README for `edera-dev/<repo>`

### Token
Needs a GitHub token with read access to the ```edera-dev``` repos, mounted at `/run/secrets/token`

### Build

Rust

```sh
cd rust && cargo build --release
```

TypeScript

```sh
cd typescript && npm ci && npm run build
```

Python doesn't need a build step

### Docker

```sh
docker build -t ttl.sh/celery/readme-fetch-rust:1h rust
docker build -t ttl.sh/celery/readme-fetch-python:1h python
docker build -t ttl.sh/celery/readme-fetch-typescript:1h typescript
```

### Launch

Launch the zone
```
sudo protect zone launch -n cap-provider
```

Launch the workload

**NOTE**: You need to create a valid github token in `/tmp/token`

```
sudo protect workload launch -n readme -z cap-provider \
  --mounts src=/tmp/token:dst=/run/secrets/token:ro=true \
  --ocap-publish 'dev.example/readme=/readme-fetch get' \
  ttl.sh/celery/readme-fetch-{language}:1h
```

Dialing the capability via protect client
```
sudo protect object-capability dial dev.example/readme protect
```
