# yew-app

## setup
```shell
$ cargo new --lib yew-app && cd yew-app
$ cargo install wasm-pack
```

## build
```shell
$ wasm-pack build --target web --out-name wasm --out-dir ./static
```

## hosting
```shell
$ miniserve ./static --index index.html
```


## tips
- [nightlyを使うためのプロジェクト設定](https://teratail.com/questions/326468
)
