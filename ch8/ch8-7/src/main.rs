/* while let */

#![allow(unused)]
fn main() {
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("`i` is `{:?}`. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            // デストラクトに失敗した場合、ループを脱出
            _ => { break; }
        }
    }



    let mut optional2 = Some(0);
    // `let`が`optional2`を`Some(i)`にデストラクトしている間はブロック内(`{}`)を評価
    while let Some(i) = optional2 {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional2 = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional2 = Some(i + 1);
        }
    }
}
