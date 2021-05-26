fn main() {
    let a = [1,2,3,4,5,6,7,8,9,10];
    let result = sum(&a);
    println!("the sum of the list is: {:?}", result);
    let b = [4294967295,1];
    let result = sum(&b);
    println!("the sum of the list is(overflow): {:?}", result);
}



fn sum(a: &[u32]) -> Option<u32> {

    let mut sum: Option<u32> = Some(0);
    for &i in a
    {
        sum = sum.unwrap().checked_add(i);
    }
    sum
}