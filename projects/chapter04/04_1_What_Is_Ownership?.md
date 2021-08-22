# Chapter04. 所有権を理解する
## 4.1. 所有権とは？
### 所有権規則
* Rustの各値は、所有者と呼ばれる変数と対応している。
* いかなる時も所有者は一つである。
* 所有者がスコープから外れたら、値は破棄される。
### 変数スコープ
スコープ: 要素が有効になるプログラム内の範囲のこと
### String型
例として`String`型の所有権について考えていく。  
Rustでは文字列を表現するのに`文字列リテラル`と`String`型がある。
* 文字列リテラル
    * 不変である（イミュータブル）
* String
    * `mut`を用いて可変化できる（ミュータブル）
    * コンパイル時にサイズ不明なテキストも保持可能（ヒープにメモリを確保するため）

`String`は可変化できるのにリテラルができないのは何故か？答えはメモリを扱う方法にある。
### メモリと確保
* 文字列リテラルの場合
    * 中身はコンパイル時に判明している。
    * よってテキストは最終的なバイナリにハードコードされる。
    * コンパイル時にサイズ不明、実行時に可変なテキスト用のメモリをバイナリに確保することは不可能
        * バイナリファイル内に一定のバイト数を持っておき、バイナリファイルを実際のメモリに展開した際、持っておいたバイト分メモリを確保することになるってことか？
* `String`型の場合
    * コンパイル時に不明瞭のメモリを**ヒープ**に確保できる。
    * メモリは実行時にOSに要求される。
    * `String`型を使い終わったらOSにメモリを返還する。そしてその方法が必要になる。
* メモリを返還する方法
    * GCがある場合：GCが使用しないメモリを片付けるため人間が考慮する必要はない。
    * GCが無い場合：明示的に返還するコードを書く必要がある。
    * Rustの場合：GCは無い。メモリを所有している変数がスコープを抜けたら自動的に返還される。
        * 具体的には閉じ括弧`}`で、自動的に返還する関数`drop`が呼び出される。
### 変数とデータの相互作用法: ムーブ
`String`型の例を以下コードに示す。
```Rust
let s1 = String::from("hello");
let s2 = s1;
```
ここで`String`型の変数`s1`は以下の構造をしている。
* 3つの部品でできており、これらはスタックに保持される。
    * 文字列の中身を保持するメモリへのポインタ
    * 長さ：`String`型の中身が現在使用しているメモリ量(byte)
    * 許容量：`String`型がOSから受け取った全メモリ量(byte)
* 文字列の中身はヒープ上のメモリに保持される。

<img src="https://doc.rust-jp.rs/book-ja/img/trpl04-01.svg" width="25%" title="Stringのメモリ上の表現">

`s1`を`s2`に代入すると`String`型のデータがコピーされる。  
スタックにあるポインタ、長さ、 許容量をコピーし、ポインタが指すヒープ上のデータはコピーしない。  
ヒープ上のデータが大きい時、実行性能がとても悪くなる。  
以下図のようになる。

<img src="https://doc.rust-jp.rs/book-ja/img/trpl04-02.svg" width="25%" title="s2のメモリ上での表現">

この状態でスコープを抜けると、`s1`,`s2`が共に同じメモリを開放してしまうのでは？  
二重開放エラーとなってしまうのではないか。

Rustでは問題を解決するため以下のような、所有権を`s1`から`s2`へ移動させるムーブを行う。
* `s1`を`s2`にコピーする代りに`s1`を無効化する。
* `s2`生成後に`s1`を使おうとするとエラーが発生する。
* `s2`だけが有効なのでスコープを抜けると`s2`のみがメモリを開放する。

<img src="https://doc.rust-jp.rs/book-ja/img/trpl04-04.svg" width="25%" title="ムーブのメモリ上の表現">

これで、所有権規則の`いかなる時も所有者は一つである`が守られる。
### 変数とデータの相互作用法: クローン
`clone`メソッドを用いると、スタック上のデータだけでなくヒープデータのコピーが可能。
```Rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);
```
`s1`は無効化されず、各々がメモリを解放して問題ない。
これは以下図で示した動作を生み出す。

<img src="https://doc.rust-jp.rs/book-ja/img/trpl04-03.svg" width="25%" title="ヒープデータのコピー">

### スタックのみのデータ: コピー
```Rust
let x = 5;
let y = x;
println!("x = {}, y = {}", x, y);
```
上記は問題なく動作する。  
整数のようなスタックに保持される型はムーブされず代入後も古い変数が使用可能。  
古い変数を無効化する理由が無いため。

スタックに保持される型には`Copy`トレイトを配置でき、`Copy`に適合していれば代入後も古い変数が使用可能。  
`Copy`に適合する主な型の一部は以下。

* あらゆる整数型。`u32`など。
* 論理値型である`bool`。`true`と`false`。
* あらゆる浮動小数点型、`f64`など。
* 文字型である`char`。
* タプル。ただ、`Copy`の型だけを含む場合。例えば、`(i32, i32)`は`Copy`だが、`(i32, String)`は違う。

### ここまでのまとめ
* ヒープに保持される型（ex. `String`）
    * 保持しているヒープ上メモリを差すポインタ等、スタック上のデータのみコピーする。
        * ヒープ上のデータをそのままコピーするとデータが大きく実行性能が悪いため。
    * コピー元変数は無効化され、コピー先変数のみが有効になる。（ムーブ）
        * コピー元とコピー先で同じヒープ上メモリを見ているため両方が有効のままだとメモリの二重解放となってしまう。
    * ヒープデータごとコピーしたい時は`clone`
* スタックに保持される型（`Copy`に適合している）
    * コピーしてもコピー元コピー先が共に有効のまま。（ムーブされない）
        * 無効化する理由が無いため

### 所有権と関数
関数に値を渡すと、変数に値を代入することと同じくコピーやムーブが起きる。  
例えば`String`が関数に渡されると関数に所有権がムーブし、以降その`String`は無効化されて使えない。  
`Copy`に適合している型の場合、ムーブせずにコピー元先が有効のまま。
```Rust
let s = String::from("hello");  // sがスコープに入る
some_function(s);               // sの値が関数にムーブされ...
                                // ... ここではもう有効ではない。ここ以降でsを使うとエラーになる。
```
### 戻り値とスコープ
関数が値を返すことでも所有権はムーブする。  
例えば`String`が関数に渡されると、関数に所有権がムーブされ、戻り値があるならば所有権は戻り値にムーブされる。
```Rust
let s2 = String::from("hello");     // s2がスコープに入る
let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ、戻り値もs3にムーブされる
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。
    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}
```
関数に所有権をムーブし、再利用したい値はまた所有権と共に返さなくてはいけない。  
この煩わしさを解消するのが**参照**である。

## 参考資料
https://zenn.dev/toga/books/rust-atcoder/viewer/23-string