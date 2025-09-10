fn main (){
    let a = 13;
    let b = 2.3;
    letc: f32 = 120.0;

    let average = (a as f32 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");
}
