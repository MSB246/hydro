use image::{RgbImage, Rgb, ImageBuffer, GenericImage};
use image::io::Reader as ImageReader;
use clap::Parser;

type Img = ImageBuffer<Rgb<u8>, Vec<u8>>;

fn average(img: &Img) -> Rgb<u8> {
    let length = (img.width() * img.height()) as f32;
    let mut rgb = Rgb([0.0, 0.0, 0.0]);

    for p in img.pixels() {
        rgb[0] += p[0] as f32 / length;
        rgb[1] += p[1] as f32 / length;
        rgb[2] += p[2] as f32 / length;
    }

    Rgb([
        rgb[0] as u8,
        rgb[1] as u8,
        rgb[2] as u8,
    ])
}

fn average_n(img: &Img) -> usize {
    let avr = average(img);
    let mut diff = 0;

    for p in img.pixels() {
        diff += (
            (p[0] as i32-avr[0] as i32).pow(2)+
            (p[1] as i32-avr[1] as i32).pow(2)+
            (p[2] as i32-avr[2] as i32).pow(2)
        ) as usize;
    }

    diff
}

#[derive(Debug, Clone)]
enum QuadTree {
    Leaf(Rgb<u8>),
    Node(Box<[QuadTree; 4]>),
}

impl QuadTree {
    fn draw_full(&self, img: &mut Img, lines: bool) {
        self.draw(img, 0, 0, img.width(), img.height(), lines)
    }

    fn draw(&self, img: &mut Img, x: u32, y: u32, width: u32, height: u32, lines: bool) {
        match self {
            QuadTree::Leaf(rgb) => {
                for y in y..y+height {
                    for x in x..x+width {
                        img.put_pixel(x, y, *rgb);
                    }
                }
            }
            QuadTree::Node(children) => {
                if width == 1 || height == 1 {
                    QuadTree::Leaf(Rgb([255, 0, 255])).draw_full(img, false);
                    return;
                }

                let w = width/2;
                let h = height/2;
                let wo = width%2;
                let ho = height%2;

                if lines {
                    children[0].draw(img, x, y, w-1, h-1, lines);
                    children[1].draw(img, x+w, y, w+wo-1, h-1, lines);
                    children[2].draw(img, x, y+h, w-1, h+ho-1, lines);
                    children[3].draw(img, x+w, y+h, w+wo-1, h+ho-1, lines);
                } else {
                    children[0].draw(img, x, y, w, h, lines);
                    children[1].draw(img, x+w, y, w+wo, h, lines);
                    children[2].draw(img, x, y+h, w, h+ho, lines);
                    children[3].draw(img, x+w, y+h, w+wo, h+ho, lines);
                }
            }
        }
    }

    fn from_img(mut img: Img, detail: usize) -> QuadTree {
        if img.width() == 1 || img.height() == 1 {
            return QuadTree::Leaf(average(&img));
        }

        let lu = img.sub_image(0, 0, img.width()/2, img.height()/2).to_image();
        let ru = img.sub_image(img.width()/2, 0, img.width()/2, img.height()/2).to_image();
        let ld = img.sub_image(0, img.height()/2, img.width()/2, img.height()/2).to_image();
        let rd = img.sub_image(img.width()/2, img.height()/2, img.width()/2, img.height()/2).to_image();

        if average_n(&img) < detail.pow(2) {
            QuadTree::Node(Box::new([
                QuadTree::Leaf(average(&lu)),
                QuadTree::Leaf(average(&ru)),
                QuadTree::Leaf(average(&ld)),
                QuadTree::Leaf(average(&rd)),
            ]))
        } else {
            QuadTree::Node(Box::new([
                QuadTree::from_img(lu, detail),
                QuadTree::from_img(ru, detail),
                QuadTree::from_img(ld, detail),
                QuadTree::from_img(rd, detail),
            ]))
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(short, long, required=true)]
    filename: String,
    #[arg(short, long, required=true)]
    detail: usize,
    #[arg(short, long)]
    lines: bool,
}

fn main() {
    let args = Args::parse();

    let src = ImageReader::open(args.filename).unwrap().decode().unwrap().to_rgb8();
    let mut img = RgbImage::new(src.width(), src.height());

    let tree = QuadTree::from_img(src, args.detail);
    tree.draw_full(&mut img, args.lines);

    img.save("img.png").unwrap();
}
