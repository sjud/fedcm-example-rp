### Fedcm Example Idp/Rp

This is a leptos full stack web app that uses the experimental BYOIDP FedCM feature of Chromium Canary
https://github.com/fedidcg/FedCM/issues/240#issuecomment-2004650817


This example only works in chromium canary
https://www.google.com/chrome/canary/

this example runs three leptos servers on 127.0.0.1:3000, 127.0.0.2:3001, 127.0.0.3:3002

<<<<<<< HEAD
having rust, and cargo already installed install cargo leptos
`sh
cargo install cargo-leptos
`
navigate to each folder (rp,idp_2,idp_1) and run
`sh
=======
```sh
rustup target add wasm32-unknown-unknown
>>>>>>> 287f2786bc33afc71ecc54939f8d809c818db96c
cargo leptos serve
`

if on unix you might need to add 127.0.0.2 and 127.0.0.3 
i.e
`sudo ifconfig lo0 alias 127.0.0.3 up`
`sudo ifconfig lo0 alias 127.0.0.2 up`

The rp is served at `http://127.0.0.1:3000/` 
