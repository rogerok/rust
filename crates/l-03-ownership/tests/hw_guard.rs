use std::process::Command;

fn guard_output() -> Vec<String> {
    let output = Command::new(env!("CARGO_BIN_EXE_guard"))
        .output()
        .expect("не удалось запустить src/bin/guard.rs");

    assert!(
        output.status.success(),
        "guard завершился с ошибкой: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .expect("вывод должен быть UTF-8")
        .lines()
        .map(str::to_owned)
        .collect()
}

fn last_line_mentioning(lines: &[String], name: &str) -> usize {
    lines
        .iter()
        .rposition(|line| line.contains(name))
        .unwrap_or_else(|| panic!("в выводе нет ни одной строки с именем {name:?}"))
}

#[test]
fn guards_close_in_reverse_order_of_declaration() {
    let lines = guard_output();

    let first = last_line_mentioning(&lines, "first");
    let second = last_line_mentioning(&lines, "second");

    assert!(
        second < first,
        "соединение \"second\" объявлено позже, значит закрыться должно раньше; вывод:\n{}",
        lines.join("\n")
    );
}
