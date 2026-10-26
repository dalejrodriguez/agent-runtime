#!/usr/bin/env python3
import signal
import sys
import urllib.request


def main() -> int:
    match sys.argv[1:]:
        case ["idle"]:
            while True:
                signal.pause()
        case ["get", repo] if valid(repo):
            try:
                body = get(repo)
            except Exception as e:
                print(f"status: {e}", file=sys.stderr)
                return 1
            sys.stdout.write(body)
            return 0
        case _:
            print("usage: get <repo>", file=sys.stderr)
            return 2


def valid(repo: str) -> bool:
    return repo.isascii() and repo.isalnum()


def get(repo: str) -> str:
    with open("/run/secrets/token") as f:
        token = f.read()
    url = f"https://raw.githubusercontent.com/edera-dev/{repo}/main/README.md"
    token = f"Bearer {token.strip()}"

    req = urllib.request.Request(url, headers={"Authorization": token})
    with urllib.request.urlopen(req) as resp:
        return resp.read().decode()


if __name__ == "__main__":
    sys.exit(main())
