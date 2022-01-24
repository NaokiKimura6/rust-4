/* 
structを用いて作成できる構造体(多言語でのクラス、オブジェクトのようなもの)は3種類ある
・タプル(tuple)
・ユニット(unit)
・クラシック
*/

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

// 構造体は他の構造体のフィールドになることができる(#[allow(dead_code)]はch13-1参照)
#[allow(dead_code)]
struct Rectangle {
    // 長方形は座標空間上における左上隅と右下隅の位置によって指定できる
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: &Rectangle) -> f32  {
    let area: f32 = rectangle.top_left.y * rectangle.bottom_right.x;
    area
}

fn main() {
    // 構造体をフィールド初期化の簡略記法で生成
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // 構造体のデバッグ表示を行う(#[derive(Debug)]を付与しているため可能)
    println!("{:?}", peter);


    // `Point` のインスタンス化
    let point: Point = Point { x: 10.3, y: 0.4 };

    // pointのフィールドにアクセスする。
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 構造体の更新記法を用いて、別の構造体のフィールドの値を基に新たなpointを生成
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` の値は `point.y` と同一になるが、これは `point` のフィールドの値を用いて生成したためである
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // pointをデストラクトする。
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    println!("This area is {}", rect_area(&_rectangle));
    println!("This area is {}", rect_area(&_rectangle));

    // ユニットをインスタンス化
    let _unit = Unit;

    // タプルをインスタンス化
    let pair = Pair(1, 0.1);

    // タプルのフィールドにアクセス
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // タプルをデストラクト
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}
