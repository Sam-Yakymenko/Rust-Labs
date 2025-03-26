//file reading
// use std::fs::File;
// use std::io::{self, BufReader};

// use std::io::prelude::*;
// struct Lines<R> {
//     reader: io::BufReader<R>,
//     buf: String,
// }
// impl <R: Read> Lines<R> {
//     fn new(r: R)-> Lines<R> {
//         Lines {reader: io::BufReader::new(r), buf: String::new()}
//     }
//     fn next<'a>(&'a mut self)-> Option<io::Result<&'a str>>{
//         self.buf.clear();
//         match self.reader.read_line(&mut self.buf){
//             Ok(nbytes) => if nbytes == 0 {
//                 None
//             }
//             else {
//                 let line = self.but.trim_right();
//                 Some(Ok(line))
//             },
//             Err(e) => Some(Err(e))  
//         }
//     }
// }

// fn read_all_lines(filename: &str) -> io::Result<()> {
//     let file = File::open(&filename)?;

//     let mut lines = Lines::new(file);
//     while let Some(line) = lines.next(){
//         let line = line?;
//         println!("{}", line);
//     }
//     Ok(())
// }

//write
// use std::fs::File;
// use std::io;
// use std::io::prelude::*;

// fn write_out(f: &str) -> io::Result<()> {
//     let mut out = File::create(f)?;
//     write!(out,"answer is {}\n", 42)?;
//     Ok(())
// }

// fn main(){
//     write_out("test.txt").expect("write failed");
// }

//info about file
// use std::env;
// use std::path::Path;
// 
// fn main(){
    // let file = env::args().skip(1).next().unwrap_or("test.txt".to_string());
    // let path = Path::new(&file);
    // match path.metadata() {
        // Ok(data)=>{
            // println!("type {:?}", data.file_type());
            // println!("len {:?}", data.len());
            // println!("perm {:?}", data.permissions());
            // println!("modified {:?}", data.modified());
        // },
        // Err(e) => println!("error {:?}", e)
    // }
// }

//use std::fs;
//use std::io;
//fn dump_dir(dir: &str) -> io::Result<()> {
//    for entry in fs::read_dir(dir)? {
//        let entry = entry?;
//        let data = entry.metadata()?;
//        let path = entry.path();
//        if data.is_file(){
//            if let Some(ex) = path.extension(){
//                if ex == "rs" && data.len() < 1024 {
//                    println!("{} length {}", path.display(), data.len());
//                }
//            }
//        } 
//    }
//    Ok(())
//}
//
//fn main(){
//    dump_dir("src");
//}

fn main(){
    println!("Hello world!");
}