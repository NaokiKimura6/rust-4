// TODO  (特にConsについて)

use crate::List::*;

enum List {
    // Cons: これは、要素をラップし、次の要素へのポインタを保持するタプル。
    // Boxを記載しないと無限ループか？？
    Cons(u32, Box<List>),
    // Nil: 連結リストの終端であることを示すノード
    Nil,
}

// 列挙型にはメソッドを付与することができる。
impl List {
    fn new() -> List {
        // `Nil` は `List`型を持つ。
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    // list の長さを返すメソッド
    fn len(&self) -> u32 {
        match *self {
            // `self`をすでに借用しているので、tailの所有権を取ることができない。
            // 代わりに参照を使用する。
            Cons(_, ref tail) => 1 + tail.len(),
            // 空リストならば長さは0
            Nil => 0
        }
    }

    // Listをheap上の文字列として表したものを返すメソッド
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!`は`print!`に似ているが、コンソール上に出力する代わりに、heap上の文字列を返す
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 空の連結リストを作成
    let mut list = List::new();

    // 要素を追加
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 追加後の状態を表示
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
