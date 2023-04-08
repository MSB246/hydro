use image::{RgbImage, Rgb, ImageBuffer, GenericImage};
use image::io::Reader as ImageReader;

//#[derive(Debug, Clone, Copy)]
//struct QuadTree<Lu, Ru, Ld, Rd>(Lu, Ru, Ld, Rd);

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

#[derive(Debug, Clone)]
enum QuadTree {
    Leaf(Rgb<u8>),
    Node(Box<[QuadTree; 4]>),
}

impl QuadTree {
    fn draw_full(&self, img: &mut Img) {
        self.draw(img, 0, 0, img.width(), img.height())
    }

    fn draw(&self, img: &mut Img, x: u32, y: u32, width: u32, height: u32) {
        match self {
            QuadTree::Leaf(rgb) => {
                for y in y..y+height {
                    for x in x..x+width {
                        img.put_pixel(x, y, *rgb);
                    }
                }
            }
            QuadTree::Node(children) => {
                let w = width/2;
                let h = height/2;

                children[0].draw(img, x, y, w, h);
                children[1].draw(img, x+w, y, w, h);
                children[2].draw(img, x, y+h, w, h);
                children[3].draw(img, x+w, y+h, w, h);
            }
        }
    }
}

impl From<(Img, usize)> for QuadTree {
    fn from((mut img, i): (Img, usize)) -> QuadTree {
        let lu = img.sub_image(0, 0, img.width()/2, img.height()/2).to_image();
        let ru = img.sub_image(img.width()/2, 0, img.width()/2, img.height()/2).to_image();
        let ld = img.sub_image(0, img.height()/2, img.width()/2, img.height()/2).to_image();
        let rd = img.sub_image(img.width()/2, img.height()/2, img.width()/2, img.height()/2).to_image();

        if i == 0 {
            QuadTree::Node(Box::new([
                QuadTree::Leaf(average(&lu)),
                QuadTree::Leaf(average(&ru)),
                QuadTree::Leaf(average(&ld)),
                QuadTree::Leaf(average(&rd)),
            ]))
        } else {
            QuadTree::Node(Box::new([
                QuadTree::from((lu, i-1)),
                QuadTree::from((ru, i-1)),
                QuadTree::from((ld, i-1)),
                QuadTree::from((rd, i-1)),
            ]))
        }
    }
}

fn main() {
    let src = ImageReader::open("CuteCat.jpg").unwrap().decode().unwrap().to_rgb8();
    let mut img = RgbImage::new(src.width(), src.height());

    let tree = QuadTree::from((src, 5));
    tree.draw_full(&mut img);

    img.save("img.png").unwrap();
}

#[allow(unused)]
fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}
