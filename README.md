# ネイピア数算出プログラム

Rustで任意精度浮動小数点数型を扱うrugクレートを使用して、ネイピア数を算出します。  

> 2.7182818284590452353602874713526624977572470936999595749669676277240766303535475945713821785251664274274663919320030599218174135966290435729003342952605956307381323286279434907632338298807531952510190115738341879307021540891499348841675092447614606680822648001684774118537423454424371075390777449920695517027618386062613313845830007520449338265602976067371132007093287091274437470472306969772093101416928368190255151086574637721112523897844250569536967707854499699679468644549059879316368892300987931277361782154249992295763514822082698951936680331825288693984964651058209392398294887933203625094431173012381970684161403970198376793206832823764648042953118023287825098194558153017567173613320698112509961818815930416903515988885193458072738667385894228792284998920868058257492796104841984443634632449684875602336248270419786232090021609902353043699418491463140934317381436405462531520961836908887070167683964243781405927145635490613031072085103837505101157477041718986106873969655212671546889570350354

## 環境情報

| 機能 | バージョン |
| ---- | ---- |
| Linux / Ubuntu| 20.04 |
| Rust | 1.63.0 |

## 環境構築

```bash
# イロイロ最新に
sudo apt update
sudo apt upgrade
```

## Rust インストール

```bash
# インストールスクリプトの実行
curl https://sh.rustup.rs -sSf | sh
# インストール設定はデフォルト(1)で!!!

# 次に環境変数(PATH)を設定します。
export PATH="$HOME/.cargo/bin:$PATH"

# 最後に正しくインストール、パスの設定がされたか、以下のコマンドで確認します。
cargo --version
# -> cargo 1.63.0
rustc 1.63.0
# -> rustc 1.63.0
rustdoc --version
# -> rustdoc 1.63.0
```

## 実行方法

```bash
# テスト実行
cargo run

# コンパイルして、、、
cargo build --release

# 実行!!!
./target/release/julia_rs
```
