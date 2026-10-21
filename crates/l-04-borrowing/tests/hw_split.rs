use l_04_borrowing::halves;

#[test]
fn even_length_splits_into_equal_halves() {
    let values = [1, 2, 3, 4];
    let (left, right) = halves(&values);
    assert_eq!(left, &[1, 2], "левая половина [1, 2, 3, 4]");
    assert_eq!(right, &[3, 4], "правая половина [1, 2, 3, 4]");
}

#[test]
fn odd_length_puts_middle_into_right_half() {
    let values = [1, 2, 3, 4, 5];
    let (left, right) = halves(&values);
    assert_eq!(left, &[1, 2], "при нечётной длине левая половина короче");
    assert_eq!(
        right,
        &[3, 4, 5],
        "средний элемент уходит в правую половину"
    );
}

#[test]
fn empty_slice_gives_two_empty_halves() {
    let (left, right) = halves(&[]);
    assert!(
        left.is_empty() && right.is_empty(),
        "у пустого среза обе половины пустые"
    );
}

#[test]
fn single_element_goes_to_right_half() {
    let (left, right) = halves(&[7]);
    assert!(left.is_empty(), "левая половина из одного элемента пустая");
    assert_eq!(right, &[7], "единственный элемент уходит вправо");
}

#[test]
fn accepts_vec_and_its_subslice() {
    let values = vec![10, 20, 30, 40, 50, 60];
    let (left, right) = halves(&values);
    assert_eq!(
        (left, right),
        (&[10, 20, 30][..], &[40, 50, 60][..]),
        "вектор"
    );

    let (left, right) = halves(&values[1..5]);
    assert_eq!(
        (left, right),
        (&[20, 30][..], &[40, 50][..]),
        "срез вектора"
    );
}

#[test]
fn both_halves_are_usable_at_the_same_time() {
    let values = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = halves(&values);
    let total: i32 = left.iter().sum::<i32>() + right.iter().sum::<i32>();
    assert_eq!(total, 21, "сумма двух половин равна сумме всего среза");
    assert_eq!(
        values.len(),
        6,
        "исходный вектор по-прежнему доступен для чтения"
    );
}

#[test]
fn halves_are_windows_into_original() {
    let values = vec![1, 2, 3, 4];
    let (left, right) = halves(&values);
    assert_eq!(
        left.as_ptr(),
        values.as_ptr(),
        "левая половина должна смотреть в исходные данные, а не в копию"
    );
    assert_eq!(
        right.as_ptr(),
        values[2..].as_ptr(),
        "правая половина должна смотреть в исходные данные, а не в копию"
    );
}
