fn main() {
    let mut count = 0u32;

    // 無限ループ
    loop {
        count += 1;

        if count == 3 {
            // ここの処理をスキップ
            continue;
        }

        println!("{}", count);

        if count == 5 {
            // ループを抜ける。
            break;
        }
    }
}

