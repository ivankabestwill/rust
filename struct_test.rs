
use std::mem::transmute;

#[derive(Debug)]
struct Info1{
    a: String,
}

#[derive(Debug)]
struct Info2{
    b: String,
}

#[derive(Debug)]
struct Info{
    a: Info1,
    b: Info2,
    c: usize,
}

fn test(info: &mut Info, info1: &mut Info1, info2: &Info2){
    println!("{}\n{}\n{}", info.c, info1.a, info2.b);
    info1.a = String::from("change info1");
}

fn main() {
    let a = Info1 {
        a: String::from("info1"),
    };
    let b = Info2 {
        b: String::from("info2"),
    };
    let mut c = Info {
        a: a,
        b: b,
        c: 2,
    };

    let tmp_a = {
        let tmp_a = &c.a;
        let tmp_a = unsafe { transmute::<&Info1, *mut Info1>(tmp_a) };
        let tmp_a = unsafe { transmute::<*mut Info1, &mut Info1>(tmp_a) };
        tmp_a
    };

    let tmp_b = {
        let tmp_b = &c.b;
        let tmp_b = unsafe { transmute::<&Info2, *mut Info2>(tmp_b) };
        let tmp_b = unsafe { transmute::<*mut Info2, &mut Info2>(tmp_b) };
        tmp_b
    };

    test(&mut c, tmp_a, tmp_b);
    println!("{:?}", c);
}

