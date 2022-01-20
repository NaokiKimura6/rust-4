/* 
整数1、浮動小数点1.2、文字(char)'a'、文字列"abc"、ブーリアンtrue、ユニット()は、リテラルを使用することで明示することが可能です。

また整数型の場合、リテラルの代わりにプレフィックスに0x、0o、0bを指定することでそれぞれ16進数、8進数、2進数を使うことができます。

可読性のため、_（アンダースコア）を数値リテラルの間に挿入することができます。例えば1_000は1000と、0.000_001は0.000001とそれぞれ同一です。

コンパイラに、どのリテラルを使用するのかを教えてあげなくてはなりません。現在の仕様では、リテラルが32ビット符号無し整数であることを伝える場合、u32サフィックスを、符号付き32ビット整数であればi32を使用します。
 */

fn main() {
    // 整数の足し算
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数の引き算
    println!("1 - 2 = {}", 1i32 - 2); // u32だと負の数にならないように注意

    // 単純な論理演算子
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // ビットワイズ演算
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 可読性のための`_`（アンダースコア）の使用
    println!("One million is written as {}", 1_000_000u32);
}

