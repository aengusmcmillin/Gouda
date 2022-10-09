use crate::Image;

pub struct Spritesheet {
    rows: usize,
    columns: usize,
    image: Image,
}

impl Spritesheet {
    pub fn new(rows: usize, columns: usize, image: Image) -> Spritesheet {
        Spritesheet {
            rows,
            columns,
            image,
        }
    }

    pub fn sprite(&self, x: usize, y: usize) -> Image {
        let width = self.image.width;
        let height = self.image.height;

        let sprite_width = width / self.columns;
        let sprite_height = height / self.rows;

        let mut data = vec![];
        for y in (sprite_height * y)..(sprite_height * y + sprite_height) {
            for x in (sprite_width * x)..(sprite_width * x + sprite_width) {
                data.push(self.image.data[width * y + x]);
            }
        }

        Image {
            width: sprite_width,
            height: sprite_height,
            data,
        }
    }
}
