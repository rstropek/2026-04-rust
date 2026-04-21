struct Windows<'a, const N: usize> {
    data: &'a [i32], // underlying data to iterate over
    pos: usize,     // current position of the iterator in the data (=start of sliding window)
}

impl<'a, const N: usize> Windows<'a, N> {
    fn new(data: &'a [i32]) -> Self {
        Self { data, pos: 0 }
    }
}

impl<const N: usize> Iterator for Windows<'_, N> {
    type Item = [i32; N]; // We produce a sliding window of 3 elements as an array
    
    fn next(&mut self) -> Option<Self::Item> {
        let slice = self.data.get(self.pos..self.pos + N)?;
        let window: [i32; N] = slice.try_into().unwrap();
        self.pos += 1;
        Some(window)
    }
}

fn main() {
    {
        let numbers = [1i64, 2, 3, 4, 5];

        for number in numbers
            .iter()
            .skip(2)
            .take(3)
            .filter(|&&n| n % 2 == 0)
            .rev()
        {
            println!("{}", number);
        }
    }

    {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let windows = Windows::<4>::new(&data);

        for window in windows {
            dbg!(&window);
        }
    }

    {
        let numbers = [1, 2, 3, 4, 5];
        let window = &numbers[1..=4]; // Slice
        for number in window.iter() {
            println!("{}", number);
        }
    }

    {
        // Lifetimes
        #[derive(Debug)] struct Vector2d { x: f32, y: f32 }
        let v1 = Vector2d { x: 1.0, y: 2.0 };
        let v2 = Vector2d { x: 3.0, y: 40.0 };
        
        fn get_longest<'a>(v1: &'a Vector2d, v2: &'a Vector2d) -> &'a Vector2d {
            let len1 = v1.x * v1.x + v1.y * v1.y;
            let len2 = v2.x * v2.x + v2.y * v2.y;
            if len1 > len2 { v1 } else { v2 }
        }
        
        let longest = get_longest(&v1, &v2);
        dbg!(longest);
    }
}