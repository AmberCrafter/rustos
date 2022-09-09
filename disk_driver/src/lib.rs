pub mod driver;

#[macro_use]
pub mod interface;


pub type Disk = Vec<u8>;

pub fn format_print(data: &[u8]) {
    for (i, v) in data.iter().enumerate() {
        if i%16 == 0 {println!();}
        print!("{:02x} ", v);
    }
    println!();
}