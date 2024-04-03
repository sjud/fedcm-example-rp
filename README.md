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
cargo leptos serve
```

And go to `http://127.0.0.1:3000/`