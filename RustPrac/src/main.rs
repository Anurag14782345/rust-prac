// struct Rectangle {
//     width: u32,
//     height:u32
// }

// impl Rectangle{
//     fn p(&self){
//         println!("Rectangle width is {} and Rectangle height is {}",self.width,self.height);
//     }
//     fn sq(&self)->bool{
//         self.width==self.height
//     }
//     fn calc(&self) ->u32{
//         self.width*self.height
//     }

// }

// fn main(){
//     let myRec:Rectangle = Rectangle{width:10 , height:220};
//     myRec.p();
//     println!("Rectangle is Square: {}",myRec.sq());
//     println!("Rectangle has area of : {} sq", myRec.calc());
// }



// fn main(){
//     stackex();
//     heapex();
// }

// fn stackex(){
//     let x = 23;
//     let y = 23;
//     let sum = add(x,y);

//     fn add(a:u32,b:u32)->u32{
//         a+b
//     }
//     println!("Sum of x and y is : {}",sum);
// }
// fn heapex(){
//     let mut v: Vec<i32> = Vec::new();
//     v.push(1);
//     v.push(2);
//     v.push(3);
//     v.pop();
    
//     println!("vector has {:?}",v);
// }

// trait Printable{
//     fn print(&self);
//     fn calc(&self);
// }

// struct Rectangle{
//     width:u32,
//     height:u32
// }
// impl Printable for Rectangle{
//     fn print(&self){
//         println!("Rectangle : {}X{} ",self.width,self.height);
//     }
//     fn calc(&self){
//         println!("area is {}",self.width*self.height);
//     }
// }
// fn main(){
//     let rec:Rectangle = Rectangle{width:10,height:10};
//     // println!("Rectangle has width {} ", rec.print());
//     rec.print();
//     rec.calc();
// }

// fn main(){
//     let mut v:Vec<i128> = vec![1,2,3,4,5];
//     println!("{:?}",v);
//     v.push(12);
//     v(2).push(11);
//     println!("{:?}",v);


// }
// use std::env;
// fn  main(){
//     let args: Vec<String> = env::args().collect();
//     println!("{}",args[1]);
//     for args in args.iter(){
//         println!("{}",args);
//     }
// }

use std::collections::HashMap ;
fn main() {
    let mut marks: HashMap<&str,i32> = HashMap::new();
    marks.insert("Rust",22);
    marks.insert("C++",33);
    marks.insert("C",33);
    marks.insert("py",33);

    println!("{:?}",marks);
    println!("{:?}",marks.len());
    match marks.get("py"){
        some()
    }
}