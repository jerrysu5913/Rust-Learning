fn main(){
    let num=0;
    for i in 1000..=8000{
        let a = i / 1000;
        let b = i / 100 % 10;
        let c = i / 10 % 10;
        let d = i % 10;
        if b == 0 || c == 0 || d == 0{
            continue;
        }
        else if a + b + c + d == 10 {
            print!("{} ", i);
        }else {continue;}
    }
}