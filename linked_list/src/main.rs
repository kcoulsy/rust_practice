fn main() {
    println!("Hello, world!");

    let mut list = LinkedList::new();

    list.insert_beginning(1);
    let first = list.pop();
    println!("first is {first:?}");

    list.insert_beginning(1);
    list.insert_beginning(2);
    list.insert_end(3);
    let last = list.shift();
    println!("last is {last:?}");
    list.insert_end(2);

    list.print();
    let position = list.find_first(2);
    println!("index of 2 is {position}");
    let length = list.get_length();
    println!("length of list is {length}");

    let position = list.find_first(4);
    println!("index of 4 is {position}");

    let mut new_list = LinkedList::new();
    for i in 0..10 {
        new_list.insert_end(i);
    }

    new_list.print();

    let mut new_random_list = LinkedList::new();

    while new_list.get_length() > 0 {
        let should_delete = rand::random_bool(0.5);
        let data = new_list.pop().unwrap();

        if should_delete {
            new_random_list.insert_end(data);
        } else {
            new_list.insert_end(data);
        }
    }

    new_random_list.print();
}

#[derive(Debug)]
struct Node<T: PartialEq> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T: PartialEq> {
    head: Option<Box<Node<T>>>,
}

impl<T: PartialEq> LinkedList<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn insert_beginning(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }

    fn insert_end(&mut self, data: T) {
        let new_node = Box::new(Node { data, next: None });

        // if the list is empty, setting this new node as the head
        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            // list is not empty, lets find the last node by traversing the list
            let mut current = &mut self.head;
            while let Some(node) = current {
                current = &mut node.next
            }
            // on that last node, we set next to the new node
            // at this point we want to dereference it and update that
            // point in memory to the new node

            *current = Some(new_node);
        }
    }

    fn find_first(&self, data: T) -> i32 {
        let mut current = &self.head;
        let mut position = 0;

        while let Some(node) = current {
            if node.data == data {
                return position;
            }
            current = &node.next;
            position += 1;
        }

        return -1;
    }

    fn get_first(&self) -> Option<&T> {
        let current = &self.head;
        if current.is_none() {
            return None;
        }
        return Some(&current.as_ref().unwrap().data);
    }

    fn get_last(&self) -> Option<&T> {
        let mut current = &self.head;
        if current.is_none() {
            return None;
        }
        while let Some(node) = current {
            if node.next.is_none() {
                return Some(&node.data);
            }
            current = &node.next;
        }
        return Some(&current.as_ref().unwrap().data);
    }

    fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
    }

    fn shift(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if self.head.as_ref().unwrap().next.is_none() {
            return self.pop();
        }

        // Find second-to-last node
        let mut current = &mut self.head;
        while let Some(current_node) = current {
            let next_node = current_node.next.as_ref();

            if next_node.as_ref().unwrap().next.is_none() {
                // current_node is the second-to-last node as next_node doesn't have next
                let first_node = current_node.next.take();
                return Some(first_node.unwrap().data); // return the data of the last node
            }
            current = &mut current_node.next;
        }
        None
    }

    fn delete(&mut self, data: T) {
        let mut current = &mut self.head;

        if current.is_none() {
            return;
        }

        // handleing deleting the first node
        if current.as_ref().unwrap().data == data {
            self.head = current.as_mut().unwrap().next.take();

            return;
        }

        // find the node to delete
        while let Some(node) = current {
            if node.data == data {
                let next_node = node.next.as_mut();
                if next_node.is_some() {
                    node.next = next_node.unwrap().next.take();
                }
                return;
            }
            current = &mut node.next;
        }

        return;
    }

    fn get_length(&self) -> u32 {
        let mut current = &self.head;
        let mut length = 0;
        while let Some(node) = current {
            length += 1;
            current = &node.next;
        }
        length
    }

    fn print(&self)
    where
        T: std::fmt::Display,
    {
        let mut current = &self.head;
        print!("List: [");
        while let Some(node) = current {
            let is_last = node.next.is_none();
            if is_last {
                print!("{}", node.data);
            } else {
                print!("{}, ", node.data);
            }
            current = &node.next;
        }
        println!("]");
    }

    // reverses in place, basically swaps the direction of the next links between nodes
    fn reverse(&mut self) {
        let mut previous = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next_node = node.next.take();
            node.next = previous;
            previous = Some(node);
            current = next_node;
        }
        self.head = previous;
    }
}
