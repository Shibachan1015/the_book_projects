// 摂氏華氏変換
// °C = ((°F) -32) ÷ 1.8
// °F = °C × 1.8 + 32

mod my_calc;  // calc_ctofなどの関数のモジュール
use std::io;

fn main() {
    // 最終目標　println!("摂氏と華氏双方向に温度を変換できます。");
    // exit()などで正常終了を実装したい。
    // 次は華氏から摂氏に変換を実装する。
    // |idea| if or match で　摂氏から華氏か華氏から摂氏を判定してそこでctof関数かftocを呼びだせば良いのではないか。
    println!("摂氏を華氏に変換します。");

    loop {
        println!("変換したい温度を入力してください。");

        let mut current_temperature = String::new();

        io::stdin()
            .read_line(&mut current_temperature)
            .expect("行の読み込みに失敗しました。");

        let current_temperature: f64 = match current_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let calc_number_ctof: f64 = my_calc::my_calc::calc_ctof(current_temperature);

        println!("摂氏{}°Cは", current_temperature);

        println!("華氏{}°Fです。", calc_number_ctof);
    }
}






