# floneum-site

Serve locally with:
```sh
npx tailwindcss -i ./input.css -o ./output.css --watch
trunk build --features web
cargo run --features ssr
```

Prepare an official build with:
```sh
npx tailwindcss -i ./input.css -o ./output.css --watch
trunk build --features web
cargo run --features prebuild
```
