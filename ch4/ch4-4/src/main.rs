/* 
データを同じ名前のイミュータブルな変数に束縛しなおすと、データは凍結されます。
凍結したデータは、イミュータブルな束縛がスコープ外になるまで変更できません。
*/

fn main() {
    let mut _mutable_integer = 7i32;

    {
        // イミュータブルな`_mutable_integer`でシャドーイングする
        let _mutable_integer = _mutable_integer;

        // エラー! `_mutable_integer`はこのスコープでは凍結している。
        // _mutable_integer = 50;
    }

    // OK! `_mutable_integer`はこのスコープでは凍結していない。
    _mutable_integer = 3;
}

