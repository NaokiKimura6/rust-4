/* 
コンパイラには、[#derive]アトリビュートを用いることで型に対して特定のトレイトの標準的な実装を提供する機能があります。より複雑なことを行わせたい場合には、同名のトレイトを手動で実装することも可能です。

以下はderive可能なトレイトの一覧です。

・型の比較に関連するトレイト: Eq, PartialEq, Ord, PartialOrd
・Clone：コピーによって&TからTを作成するトレイト
・Copy：to give a type 'copy semantics' instead of 'move semantics'.
・Hash：&Tからハッシュ値を計算するためのトレイト
・Default：空のインスタンスを作成するためのトレイト
・Debug：{:?}フォーマッタを利用して値をフォーマットするためのトレイト
 */

// `Centimeters`は比較可能なタプルになる
#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);

// `Inches`は{:?}でプリント可能なタプルになる
#[derive(Debug)]
struct Inches(i32);

impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;

        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

fn main() {
    let _one_second = Seconds(1);

    // エラー: `Seconds`はプリントできない。これは`Debug`トレイトを実装していないため
    //println!("One second looks like: {:?}", _one_second);

    // エラー: `Seconds`は比較できない。これは`PartialEq`トレイトを実装していないため
    //let _this_is_true = (_one_second == _one_second);

    let foot = Inches(12);

    println!("One foot equals {:?}", foot);

    let meter = Centimeters(100.0);

    let cmp =
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };

    println!("One foot is {} than one meter.", cmp);
}
