// 4.1. 所有権とは？ -> 所有権と関数
// fn main() {
//     let s = String::from("hello");  // sがスコープに入る

//     takes_ownership(s);             // sの値が関数にムーブされ...
//                                     // ... ここではもう有効ではない
//                                     // ここ以降でsを使うとエラーになる
    
//     let x = 5;                      // xがスコープに入る

//     makes_copy(x);                  // xも関数にムーブされるが、
//                                     // i32はCopyなので、この後にxを使っても
//                                     // 大丈夫

// } // ここでxがスコープを抜け、sもスコープを抜ける。ただし、sの値はムーブされているので、何も特別なことは起こらない。

// fn takes_ownership(some_string: String) { // some_stringがスコープに入る。
//     println!("some_string: {}", some_string);
// } // ここでsome_stringがスコープを抜け、`drop`が呼ばれる。後ろ盾してたメモリが解放される。

// fn makes_copy(some_integer: i32) { // some_integerがスコープに入る
//     println!("some_integer: {}", some_integer);
// } // ここでsome_integerがスコープを抜ける。何も特別なことはない。



// 4.1. 所有権とは？ -> 戻り値とスコープ
// fn main() {
//     let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
//                                         // ムーブする
//     let s2 = String::from("hello");     // s2がスコープに入る
//     let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
//                                         // 戻り値もs3にムーブされる
// } // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
//   // 何も起きない。s1もスコープを抜け、ドロップされる。

// fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
//                                              // 呼び出した関数にムーブする
//     let some_string = String::from("hello"); // some_stringがスコープに入る
//     some_string                              // some_stringが返され、呼び出し元関数に
//                                              // ムーブされる
// }

// // takes_and_gives_backは、Stringを一つ受け取り、返す。
// fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。
//     a_string  // a_stringが返され、呼び出し元関数にムーブされる
// }


// // 4.2. 参照と借用
// fn main() {
//   let s = String::from("hello");
//   change(&s);
// }
// fn change(some_string: &String) {
//   some_string.push_str(", world");
// }


// // 4.2. 参照と借用 -> 可変な参照
// fn main() {
//   let mut s = String::from("hello");
//   change(&mut s);
// }
// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }


// // 4.2. 参照と借用 -> 可変な参照
// fn main() {
//   let mut s = String::from("hello");
//   let r1 = &mut s;
//   let r2 = &mut s;
//   println!("{}, {}", r1, r2);
// }


// // 4.2. 参照と借用 -> 可変な参照
// fn main() {
//   let mut s = String::from("hello");
//   {
//     let r1 = &mut s;
//   }
//   let r2 = &mut s;
// }


// // 4.2. 参照と借用 -> 可変な参照
// fn main() {
//   let mut s = String::from("hello");
//   let r1 = &s;
//   let r2 = &s;
//   let r3 = &mut s;
//   println!("{}, {}, and {}", r1, r2, r3);
// }


// 4.2. 参照と借用 -> 宙に浮いた参照
fn main() {
  let reference_to_nothing = dangle();
}
fn dangle() -> String {
  let s = String::from("hello");
  s
}