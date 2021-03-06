# Chapter03. 一般的なプログラミングの概念
## 3.2. データ型
Rustのデータ型は主に以下。
* スカラー型
    * 整数（ex. `i32`, `u64`, ...）
    * 浮動小数点数（`f32`, `f64`）
    * 論理値（`true`, `false`）
    * 文字（`char`）
        * シングルクォートで表される
* 複合型
    * タプル
        * 各要素の型は同じ型である必要はない
        * 要素へのアクセスは `x.0`, `x.1`,...
        * 分配して各要素を個別の変数に格納可能
        * 長さは固定長。伸ばしも縮めもできない。
    * 配列
        * 各要素の型は同じ型である必要がある
        * 要素へのアクセスは `x[0]`, `x[1]`,...
        * 長さは固定長。伸ばしも縮めもできない。
## 3.3. 関数
* 関数定義において、引数の型は宣言必須
* Rustにおいて式と文の差異は重要
    * 式(expression): 値を返す
        * ex. `5+6`, `6`, `関数呼び出し`, `マクロ呼び出し`, `{}`
        * 終端にセミコロンを含まない
    * 文(statement): 値を返さない
        * 終端にセミコロンを含む
## 3.4. フロー制御
### if式
* Rustにおいて`if`には必ず条件式として論理値(`bool`型)を与える必要がある。
    * 論理値以外の値(`3`, `String`, etc)が自動的に論理値に変換されることはない。
    * `if 3 {...}`や `if "Hello" {...}` はエラーとなる。
    * `if true {...}` , `if false {...}` はOK。
* `if` は式なので`let`文の右辺に持ってくることができる。
    * その場合、`if`, `else`の結果は同じ型で無くてはいけない。
    * `let`で宣言された変数に`if`, `else`の結果が入るが、`if`, `else`の結果が異なると変数の型が一意でなくなるため。
### ループでの繰り返し
* Rustには3種類のループが存在する。
    * `loop`
        * 明示的に止めさせるまで永遠に実行する
    * `while`
        * 条件が真の間ループが走る
    * `for`
        * コレクションの各アイテムに対してコードを実行する
        * 一定の回数ループしたいときも`Range`型を使って`for`で実現する