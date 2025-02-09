# floneum-site

Serve locally with:
```sh
npx tailwindcss -i ./input.css -o ./public/output.css
dx serve --fullstack
```

Prepare an official build with:
```sh
npx tailwindcss -i ./input.css -o ./public/output.css
rm -rf docs && rm -rf target/dx
dx bundle --fullstack --ssg --features prebuild --release
cp -r target/dx/floneum-site/release/web/public docs
target/dx/floneum-site/release/web/server --search
cp docs/index.html docs/404.html
```
