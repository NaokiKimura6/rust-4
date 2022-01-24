// 変数はデフォルトでイミュータブル（変更不可能）ですがmut構文を使用することでミュータブルになります。

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;
    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;
}

