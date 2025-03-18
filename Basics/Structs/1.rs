// Basic Struct 
struct Obj {
    height: i32,
    width: i32,
}
// bound to object as method
impl Obj {
    fn area(&self) -> i32 {
        self.height*self.width
     }
    fn show(&self){
        println!("{}x{} is this {}", self.height,self.width,self.area());
    }
}
// related functions
impl Obj {
    fn new(h:i32,w:i32) ->Obj {
        Obj{
            height: h,
            width:w,
        }
    }
}
// basic func for area cal
/* 
fn area(obj:&Obj) -> i32 {
   obj.height*obj.width
}
*/

fn main(){
    let o = Obj {
        height : 12,
        width : 14,
    };
    let o2 = Obj::new(25,50);
    o2.show();
    o.show();
}