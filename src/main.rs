fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
fn main() {
    let mut i32_arr = [5, 4, 3, 2, 1];
    println!("i32:");
    println!("{:?}", i32_arr);
    bubble_sort(&mut i32_arr);
    println!("{:?}", i32_arr);

    let mut str_arr = ["c", "d", "a", "aa", "ba", "ae"];
    println!("\nstr:");
    println!("{:?}", str_arr);
    bubble_sort(&mut str_arr);
    println!("{:?}", str_arr);
}
