/* 
Rustは様々な基本データ型(primitives)の使用をサポートしています。以下がその例です。

スカラー型
符号付き整数: i8, i16, i32, i64, i128, isize（ポインタのサイズ）
符号無し整数: u8, u16, u32, u64, u128, usize（ポインタのサイズ）
浮動小数点数: f32, f64
char: 'a', 'α', '∞'などのUnicodeのスカラー値
bool: trueまたはfalse
ユニット型: ()が唯一の値
ユニット型はその値がタプルですが、複合型とはみなされません。内部に複数の値を含んでいるわけではないからです。

複合型
配列: e.g. [1, 2, 3]など
タプル: e.g. (1, true)
変数は常に 型指定(type annotate)可能 です。数値型の場合はさらにサフィックスでの指定が可能です。指定しない場合デフォルトになります。例えば整数はi32が、浮動小数点はf64がデフォルトです。また、Rustは文脈から型を推定することもできます。
*/

fn main() {
    // 変数に型を指定
    let logical: bool = true;

    // 数値型の場合のみサフィックスでの指定が可能
    let a_float: f64 = 1.0;  // 通常の型指定
    let an_integer   = 5i32; // サフィックスによる型指定

    // サフィックスを指定しない場合、デフォルト
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // 型を文脈から推定することも可能
    let mut inferred_type = 12; // 型 i64 は次行の内容に基づいて推定
    inferred_type = 4294967296i64;
    
    // ミュータブルな変数は値を変更できる
    let mut mutable = 12; // ミュータブルな `i32`
    mutable = 21;
    
    // エラー！ ミュータブルな変数でも型は不変
    // mutable = true;
    
    // 変数はシャドーイングによって上書きできる
    let mutable = true;
}

