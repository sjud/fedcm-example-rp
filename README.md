### Fedcm Example Idp/Rp

This is a leptos full stack web app that uses the experimental FedCM feature of Chromium Canary
https://github.com/fedidcg/FedCM/issues/240#issuecomment-2004650817
to authenticate against it's own IDP per
https://fedidcg.github.io/FedCM/#idp-api-config-file

This example only works in chromium canary
https://www.google.com/chrome/canary/

but otherwise
cast

```sh
rustup target add wasm32-unknown-unknown
cargo leptos serve
```

if on unix you might need to add 127.0.0.2 and 127.0.0.3 
i.e
`sudo ifconfig lo0 alias 127.0.0.3 up`
`sudo ifconfig lo0 alias 127.0.0.2 up`

And go to `http://127.0.0.1:3000/` for rp 
