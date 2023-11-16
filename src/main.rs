#[derive(Debug)]
struct Pixel {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Field {
    width: usize,
    height: usize,
    landscape: Vec<Pixel>,
    figure: Vec<Pixel>,
}

fn main() {
    let input = r"15 4
    .p..
    .pp.
    ..p.
    ....
    ....
    ....
    ....
    ....
    ....
    ....
    #...
    ...#
    #..#
    ##.#
    ####";

    let mut field = parse_into_field(input);

    loop {
        // clear the screen
        let can_move = field.tick();
        println!("{}", field.to_string());
        std::thread::sleep(std::time::Duration::from_millis(8000));

        if !can_move { 
            println!("\nThe end!");
            break;
         }
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn parse_into_field(string: &str) -> Field {
    let lines: Vec<&str> = string.split('\n').collect();

    let dimensions: Vec<&str> = lines[0].split(' ').collect();
    let height: usize = dimensions[0].parse().unwrap();
    let width: usize = dimensions[1].parse().unwrap();

    let mut field = Field::new(width, height);

    if lines.len() != height + 1 {
        panic!("Specified height and actual don't correlate")
    }

    for y in 0..height {
        // field starts from 2nd line
        let line = lines[y + 1].trim();

        if line.len() != width {
            panic!("One of the lines' width is different with width");
        }

        for x in 0..width {
            let c = line.as_bytes()[x];

            let pixel = Pixel { x, y };

            match c {
                b'#' => field.landscape.push(pixel),
                b'p' => field.figure.push(pixel),
                _ => {}
            };
        }
    }

    field
}

impl Pixel {}

impl Field {
    fn new(width: usize, height: usize) -> Self {
        Field {
            width,
            height,
            landscape: vec![],
            figure: vec![],
        }
    }

    fn get(&self, x: usize, y: usize) -> char {
        for pixel in self.figure.iter() {
            if pixel.x == x && pixel.y == y {
                return 'p';
            }
        }

        for pixel in self.landscape.iter() {
            if pixel.x == x && pixel.y == y {
                return '#';
            }
        }

        '.'
    }

    fn to_string(&self) -> String {
        let mut field_string = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                field_string += &self.get(x, y).to_string();
            }
            field_string += "\n";
        }
        field_string
    }

    fn tick(&mut self) -> bool {
        let mut can_move = true;

        for figure_pixel in self.figure.iter() {
            let new_y = figure_pixel.y + 1;

            if new_y == self.height {
                can_move = false;
                break;
            }

            if self.get(figure_pixel.x, new_y) == '#' {
                can_move = false;
                break;
            }
        }

        if can_move {
            for figure_pixel in self.figure.iter_mut() {
                figure_pixel.y += 1;
            }
        }

        can_move
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_into_field;

    #[test]
    fn test_parse_into_field() {
        let field = parse_into_field(
            r"2 4
    p...
    #...",
        );
        assert_eq!(field.get(0, 0), 'p');
        assert_eq!(field.get(0, 1), '#');
    }

    #[test]
    #[should_panic(expected = "One of the lines' width is different with width")]
    fn test_parse_into_field_shorter_line() {
        let _ = parse_into_field(
            r"2 4
    p...
    #..#.",
        );
    }

    #[test]
    #[should_panic(expected = "One of the lines' width is different with width")]
    fn test_parse_into_field_longer_line() {
        let _ = parse_into_field(
            r"2 4
    p.....
    #..#d",
        );
    }

    #[test]
    #[should_panic(expected = "Specified height and actual don't correlate")]
    fn test_parse_into_field_more_lines() {
        let _ = parse_into_field(
            r"2 4
    p...
    #..#d
    #####",
        );
    }

    #[test]
    #[should_panic(expected = "Specified height and actual don't correlate")]
    fn test_parse_into_field_less_lines() {
        let _ = parse_into_field(
            r"3 4
    p...
    #..#d",
        );
    }

    #[test]
    fn test_tick() {
        // Given
        let mut field = parse_into_field(
            r"4 4
        .pp.
        pppp
        .pp.
        ....",
        );

        // When
        field.tick();

        // Then
        assert_eq!(
            field.to_string(),
            r"....
.pp.
pppp
.pp.
"
        )
    }
}
