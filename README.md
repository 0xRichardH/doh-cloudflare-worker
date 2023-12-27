# DoH - Deploy to Cloudflare Workers

## DNS over HTTPS

- Using DNS Wireformat: https://developers.cloudflare.com/1.1.1.1/encryption/dns-over-https/make-api-requests/dns-wireformat/

  - GET

  ```shell
  curl -H 'accept: application/dns-message' -v 'https://cloudflare-dns.com/dns-query?dns=q80BAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB' | hexdump
  ```

  - POST

  ```shell
  echo -n 'q80BAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB' | base64 --decode | curl -H 'content-type: application/dns-message' --data-binary @- https://cloudflare-dns.com/dns-query -o - | hexdump
  ```

- using JSON: https://developers.cloudflare.com/1.1.1.1/encryption/dns-over-https/make-api-requests/dns-json/

  - GET

  ```shell
  curl -H "accept: application/dns-json" "https://cloudflare-dns.com/dns-query?name=example.com&type=AAAA"
  ```

[![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/cloudflare/templates/tree/main/worker-rust)

A template for kick starting a Cloudflare worker project using [`workers-rs`](https://github.com/cloudflare/workers-rs).

This template is designed for compiling Rust to WebAssembly and publishing the resulting worker to Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/).

## Setup

- install bun and just

```bash
brew install bun
brew install just
```

- install wrangler

```bash
bun install
```

- Test in local and Deploy to Cloudflare

```bash
just dev

just test

just deploy
```

Read the latest `worker` crate documentation here: https://docs.rs/worker
