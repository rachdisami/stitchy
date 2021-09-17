extern crate raster;
extern crate walkdir;

mod sprite;
use sprite::Spritesheet;


fn main(){

    let mut spritesheet = Spritesheet::new(
        "tests/out/", 
        "test", 
        1024, 
        1024, 
        32, 
        32, 
        0, 
        0, 
        raster::Color::rgba(255, 0, 255, 255)).unwrap();

    spritesheet.populate("tests/in/").unwrap();
    spritesheet.stitch().unwrap();

}

//     let parts_count = WalkDir::new(path_in).into_iter().count();
    // let parts_max = (out_w * out_h) / (in_w * in_h); 