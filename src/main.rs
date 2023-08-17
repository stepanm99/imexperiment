#[macro_use]
extern crate bmp;
use bmp::{Image, Pixel};

#[derive(Clone)]
struct ImageDimensions {
    width: usize,
    height: usize,
}
#[derive(Debug)]
#[derive(Clone)]
struct PixelData {
    x: usize,
    y: usize,
    r: u8,
    g: u8,
    b: u8,
}

struct PixelColor {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let img_dim = ImageDimensions {
        width: 640,
        height: 480,
    };

    let mut image_data: Vec<Vec<PixelData>> = vec![vec![PixelData { x: 0, y: 0, r: 0,
        g: 0, b: 0}; img_dim.height]; img_dim.width];

    matrix_init (&mut image_data, img_dim.width, img_dim.height);

    let mut img = Image::new(img_dim.width.try_into().unwrap(), img_dim.height.try_into().unwrap());

    draw_line(&mut image_data, 0, 0, 639, 479, 50, 80, 200);
    draw_line(&mut image_data, 300, 100, 200, 200, 100, 80, 200);
//    draw_line(&mut image_data, 400, 400, 100, 100, 200, 80, 50);
    matrix_to_img(image_data, &mut img, img_dim.width, img_dim.height);
    let _ = img.save("img.bmp");
}

fn draw_line (image_data: &mut Vec<Vec<PixelData>>, mut x1: usize, mut y1: usize, mut x2: usize, mut y2: usize,
              r: u8, g: u8, b: u8) -> Vec<Vec<PixelData>> {
    if x1 < x2 && y1 < y2 {
        let dx: f32 = (x2 - x1) as f32;
        let dy: f32 = (y2 - y1) as f32;
        let slope: f32 = dy / dx;
        let mut xt: usize = x1;
        let mut yt: usize;
        let mut xi: usize = 0;

        image_data[x1][y1] = PixelData {
            x: x1,
            y: y1,
            r: r,
            g: g,
            b: b,
        };
        loop {
            yt = y1 + (xi as f32 * slope) as usize;
            xt = xt + 1;
            image_data[xt][yt] = PixelData {
                x: xt,
                y: yt,
                r: r,
                g: g,
                b: b,
            };
            xi = xi + 1;
            if xt == x2 {
                image_data[x2][y2] = PixelData {
                    x: x2,
                    y: y2,
                    r: r,
                    g: g,
                    b: b,
                };
                break;
            }

        }
    }

    if x1 > x2 && y1 < y2 {
        let dx: f32 = (x1 - x2) as f32;
        let dy: f32 = (y2 - y1) as f32;
        let slope: f32 = dy / dx;
        let mut xt: usize = x1;
        let mut yt: usize;
        let mut xi: usize = 0;

        image_data[x1][y1] = PixelData {
            x: x1,
            y: y1,
            r: r,
            g: g,
            b: b,
        };
        loop {
            yt = y2 + (xi as f32 * slope) as usize;
            xt = xt + 1;
            image_data[xt][yt] = PixelData {
                x: xt,
                y: yt,
                r: r,
                g: g,
                b: b,
            };
            xi = xi + 1;
            if xt == x2 {
                image_data[x2][y2] = PixelData {
                    x: x2,
                    y: y2,
                    r: r,
                    g: g,
                    b: b,
                };
                break;
            }

        }
    }
    /*if x1 > x2 && y1 > y2 {
        let t = x1;
        let x1 = x2;
        let x2 = t;
        let t = y1;
        let y1 = y2;
        let y2 = t;
        let dx: f32 = (x2 - x1) as f32;
        let dy: f32 = (y2 - y1) as f32;
        let slope: f32 = dy / dx;
        let mut xt: usize = x1;
        let mut yt: usize;

        image_data[x1][y1] = PixelData {
            x: x1,
            y: y1,
            r: r,
            g: g,
            b: b,
        };
        loop {
            yt = y1 + (xt as f32 * slope) as usize;
            xt = xt + 1;
            image_data[xt][yt] = PixelData {
                x: xt,
                y: yt,
                r: r,
                g: g,
                b: b,
            };
            if xt == x2 {
                image_data[x2][y2] = PixelData {
                    x: x2,
                    y: y2,
                    r: r,
                    g: g,
                    b: b,
                };
                break;
            }

        }
    }
    if x1 < x2 && y1 > y2 {
        let dx: f32 = (x2 - x1) as f32;
        let dy: f32 = (y1 - y2) as f32;
        let slope: f32 = dy / dx;
        let mut xt: usize = x1;
        let mut yt: usize;

        image_data[x1][y1] = PixelData {
            x: x1,
            y: y1,
            r: r,
            g: g,
            b: b,
        };
        loop {
            yt = y1 - (xt as f32 * slope) as usize;
            xt = xt + 1;
            image_data[xt][yt] = PixelData {
                x: xt,
                y: yt,
                r: r,
                g: g,
                b: b,
            };
            if xt == x2 {
                image_data[x2][y2] = PixelData {
                    x: x2,
                    y: y2,
                    r: r,
                    g: g,
                    b: b,
                };
                break;
            }

        }
    }
*/

//    draw_point(image_data, 0, 0, 250, 0, 0);

    image_data.to_vec()
}

fn draw_point (image_data: &mut Vec<Vec<PixelData>>, x: usize, y: usize,
               r: u8, g: u8, b: u8 ) -> Vec<Vec<PixelData>> {
    image_data[x][y] = PixelData {
        x: x,
        y: y,
        r: r,
        g: g,
        b: b,
    };
    image_data.to_vec()
}

fn matrix_init (image_data: &mut Vec<Vec<PixelData>>, width: usize, height: usize) -> Vec<Vec<PixelData>> {
        let width: usize = <usize as TryInto<usize>>::try_into(width).unwrap() - 1;
        let height: usize =  <usize as TryInto<usize>>::try_into(height).unwrap() - 1;
        let mut iw: usize = width;
        let mut ih: usize = height;
        let mut ic: usize = 0;

        loop {
            loop {
                image_data[iw][ih] = PixelData {
                    x: iw,
                    y: ih,
                    r: 0,
                    g: 0,
                    b: 0,
                };
                if iw == 0 {
                    break;
                }
                iw = iw - 1;
            }
            ic = ic + 1;
            if ih == 0 {
                break;
            }
            ih = ih - 1;
            iw = width;
        }
        image_data.to_vec()
}

fn matrix_to_img (image_data: Vec<Vec<PixelData>>, img: &mut Image, width: usize, height: usize) {
    let width: usize = <usize as TryInto<usize>>::try_into(width).unwrap() - 1;
    let height: usize =  <usize as TryInto<usize>>::try_into(height).unwrap() - 1;
    let mut iw: usize = width;
    let mut ih: usize = height;
    let mut ic: usize = 0;

    loop {
        loop {
            img.set_pixel(image_data[iw][ih].x.try_into().unwrap(), image_data[iw][ih].y.try_into().unwrap(), px!(image_data[iw][ih].r,
                image_data[iw][ih].g, image_data[iw][ih].b));
            if iw == 0 {
                break;
            }
            iw = iw - 1;
        }
        ic = ic + 1;
        if ih == 0 {
            break;
        }
        ih = ih - 1;
        iw = width;
    }
}