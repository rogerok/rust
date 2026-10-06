use std::process::Command;

fn program_output() -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rust"))
        .output()
        .expect("Не удалось запустить программу");

    assert!(
        output.status.success(),
        "Программа завершилась с ошибкой: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout).expect("Вывод программы должен быть UTF-8")
}

#[test]
fn fizzbuzz_prints_every_number_from_1_to_100_with_correct_replacements() {
    let output = program_output();
    let lines: Vec<_> = output.lines().take(100).collect();

    assert_eq!(lines.len(), 100, "Для FizzBuzz нужны все 100 строк");

    for n in 1..=100 {
        let expected = if n % 15 == 0 {
            "FizzBuzz".to_owned()
        } else if n % 3 == 0 {
            "Fizz".to_owned()
        } else if n % 5 == 0 {
            "Buzz".to_owned()
        } else {
            n.to_string()
        };

        assert_eq!(lines[n - 1], expected, "Неверная строка для числа {n}");
    }
}

#[test]
fn fibonacci_prints_the_first_number_strictly_greater_than_10000() {
    let output = program_output();
    let lines: Vec<_> = output.lines().collect();

    assert_eq!(
        lines.len(),
        101,
        "Ожидаются 100 строк FizzBuzz и одна строка с числом Фибоначчи"
    );

    let result: u64 = lines[100]
        .parse()
        .expect("В строке 101 должно быть только целое неотрицательное число");

    assert_eq!(
        result, 10_946,
        "Нужно первое число Фибоначчи больше 10 000, а не предыдущее или следующее"
    );
}
