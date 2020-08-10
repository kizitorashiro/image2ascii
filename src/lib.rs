//! convert image file to ascii art

use image::imageops::FilterType;
use rusttype::{point, Font, Scale, PositionedGlyph};
use std::fs::File;
use std::io::{self, BufWriter, Write, Read};
use image::GenericImageView;

/// characters that is used to make ascii art 
pub static ASCIIS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!\"#$%&'()-^\\=~|@[`{;:]+*},./_<>?_ ";

/// 2-d array of chars
pub struct Char2DArray {
    /// 2-d array of chars by Vec 
    pub buffer: Vec<Vec<char>>,
}

impl Char2DArray {
    /// create Char2DArray
    /// create Char2DArray and initialize value by ' '
    pub fn new(width: usize, height: usize) -> Char2DArray {
        let mut array2d = Char2DArray{
            buffer: Vec::with_capacity(height),
        };
        for _ in 0..height {
            let mut line: Vec<char> = Vec::with_capacity(width);
            for _ in 0..width {
                line.push(' ');
            }
            array2d.buffer.push(line);
        }        
        array2d
    }

    /// conver to Vec<String>
    pub fn to_lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for chars in self.buffer.iter() {
            let line: String = chars.into_iter().collect();
            lines.push(line); 
        }
        lines
    }

    /// save to file
    pub fn save(&self, save_file: &str) -> io::Result<()>{
        let file = File::create(save_file)?;
        let mut writer = BufWriter::new(file);
        let lines = self.to_lines();
        for line in lines.iter() {
            writer.write_all(line.as_bytes())?;
            writer.write(&[10u8])?; // LF
        }
        writer.flush()?;
        Ok(())
    }
    /*
    fn overwrite_rect(&mut self, pos_x: usize, pos_y: usize, width: usize, height: usize) {

    }
    */
}

/// pad String to 256 characters
fn pad(ascii_list: &str, target_size: usize) -> String {
    let mut padded_list = String::from(ascii_list);
    let list_size = ascii_list.len();
    let diff = target_size - list_size;
    let n = diff / list_size;
    let m = diff % list_size;
    for _ in 0..n {
        padded_list.push_str(ascii_list);
    }
    padded_list.push_str(&ascii_list[0..m]);
    padded_list
}

/// calculate density of each ascii character
fn ascii2density(ascii_list: &String) -> Vec<(u32, char)> {
    let font = Vec::from(include_bytes!("../font/OpenSans-Regular.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let scale = Scale {
        x: 20.0,
        y: 20.0,
    };

    let mut density_ascii: Vec<(u32,char)> = Vec::new();
    for ch in ascii_list.chars() {
        let v_metrics = font.v_metrics(scale);
        let offset = point(0.0, v_metrics.ascent);
        let glyphs: Vec<PositionedGlyph<'_>> = font.layout(&ch.to_string(), scale, offset).collect();

        for g in glyphs {
            let mut target_pixels = 0;
            g.draw(|_, _, _|{
                target_pixels = target_pixels + 1;
            });
            density_ascii.push((target_pixels, ch));
        }
    }
    density_ascii.sort_by(|a, b| (b.0).cmp(&a.0));    
    density_ascii
}

/// convert image file to ascii art
pub fn image2ascii(image_file: &str, target_width: u32, contrast: Option<f32>, characters: Option<&str>) -> Result<Char2DArray, String> {
    let density_ascii = ascii2density(&pad(characters.unwrap_or(ASCIIS), 256));
    if let Ok(img) = image::open(image_file){
        // adjust contrast 
        let img = img.adjust_contrast(contrast.unwrap_or(30.0));

        // resize
        let height = img.height();
        let width = img.width();
        let scale: f32 = target_width as f32 / width as f32;
        let target_height: f32 = (height as f32 * scale) / 2.0;
        let target_height = target_height as u32;
        let resized_img = img.resize_exact(target_width, target_height, FilterType::Lanczos3);

        // grayscale
        let luma_img = resized_img.to_luma();

        // pack to Char2DArray
        let mut char2d: Char2DArray = Char2DArray::new(target_width as usize, target_height as usize); 
        for (x, y, pixel) in luma_img.enumerate_pixels() {
            let index = pixel[0] as usize;
            char2d.buffer[y as usize][x as usize] = density_ascii[index].1;
        }
        Ok(char2d)
    } else {
        Err(format!("can not open file {}",image_file))
    }
}

#[test]
fn char2darray_new_works() {
    let w = 10;
    let h = 20;
    let c2d = Char2DArray::new(w, h);
    assert_eq!(c2d.buffer.len(), h);
    for line in c2d.buffer.iter() {
        assert_eq!(line.len(), w);
        for ch in line.iter() {
            assert_eq!(ch, &' ');
        }
    }
}

#[test]
fn char2darray_to_lines_works() {
    let w = 3;
    let h = 2;
    let mut c2d = Char2DArray::new(w, h);

    c2d.buffer[0][0] = 'A';
    c2d.buffer[0][1] = 'B';
    c2d.buffer[0][2] = 'C';
    
    c2d.buffer[1][0] = 'X';
    c2d.buffer[1][1] = 'Y';
    c2d.buffer[1][2] = 'Z';

    let lines = c2d.to_lines();
    assert_eq!(lines[0], String::from("ABC"));
    assert_eq!(lines[1], String::from("XYZ"));
}

#[test]
fn char2darray_save() {
    let w = 3;
    let h = 2;
    let mut c2d = Char2DArray::new(w, h);

    c2d.buffer[0][0] = 'A';
    c2d.buffer[0][1] = 'B';
    c2d.buffer[0][2] = 'C';
    
    c2d.buffer[1][0] = 'X';
    c2d.buffer[1][1] = 'Y';
    c2d.buffer[1][2] = 'Z';

    let tmp = tempfile::NamedTempFile::new().unwrap();
    let ret = c2d.save(tmp.path().to_str().unwrap());
    assert!(ret.is_ok());

    let mut save_file = tmp.reopen().unwrap();
    //let reader = BufReader::new(save_file);
    let mut data = String::new();
    let ret = save_file.read_to_string(&mut data);
    assert!(ret.is_ok());
    assert_eq!(data, "ABC\nXYZ\n");
    
}

#[test]
fn pad_works() {
    let text = String::from("ABC");
    let padded = pad(&text, 10);
    assert_eq!(padded, "ABCABCABCA");
}

#[test]
fn ascii2density_works() {
    let asciis = String::from("A.@=");
    let density_ascii = ascii2density(&asciis);
    assert_eq!(density_ascii.len(), 4);
    assert_eq!(density_ascii[0].1, '@');
    assert_eq!(density_ascii[1].1, 'A');
    assert_eq!(density_ascii[2].1, '=');
    assert_eq!(density_ascii[3].1, '.');
}

#[test]
fn image2ascii_works() {
    let c2d = image2ascii("./img/black.png", 10, Option::None, Option::None).unwrap();
    assert_eq!(c2d.buffer.len(), 6);
    for line in c2d.buffer.iter() {
        assert_eq!(line.len(), 10);
        for pixel in line.iter(){
            assert_eq!(pixel, &'@');
        }
    }
    let c2d = image2ascii("./img/pink.png", 10, Option::None, Option::None).unwrap();
    assert_eq!(c2d.buffer.len(), 6);
    for line in c2d.buffer.iter() {
        assert_eq!(line.len(), 10);
        for pixel in line.iter(){
            assert_eq!(pixel, &' ');
        }
    }
    
}