fn main() {
    let mut v: Vec<i32> = Vec::new(); //can init like this
    v.push(1);

    let mut v = vec![1, 2, 3, 4, 5]; //or like this
    v.push(1);

    let mut sum = 0;

    for i in &v {
        sum += i;
    }

    let len = v.len();
    let avg = (sum as f32) / (len as f32);

    println!("Sum: {}, Count: {}, Average: {}", sum, len, avg);
}
