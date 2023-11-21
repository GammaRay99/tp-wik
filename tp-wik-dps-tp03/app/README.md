# Description

A simple API poc in rust using the stdlib, a single route is accessible: `/ping` and returns the headers sent in a JSON format.

You can change the default port by creating an environnement variable named `PING_LISTEN_PORT`


# Installation 

```
$ git clone https://github.com/GammaRay99/tp-wik.git`
```

# Usage

```
$ cd tp-wik-dps-tp01
$ cargo run
```

In another terminal:
``` 
$ curl http://127.0.0.1:8080
``` 