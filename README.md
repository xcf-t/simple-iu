# simple-iu.rs

> A very minimalistic uploader for all kinds of files

### Configuration 

> All Uploader related configuration can be done in the `Settings.toml` file

```toml
[settings]
# Output directory
output = "Files"

# A minimum id in base36
min_id = "aaaaa"

# A maximum id in base36
# This has to be larger than min_id
max_id = "zzzzzz"

# Global upload limit
upload_limit = "256 MiB"

# You can give this user any name you want
[user.user1]
# Put this in the Authorization header
# For increased security the token should be random and as long as possible
token = "NLhv97y0b+CvQrur5wyIG2SUYjVzKBg3EFbE2xup52EWSw0JnmBkn6ENWP5fOL"

# Every user can have a custom descriptor
# Replaces {name} with the generated id and {ext} with the file extension
# Default: "{name}.{ext}"
descriptor = "user1-{name}.{ext}"

# Overrides the global upload limit for this specific user
# Optional
upload_limit = "20 MiB"

[User.test]
token = "gJHJ5scsmBweSD7Um/kWNmPKFzRc4jtJJ+QO3tR3NEQptqlJPEcMIYAj3FYBJKKhNHGvnPex"
```

- The output directory will be served statically on route `/`
    - If the output directory is `Files`, the file `Files/LKEVQJ.png` would be available under `/LKEVQJ.png`
- The token can be any random combination of characters
- The filename will be the descriptor
    - `{name}` will be replaced with a random Base36-Id in the range of min_id and max_id
    - `{ext}` will be replaced with the file extension specified in the url path

Accepted grammar for the UploadLimits is:

```
byte_unit := uint+ ('.' uint+)? WHITESPACE* suffix

uint := '0'..'9'
suffix := case insensitive SI byte unit suffix ('b' to 'eib')
WHITESPACE := the ' ' character
```

### Uploading

- Request type: Post
- Authorization-Header
- File as binary data body

```bash
curl -X POST -H "Authorization: TOKEN" --data-binary '@path/to/file.png' http://HOSTNAME:8000/upload/png
```

## Configuring the Webserver

This application is based on [Rocket](https://rocket.rs) and takes the default [Rocket configuration format](https://rocket.rs/master/guide/configuration/).
An example is included in this repo.