# Iterator Sample

## Step 1: Vec over Vec

```rs
struct Windows3 {
    data: Vec<i32>, // underlying data to iterate over
    pos: usize,     // current position of the iterator in the data (=start of sliding window)
}

impl Windows3 {
    fn new(data: Vec<i32>) -> Self {
        Windows3 { data, pos: 0 }
    }
}

impl Iterator for Windows3 {
    type Item = Vec<i32>; // We produce a sliding window of 3 elements as a vector
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + 3 <= self.data.len() {
            let window = self.data[self.pos..self.pos + 3].to_vec(); // Get the current window
            self.pos += 1; // Move to the next position for the next call
            Some(window)
        } else {
            None // No more windows available
        }
    }
}
```

## Step 2: Use Slice instead of Vec

```rs
struct Windows3<'a> {
    data: &'a [i32], // underlying data to iterate over
    pos: usize,     // current position of the iterator in the data (=start of sliding window)
}

impl<'a> Windows3<'a> {
    fn new(data: &'a [i32]) -> Self {
        Windows3 { data, pos: 0 }
    }
}

impl Iterator for Windows3<'_> {
    type Item = Vec<i32>; // We produce a sliding window of 3 elements as a vector
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + 3 <= self.data.len() {
            let window = self.data[self.pos..self.pos + 3].to_vec(); // Get the current window
            self.pos += 1; // Move to the next position for the next call
            Some(window)
        } else {
            None // No more windows available
        }
    }
}
```

## Step 3: Const Generics mit Array

```rs
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
```

## Aufgabe für Profis (und solche, die es werden wollen)

* Implementierung so ändern, dass der Ausgangspunkt keine Slice sein muss, sondern ein beliebiger Iterator über i32 (z.B. Sliding Window über Linked List)
* Implementierung so ändern, dass der Algorithmus für alle Datentypen funktioniert, die Addition und Division unterstützen (z.B. für das Bilden des Durchschnitts; i32, f32, etc.)
