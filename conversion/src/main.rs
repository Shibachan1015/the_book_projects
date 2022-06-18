// 摂氏華氏変換
// °C = ((°F) -32) ÷ 1.8
// °F = °C × 1.8 + 32  

use std::io;

fn main() {
    // 最終目標　println!("摂氏と華氏双方向に温度を変換できます。");
    // exit()などで正常終了を実装したい。
    // 次は華氏から摂氏に変換を実装する。
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

        let calc_number_ctof: f64 = calc_ctof(current_temperature);

        println!("摂氏{}°Cは", current_temperature);

        println!("華氏{}°Fです。", calc_number_ctof);
    }
}

fn calc_ctof(current_temperature: f64) -> f64 {
    current_temperature * 1.8 + 32.0
}

