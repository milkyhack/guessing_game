use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Угадай цифру");
    let secret_number = rand::thread_rng().gen_range(1..101);
loop{

    println!("Введи число чтоб угадать загаданное");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Ошибка чтения");

let guess : u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};

    println!("Твоё число : {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less=> println!("меньше загаданного"),
        Ordering::Greater=>println!("Больше загаданного"),
        Ordering::Equal=> {
            println!("Ты выиграл");
            break;
}
        }
    }
}
