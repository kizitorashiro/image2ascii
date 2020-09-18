//! convert image file or string to ascii art

use image::imageops::FilterType;
use rusttype::{point, Font, Scale, PositionedGlyph};
use std::fs::File;
use std::io::{self, BufWriter, Write, Read};
use std::cmp;
use image::GenericImageView;
use rand::{thread_rng, Rng};

/// characters that is used to make ascii art 
pub static ASCIIS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!\"#$%&'()-^\\=~|@[`{;:]+*},./_<>?_     ";
//pub static ASCIIS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890!\"#$%&'()-^\\=~|@[`{;:]+*},./_<>?_ ";

/// 2-d array of chars
pub struct Char2DArray {
    /// 2-d array of chars by Vec 
    pub buffer: Vec<Vec<char>>,
    height: usize,
    width: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct CharPosition {
    pub x: i32,
    pub y: i32,
}

impl Char2DArray {
    /// create Char2DArray
    /// create Char2DArray and initialize value by ' '
    pub fn new(width: usize, height: usize) -> Char2DArray {
        let mut array2d = Char2DArray{
            buffer: Vec::with_capacity(height),
            height: height,
            width: width,
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

    pub fn debug_print(&self) {
        let lines = self.to_lines();
        for line in lines {
            println!("{}", line);
        }
    }

    ///create Char2DArray from Vec<Vec<char>>
    pub fn from(src: Vec<Vec<char>>) -> Char2DArray {
        let mut c2d = Char2DArray::new(src[0].len(), src.len());
        for y in 0..c2d.height() {
            for x in 0..c2d.width() {
                c2d.buffer[y][x] = src[y][x];
            }
        }
        c2d
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

    /// get number of lines
    pub fn height(&self) -> usize {
        self.height
    }

    /// get number of rows
    pub fn width(&self) -> usize {
        self.width
    }

    /// overwrite data by another Char2DArray
    pub fn overwrite_rect(&mut self, rect: &Char2DArray, position: CharPosition, transparent: Option<char>) {
        let y_start = cmp::max(0, position.y);
        let y_end = cmp::min(self.height() as i32, position.y + rect.height() as i32);
        let x_start = cmp::max(0, position.x);
        let x_end = cmp::min(self.width() as i32, position.x + rect.width() as i32);

        for y in y_start..y_end {
            for x in x_start..x_end {
                match transparent {
                    Some(ch) => {
                        if rect.buffer[(y - position.y) as usize][(x - position.x) as usize] == ch {
                            continue;
                        }
                    },
                    _ => {}
                }
                self.buffer[y as usize][x as usize] = rect.buffer[(y - position.y) as usize][(x - position.x) as usize];
            }
        }
    }
 

    pub fn overwrite_rect_center(&mut self, rect: &Char2DArray, position: CharPosition, transparent: Option<char>) {
        let center_x = (self.width() / 2) - (rect.width() / 2);    
        let center_y = (self.height() / 2) - (rect.height() / 2);
        let offset_posi = CharPosition {
            x: position.x + center_x as i32,
            y: position.y + center_y as i32,
        };
        self.overwrite_rect(rect, offset_posi, transparent)
    }
    

    pub fn overwrite_line(&mut self, ch: char, line_index: usize) {
        for x in 0..self.width() {
            self.buffer[line_index][x] = ch;
        }
    }
    
    pub fn overwrite_char(&mut self, ch: char, position: CharPosition){
        self.buffer[position.y as usize][position.x as usize] = ch;
    }

    pub fn overwrite_char_all(&mut self, ch: char) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.buffer[y][x] = ch;
            }
        }
    }
    
    pub fn copy_from(&mut self, src: &Char2DArray) {
        self.overwrite_rect(src, CharPosition{x:0,y:0}, Option::None);
    }

    pub fn overwrite_fn<F: Fn(usize, usize, char) -> bool>(&mut self, ch: char, f: F){
        for y in 0..self.height() {
            for x in 0..self.width() {
                if f(x, y, self.buffer[y as usize][x as usize]) {
                    self.buffer[y as usize][x as usize] = ch;
                }
            }
        }
    }

    pub fn overwrite_random_fn<F: Fn(usize, usize, char) -> bool>(&mut self, chars: Vec<char>, f: F){
        let mut rng = thread_rng();
        for y in 0..self.height() {
            for x in 0..self.width() {
                if f(x, y, self.buffer[y as usize][x as usize]) {
                    let index: usize = rng.gen_range(0, chars.len());
                    self.buffer[y as usize][x as usize] = chars[index];
                }
            }
        }
    }
    

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

/// convert string to ascii art 
pub fn string2ascii(message: &str, height: f32, ch: char, ch2nd: Option<(usize, char)>, font_file: Option<&str>) -> Result<Char2DArray, String> {
    let font = if let Some(file) = font_file {
        let font_data = std::fs::read(&file).unwrap();
        Font::try_from_vec(font_data).unwrap()
    } else {
        let font_data = include_bytes!("../font/OpenSans-Regular.ttf");
        Font::try_from_bytes(font_data as &[u8]).unwrap()
    };
    let pixel_height = height.ceil() as usize;
    let scale = Scale {
        x: height * 2.0,
        y: height,
    };
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    let glyphs: Vec<_> = font.layout(&message, scale, offset).collect();

    let width = glyphs
        .iter()
        .rev()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .next()
        .unwrap_or(0.0)
        .ceil() as usize;
    
    let mut c2d = Char2DArray::new(width, pixel_height);
    let latter = ch2nd.unwrap_or((0, ch));
    for (i, g) in glyphs.iter().enumerate() {
        if let Some(bb) = g.pixel_bounding_box() {
            g.draw(|x, y, v| {
                let mut c = ' ';
                if v > 0.5 {
                    if i < latter.0 {
                        c = ch;
                    } else {
                        c = latter.1;
                    }
                }
                let x = x as i32 + bb.min.x;
                let y = y as i32 + bb.min.y;
                if x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
                    c2d.buffer[y as usize][x as usize] = c;
                }
            });
        }
    }
    Ok(c2d)
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
fn char2darray_height_width_works() {
    let c2d = Char2DArray::new(5, 10);
    assert_eq!(c2d.height(), 10);
    assert_eq!(c2d.width(), 5);
}

#[test]
fn char2darray_from_works() {
    let c2d = Char2DArray::from(
        vec![
            vec!['A', 'B', 'C'],
            vec!['X', 'Y', 'Z'],
        ]
    );
    assert_eq!(c2d.width(), 3);
    assert_eq!(c2d.height(), 2);
    assert_eq!(c2d.buffer[0], ['A', 'B', 'C']);
    assert_eq!(c2d.buffer[1], ['X', 'Y', 'Z']);
}

#[test]
fn char2darray_overwrite_rect_works() {
    let mut base = Char2DArray::from(vec![
        vec!['A', 'B', 'C'],
        vec!['X', 'Y', 'Z'],
    ]);
    let rect = Char2DArray::from(vec![
        vec!['1', '2'],
        vec!['3', '4'],
    ]);
 
    let pos = CharPosition{x: 0, y: 0};
    base.overwrite_rect(&rect, pos, Option::None);

    assert_eq!(base.buffer[0], ['1', '2', 'C']);
    assert_eq!(base.buffer[1], ['3', '4', 'Z']);

    let mut base = Char2DArray::from(vec![
        vec!['A', 'B', 'C'],
        vec!['X', 'Y', 'Z'],
    ]);
 
    let pos = CharPosition{x: -1, y: -1};
    base.overwrite_rect(&rect, pos, Option::None);

    assert_eq!(base.buffer[0], ['4', 'B', 'C']);
    assert_eq!(base.buffer[1], ['X', 'Y', 'Z']);

    let mut base = Char2DArray::from(vec![
        vec!['A', 'B', 'C'],
        vec!['X', 'Y', 'Z'],
    ]);
 
    let pos = CharPosition{x: 1, y: 1};
    base.overwrite_rect(&rect, pos, Option::None);

    assert_eq!(base.buffer[0], ['A', 'B', 'C']);
    assert_eq!(base.buffer[1], ['X', '1', '2']);

}
#[test]
fn char2darray_overwrite_rect_center_works() {
    let mut base = Char2DArray::from(vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['E', 'F', 'G', 'H'],
        vec!['I', 'J', 'K', 'L'],
        vec!['M', 'N', 'O', 'P'],
    ]);
    let rect = Char2DArray::from(vec![
        vec!['1', '2'],
        vec!['3', '4'],
    ]);
 
    let pos = CharPosition{x: 0, y: 0};
    base.overwrite_rect_center(&rect, pos, Option::None);

    assert_eq!(base.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(base.buffer[1], ['E', '1', '2', 'H']);
    assert_eq!(base.buffer[2], ['I', '3', '4', 'L']);
    assert_eq!(base.buffer[3], ['M', 'N', 'O', 'P']);
    
}

#[test]
fn char2darray_overwrite_rect_center_trans_works() {
    let mut base = Char2DArray::from(vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['E', 'F', 'G', 'H'],
        vec!['I', 'J', 'K', 'L'],
        vec!['M', 'N', 'O', 'P'],
    ]);
    let rect = Char2DArray::from(vec![
        vec!['\u{0000}', '2'],
        vec!['3', '4'],
    ]);
 
    let pos = CharPosition{x: 0, y: 0};
    base.overwrite_rect_center(&rect, pos, Some('\u{0000}'));

    assert_eq!(base.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(base.buffer[1], ['E', 'F', '2', 'H']);
    assert_eq!(base.buffer[2], ['I', '3', '4', 'L']);
    assert_eq!(base.buffer[3], ['M', 'N', 'O', 'P']);
    
}
#[test]
fn char2darray_overwrite_line_works() {
    let mut base = Char2DArray::from(vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['E', 'F', 'G', 'H'],
        vec!['I', 'J', 'K', 'L'],
        vec!['M', 'N', 'O', 'P'],
    ]);
    base.overwrite_line('@', 1);
    assert_eq!(base.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(base.buffer[1], ['@', '@', '@', '@']);
    assert_eq!(base.buffer[2], ['I', 'J', 'K', 'L']);
    assert_eq!(base.buffer[3], ['M', 'N', 'O', 'P']);
}
#[test]
fn char2darray_overwrite_char_works(){
    let mut base = Char2DArray::from(vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['E', 'F', 'G', 'H'],
        vec!['I', 'J', 'K', 'L'],
        vec!['M', 'N', 'O', 'P'],
    ]);
    base.overwrite_char('@', CharPosition{x: 2, y:1});
    assert_eq!(base.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(base.buffer[1], ['E', 'F', '@', 'H']);
    assert_eq!(base.buffer[2], ['I', 'J', 'K', 'L']);
    assert_eq!(base.buffer[3], ['M', 'N', 'O', 'P']);
}

#[test]
fn char2darray_overwrite_fn_works(){
    let mut base = Char2DArray::from(vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['E', 'F', 'G', 'H'],
        vec!['I', 'J', 'K', 'L'],
        vec!['M', 'N', 'O', 'P'],
    ]);
    let threshold = 2;
    base.overwrite_fn('@', |_, y,_| {
        if y >= threshold {
            true
        }else {
            false
        }
    });
    assert_eq!(base.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(base.buffer[1], ['E', 'F', 'G', 'H']);
    assert_eq!(base.buffer[2], ['@', '@', '@', '@']);
    assert_eq!(base.buffer[3], ['@', '@', '@', '@']);

}
