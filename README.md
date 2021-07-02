# pastebin

Yet another pastebin app.
Currently in a very early development stage.

## Current status / features

- Simple deployment (single, ca. 3.3 MB executable);
- Serve traffic using an in-memory hash map;
- Easy data removal (just restart the app!);
- Increases your server Cool-Factor (since it's written in Rust).

## Usage

```sh
# To upload some UTF-8 text, POST to `/`:
$ curl http://localhost:2137/ --data 'Lorem ipsum dolor sit amet'
2ba728dc-6a7c-49fe-97f5-34b52210a5c4

# To download, GET `/<UUID>`:
$ curl http://localhost:2137/2ba728dc-6a7c-49fe-97f5-34b52210a5c4
Lorem ipsum dolor sit amet
```

