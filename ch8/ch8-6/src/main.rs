/* if let */


#![allow(unused)]
fn main() {
    // `optional`という変数の型を`Option<i32>`に指定
    let optional = Some(7);

    // 列挙型をマッチさせるとき、場合によってはmatchを使用すると不自然な書き方になってしまう場合がある
    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        },
        _ => {},
    };



    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // もしletがnumberをデストラクトした結果が`Some(i)`になるならば、ブロック内(`{}`)を実行する。
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // デストラクトした結果が`Some()`にならない場合の処理を明示したい場合、`else`を使用する。
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}


