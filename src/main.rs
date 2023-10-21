use std::arch::aarch64::float64x1_t;

fn main(){
    let mut count =0;
    let mut flag=-1.0;
    let mut result =1.0;
    let mut i=3.0;
    while 1.0/i>0.000001{
        count=count+1;
        result=result+flag*(1.0/i);
        flag=-flag;
        i=i+2.0;
    }
    print!("result={:.5}",result*4.0);
}
