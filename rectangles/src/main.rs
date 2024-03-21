// This is
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    // let's make it clearer...
    // let rect = (30, 50);
    // ...even more clearer
    let rect =dbg!( Rectangle {
        width: 30,
        height: 50,
    });
    //FIXME: you have to define Display trait for your structs
    // It doesn't work with :? format (debug) either, because it requires Debug trait
    // println!("{:#?}", rect);
    dbg!(&rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        // area(width1, height1)
        // area(rect)
        area(&rect)
    );
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
// fn area (dimensions: (u32,u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
