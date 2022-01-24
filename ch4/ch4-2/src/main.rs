// 変数はスコープを持つため、 ブロック の中に閉じ込められています。ブロックとは{}で囲まれた領域のことです。

fn main() {
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        // この変数はスコープ外の同名の変数を *シャドーイング* します。
        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // `short_lived_binding`はこのスコープ内には存在しませんのでエラーとなります。
    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding); // ブロック内で定義した5ではなく1のまま
    
    // この変数バインディングも以前に定義した変数を *シャドーイング* します
    let long_lived_binding = 'a';
    println!("outer long: {}", long_lived_binding);
}
