/* forとrange  */

/* 
for in文を用いることで、イテレータ(Iterator)のそれぞれの要素に対して処理をすることが可能です。
イテレータを作る最も単純な方法はa..bのような書き方をすることです。
これは「a」から「bのひとつ前」までの要素を順に産出するというものです。 
*/

fn main() {
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    // `n`は1, 2, ...., 100のそれぞれの値を取ります。
    // 1..=100でも同じ
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}