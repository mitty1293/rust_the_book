# Chapter05. 構造体を使用して関係のあるデータを構造化する
## 5.2. 構造体を使ったプログラム例
構造体を用いることで
* 関連するデータをグループ化し、
* データにラベル付けを行いその意味を明瞭化することができる。
### トレイトの導出で有用な機能を追加する
`{:?}`を用いると、`println!`に`Debug`とよばれる出力整形を使いたいと指示する。  
ただし構造体で実装していない場合、使えるようにするには明示的に`Debug`トレイトを導出する注釈を追加する必要がある。  
`#[derive(Debug)]`を構造体定義直前に追加する。  