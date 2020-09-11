
fn change(mut vec: &mut Vec<Vec<u32>>) {
    for v in &mut *vec{
        for elm in v{
            *elm = 2;
        }
    }

    for v in vec{
        for elm in v{
            *elm = 3;
        }
    }
}

fn change1(v: &mut u32){
    *v = 1;
}
fn change2(v: &mut u32){
    *v = 2;
}

fn main() {
    let vec_a = vec![0u32; 3];
    let vec_b = vec![0u32; 3];
    let mut vec = vec![vec_a, vec_b];
    change(&mut vec);

    let mut a = 5u32;
    let b = &mut a;
    //let d = &mut a;
    let c = &mut *b;
    change1(c);
    change2(b);
}
