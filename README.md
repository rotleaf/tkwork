
# TKWORK-

Supposed to be an autopilot script for tkwork.org, does likes and comments. Doesn't touch tiktok so beware.


## Environment Variables

To run this script, you will need to add the following environment variables to your .env file

`USER_AGENT` - user agent, find one online or copy from devtools of browser

`COOKIE` - request header cookie, can be obtained via devtools in your browser

`USER_TOKEN` - user token used to post in each request, check devtools in your browser


## Running

To run this script

```bash
  # run in debug mode
  cargo run -q
```

```bash
  # build a release binary
  cargo build -r
```

or you could just download an executable from the releases section.

## Disclaimer

This script hasn't been tested for long so no one is responsible for you losing your account. Use with caution
