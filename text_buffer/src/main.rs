fn main() {
    let mut buffer = TextBuffer::new();

    buffer.insert('w');
    buffer.insert('o');
    buffer.insert('r');
    buffer.insert('l');
    buffer.insert('d');

    buffer.cursor_position = 0;

    buffer.insert('h');
    buffer.insert('e');
    buffer.insert('l');
    buffer.insert('l');
    buffer.insert('o');
    buffer.insert(' ');

    buffer.delete();
    buffer.delete();
    buffer.find_replace('l', 'L');
    buffer.find_replace_iter('d', 'D');
    buffer.capitalize_first();
    buffer.print();

}

struct TextBuffer {
    content: Vec<char>,
    cursor_position: usize,
}

impl TextBuffer {
    fn new() -> Self {
        Self {
            content: Vec::new(),
            cursor_position: 0,
        }
    }

    fn insert(&mut self, data: char) {
        self.content.insert(self.cursor_position, data);
        self.cursor_position += 1;
    }

    // delete the character to left of the cursor
    fn delete(&mut self) {
        if self.cursor_position == 0 {
            // nothing to delete
            return;
        }
        self.cursor_position -= 1;
        self.content.remove(self.cursor_position);
    }

    fn get_text(&self) -> String {
        self.content.iter().collect::<String>()
    }

    fn print(&self) {
        println!("{}", self.get_text());
    }

    fn find_replace(&mut self, find: char, replace: char) {
        for i in 0..self.content.len() {
            if self.content[i] == find {
                self.content[i] = replace;
            }
        }
    }

    // forcing working with the borrow checker by using enumerate
    fn find_replace_iter(&mut self, find: char, replace: char) {
        let mut content = self.content.clone();
        for (index, character) in self.content.iter().enumerate() {
            // enumerate returns a tuple of the index and a reference to the character
            // so we need to dereference the character to get the value
            if *character == find {
                content[index] = replace;
            }
        }
        self.content = content;
    }

    fn capitalize_first(&mut self) {
        let first_char = self.content.get(0);
        if first_char.is_none() {
            return;
        }
        if first_char.unwrap().is_lowercase() {
            self.content[0] = first_char.unwrap().to_uppercase().next().unwrap();
        }
    }

    
}
