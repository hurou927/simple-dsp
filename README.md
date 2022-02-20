# Simple Dsp

## Options

```sh
OPTIONS:
    -c, --conf-path <CONF_PATH>    [default: ./config.yml]
    -h, --help                     Print help information
    -p, --port <PORT>              [default: 3000]
    -V, --version                  Print version information
```

## Macro


| MACRO          | DESCRIPTION |
|----------------|-------------|
| `$[XX_IMP_ID]` | imp_id      |


## Config

Use yaml

```yaml

resources: #array
  - path: "/path/to/response/content" # type: string. desc: resource path

    uri: "/http/path" # type: string, desc: http path
    cond: 1 # type: int, desc: imp_condition

```

### ImpCondition 

```rust
pub enum ImpCondition {
    NativeVideo = 1,
    NativeImage = 2,
    Video = 3,
    ImpFirst = 11,
    ImpSecond = 12,
    ImpThird = 13,
}
```


### Sample

```sh
$ make run

$ curl -sv -d @./resources/req.json localhost:3000/r/video
$ curl -sv -d @./resources/req.json localhost:3000/r/nvideo
$ curl -sv -d @./resources/req.json localhost:3000/r/nimage
$ # Support GZIP
$ cat ./resources/req.json | gzip | curl -sv --data-binary @- -H "Content-Encoding: gzip" localhost:3000/r/nimage
```

