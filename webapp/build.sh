
cd client && wasm-pack build --target web --out-dir static && cd ..

cp client/style.css client/static/
cp client/index.html client/static/
