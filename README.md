# Responder

Responder is a feather-light HTTP server for static sites that can be ran using
Docker + WASM. Motivation for the project is to have websites run as fast as
possible with as little resources as possible. To achieve that, it requires
that responses are precomputed.

# Packager

Responder Packager is a command line utility to create packages of matchers and responses
that Responder server can use. It requires a manifest similar to
[the example](example/manifest.yaml).

## Usage

```
$ responder-packager <MANIFEST> <OUTPUT>
```

Where arguments are:

- `<MANIFEST>` - Path to Responder manifest
- `<OUTPUT>` - Path to output file (for example package.bin)

# Server

Responder Server is a binary that binds to port 8080 to listen for connections
and responds according to the package.

## Usage

```
$ responder-server <PACKAGE>
```

Where `<PACKAGE>` is path to package generated by Responder Packager.
