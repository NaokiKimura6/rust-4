// 変数の宣言だけを行っておき、初期化（定義）をのちに行うことも可能です。 しかし、最後まで初期化されない変数が生じる可能性があるため、ふつうは同時に行われます。

fn main() {
    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // エラー！ 初期化していないため
    // println!("another binding: {}", another_binding);

    another_binding = 1;
    println!("another binding: {}", another_binding);
}
