#!/usr/bin/env node
import { readFileSync } from "node:fs";

async function main(args: string[]): Promise<number> {
  if (args.length === 1 && args[0] === "idle") {
    setInterval(() => {}, 2 ** 31 - 1);
    return new Promise(() => {});
  }
  if (args.length === 2 && args[0] === "get" && valid(args[1])) {
    try {
      process.stdout.write(await get(args[1]));
      return 0;
    } catch (e) {
      console.error(`status: ${e instanceof Error ? e.message : e}`);
      return 1;
    }
  }
  console.error("usage: get <repo>");
  return 2;
}

function valid(repo: string): boolean {
  return /^[A-Za-z0-9]+$/.test(repo);
}

async function get(repo: string): Promise<string> {
  const secret = readFileSync("/run/secrets/token", "utf8");
  const url = `https://raw.githubusercontent.com/edera-dev/${repo}/main/README.md`;
  const token = `Bearer ${secret.trim()}`;

  const res = await fetch(url, { headers: { Authorization: token } });
  if (!res.ok) {
    throw new Error(`HTTP status ${res.status} ${res.statusText} for url (${url})`);
  }
  return res.text();
}

main(process.argv.slice(2)).then((code) => {
  process.exitCode = code;
});
