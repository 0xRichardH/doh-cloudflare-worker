# DoH - Deploy to Cloudflare Workers

[![Deploy to Cloudflare Workers](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/cloudflare/templates/tree/main/worker-rust)

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

## Setup

- prequirements

```bash
brew install bun
brew install just
gem install envify
```

- setup `.env ` (see `.env.example`)
  > optional

```bash
envify g
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
