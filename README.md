# 🚀 Shuttle

Shuttle is a bare-bones user authentication system.

## Installation

`just`, `bun`, and (obviously) `rust` are needed to install Shuttle.

```bash
git clone https://github.com/swmff/shuttle
cd shuttle
bun i
just build
```

You can also supply your desired database type after `just build`. You can use one of the following:

* `mysql`
* `postgres`
* `sqlite`
