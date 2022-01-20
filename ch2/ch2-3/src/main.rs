/* 
配列はTという単一の型（訳注: ジェネリック型でも可）のオブジェクトの集合です。
それらのオブジェクトはメモリ上の連続した領域に保存されます。配列は[]を用いて生成されます。
長さはコンパイル時には決定されていて、[T; length]という形で指定できます。

スライスは配列に似ていますが、コンパイル時に長さが決定されていません。
スライスは2ワードからなるオブジェクトであり、最初のワードがデータへのポインタ、2番目のワードがスライスの長さです。
ワード長はusizeと同一で、プロセッサのアーキテクチャによって決まります。例えばx86-64では64ビットです。
スライスは配列の一部を借用するのに使用され、&[T]という型シグネチャを持ちます。
 */

use std::mem;

// この関数はスライスを借用する
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 固定長の配列（型シグネチャは冗長なので、なくても可）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // すべての要素を0にする場合
    let ys: [i32; 500] = [0; 500];

    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len`は配列の要素数を返す。
    println!("number of elements in array: {}", xs.len());

    // 配列はスタック上に置かれる
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 配列は自動的にスライスとして借用される。
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    /* 
    スライスは配列の一部を指すことができる。
    [starting_index..ending_index] の形をとり、
    starting_index はスライスの先頭の位置を表し、
    ending_index はスライスの末尾の1つ先の位置を表す。
     */
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // インデックスが範囲外のときはコンパイルエラー
    // println!("{}", xs[5]);
}

