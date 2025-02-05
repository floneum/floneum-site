# floneum-site

Serve locally with:
```sh
npx tailwindcss -i ./input.css -o ./public/output.css
dx serve --fullstack
```

Prepare an official build with:
```sh
npx tailwindcss -i ./input.css -o ./public/output.css
rm -rf docs
dx bundle --fullstack --ssg --features prebuild 
cp -r target/dx/floneum-site/release/web/public docs
target/dx/floneum-site/release/web/server --search
cp docs/index.html docs/404.html
```
