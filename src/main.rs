use rand::Rng;
use std::io::Write;

fn main()
{
	let num = rand::rng().random_range(0..100);
	loop {
		let mut buf = String::with_capacity(10);

		std::print!("Введите число: ");
		std::io::stdout().flush().expect("Флаш упал");

		if 0 == match std::io::stdin().read_line(&mut buf) {
			Err(_) => {
				std::eprintln!("read_line упал!");
				break;
			}
			Ok(num) => num,
		} {
			// EOF
			std::println!("\nЗавершение игры");
			break;
		}

		let i: i32 = match buf.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				std::eprintln!("Ошибка ввода!");
				continue;
			}
		};

		match i.cmp(&num) {
			std::cmp::Ordering::Less => std::println!("Меньше"),
			std::cmp::Ordering::Greater => std::println!("Больше"),
			std::cmp::Ordering::Equal => {
				std::println!("Вы угадали");
				break;
			}
		}
	}
}
