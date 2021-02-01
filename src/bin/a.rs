use proconio::input;

fn main() {
    input! {a: i32,
    b: i32,
    c: i32
    }

    if a > b{
        println!("Takahashi")
    }
    else if a < b{
        println!("Aoki")
    }
    else {
        if c == 0{
            println!("Aoki")

        }
        else {
            println!("Takahashi")

        }
    }
}
