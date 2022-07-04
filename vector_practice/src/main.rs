fn main() {
    let mut v = vec![1, 2, 3, 4];

    let second: &i32 = &v[1];
    println!("1.The second element is {}", second);

    match v.get(1) {
        Some(second) => println!("2.The second element is {}", second),
        None => println!("There is no second element."),
    }

    for vi in v.iter() {
        match vi {
            3 => println!("Three"),
            6 => println!("Six"),
            _ => println!("Number {}", vi),
        }
    }

    println!("{:?}", &mut v);

    /*---------------------------------------------*/
    {
        let mut names = vec!["Bob", "Frank", "Ferris"];

        names.push("Takao");
        names.push("Aya");

        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us!"),
                _ => println!("Hello {}", name),
            }
        }

        println!("names: {:?}", names);
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            let x = *i += 50;
            println!("{:?}", x);
        }
    }
}
