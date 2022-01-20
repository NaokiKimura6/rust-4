/* 
loopの用途のひとつに「成功するまである処理を再試行する」ことがあります。
もしその処理が値を返すならば、それをコードの他の部分に渡す必要があるでしょう。
breakの後に値を置くと、それがloop式の値として返されます。
*/

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}

