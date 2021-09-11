# yew-kanban-app

[Rust + Yew = WebAssembly でかんばんライクなタスク管理アプリを作ってみました。](https://nulab.com/ja/blog/nulab/rust-yew-webassembly-kanban-app/)
を読んで同じもの作って見ようとしたら記事が崩れて読めなかったので、サンプルとか色々みて補完した。

## setup
```shell
$ cargo new --lib yew-app && cd yew-app
$ cargo install wasm-pack
```

## build
```shell
$ wasm-pack build --target web --out-name wasm --out-dir ./static
```

## local serve
```shell
$ miniserve ./static --index index.html
```


## tips
- [nightlyを使うためのプロジェクト設定](https://teratail.com/questions/326468
)
