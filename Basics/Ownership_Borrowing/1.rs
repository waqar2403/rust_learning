fn test(v: Vec<i32>)/*-> Vec<i32*/{
    println!("We got this {}",v[0]+v[998]);
    //v this statement will return ownership back to main
}
fn cop(a:i32,b:i32){
    println!("a + b = {}",a+b);
}
fn main(){
    let a = 50; 
    let b = 40;
    let mut v = Vec::new();
    for i in 1..1000 {
        v.push(i);
    }
    test(v);
    //v = test(v) this will return back the ownership
    cop(a,b);
    println!("{} {}",a,b); // this will work bcuz primitve types copy instead of changing ownership
   //println!("Done {}",v[25]); //this will give err bcuz vec is moved to upper scope
}