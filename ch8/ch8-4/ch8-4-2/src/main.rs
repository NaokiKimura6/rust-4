/* forとイテレータ */

fn main() {
    /*  iter
        各要素を借用します。
        よってコレクションには手を加えないので、ループの実行後もコレクションを再利用できます。
    */
    let names1 = vec!["Bob", "Frank", "Ferris"];
    for name in names1.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names1: {:?}", names1); // ["Bob", "Frank", "Ferris"]

    /*  into_iter
        各要素のデータそのものが提供されます。
        データを取り出してしまうと、データはループ内に「移動」してしまうので、ループ実行後にコレクションを再利用することはできません。
    */
    let names2 = vec!["Bob", "Frank", "Ferris"];
    for name in names2.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names2: {:?}", names2); エラー

    /*  iter_mut
        この関数はコレクションの各要素をミュータブルで借用します。
        コレクションの要素をその場で変更できます。
    */
    let mut names3 = vec!["Bob", "Frank", "Ferris"];
    for name in names3.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names3: {:?}", names3); // ["Hello", "Hello", "There is a rustacean among us!"]
}

