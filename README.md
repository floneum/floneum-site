# floneum-site

Serve locally with:
```sh
npx tailwindcss -i ./input.css -o ./public/output.css --watch
dx build --features web
cargo run --features ssr
```

Prepare an official build with:
```sh
npx tailwindcss -i ./input.css -o ./public/output.css --watch
dx build --features web --release --skip-assets
cargo run --features prebuild --release
```
