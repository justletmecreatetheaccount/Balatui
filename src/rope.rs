// Implementation of the "Rope" data structure for the text editing part
// of balatui. The implementation is based on https://en.wikipedia.org/wiki/Rope_(data_structure)

use std::fmt;

#[derive(Debug, Clone)]
struct BadPath;

impl fmt::Display for BadPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The path given cannot be followed")
    }
}

#[derive(PartialEq, Clone, Debug)]
enum Directions {
    //Help with new rope creation
    RIGHT,
    LEFT,
}
impl fmt::Display for Directions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &Directions::RIGHT {
            write!(f, "RIGHT")
        } else {
            write!(f, "LEFT")
        }
    }
}
struct Rope {
    head: Option<Box<RopeNode>>,
    path_to_last: Vec<Directions>,
}

struct RopeNode {
    left_node: Option<Box<RopeNode>>,
    right_node: Option<Box<RopeNode>>,
    layer: u32,
    weight: u32,             //lenght of the full string under the left node
    right_weight: u32,       //lenght of the full string under the right node
    content: Option<String>, //Contained string
}

impl Rope {
    //search for the string bit that contains the character at the index and
    //the index of said character in the string
    pub fn search(&self, index: u32) -> Option<(&String, u16)> {
        let mut index = index + 1; //to keep the index on a 0.. scale
        let mut current_node = &self.head;
        let mut prev_node = &self.head;
        while current_node.is_some() {
            if index > current_node.as_ref().unwrap().weight {
                index -= current_node.as_ref().unwrap().weight;
                prev_node = &current_node;
                current_node = &current_node.as_ref().unwrap().right_node;
            } else {
                prev_node = &current_node;
                current_node = &current_node.as_ref().unwrap().left_node;
            }
        }
        if index > prev_node.as_ref().unwrap().weight {
            None
        } else {
            Some((
                prev_node.as_ref().unwrap().content.as_ref().unwrap(),
                index as u16,
            ))
        }
    }

    fn follow_path_from<'a>(
        head: &'a mut Box<RopeNode>,
        path: &[Directions],
    ) -> Result<&'a mut Box<RopeNode>, BadPath> {
        let mut current_node = head;
        for direction in path {
            if direction == &Directions::LEFT {
                if let Some(ref mut inner_current_node) = current_node.left_node {
                    current_node = inner_current_node;
                } else {
                    return Err(BadPath);
                }
            } else {
                if let Some(ref mut inner_current_node) = current_node.right_node {
                    current_node = inner_current_node;
                } else {
                    return Err(BadPath);
                }
            }
        }
        Ok(current_node)
    }

    fn follow_path(&mut self, path: &[Directions]) -> Result<&mut Box<RopeNode>, BadPath> {
        if let Some(ref mut head_node) = self.head {
            let mut current_node = head_node;
            for direction in path {
                if direction == &Directions::LEFT {
                    if let Some(ref mut inner_current_node) = current_node.left_node {
                        current_node = inner_current_node;
                    } else {
                        return Err(BadPath);
                    }
                } else {
                    if let Some(ref mut inner_current_node) = current_node.right_node {
                        current_node = inner_current_node;
                    } else {
                        return Err(BadPath);
                    }
                }
            }
            Ok(current_node)
        } else {
            Err(BadPath)
        }
    }

    fn follow_path_and_add_weights(
        &mut self,
        path: &[Directions],
        weight: u32,
    ) -> Result<&mut Box<RopeNode>, BadPath> {
        if let Some(ref mut head_node) = self.head {
            let mut current_node = head_node;
            for direction in path {
                if direction == &Directions::LEFT {
                    if let Some(ref mut inner_current_node) = current_node.left_node {
                        current_node.weight += weight;
                        current_node = inner_current_node;
                    } else {
                        return Err(BadPath);
                    }
                } else {
                    if let Some(ref mut inner_current_node) = current_node.right_node {
                        current_node.right_weight += weight;
                        current_node = inner_current_node;
                    } else {
                        return Err(BadPath);
                    }
                }
            }
            Ok(current_node)
        } else {
            Err(BadPath)
        }
    }

    //split a string in a new rope
    pub fn string_to_rope(mut input: String, leaf_weight: u8) -> Rope {
        let mut new_rope = Rope {
            head: None,
            path_to_last: Vec::new(),
        };
        //Spilt the string bit by bit and put the bits in a node
        //Then put the node in the rope
        while input.len() > leaf_weight as usize {
            //Creation of the new node
            //Inversion of result/self for the function "split_off"
            let temp = input.split_off(leaf_weight as usize);
            let temp_content = input;
            input = temp;
            new_rope.elongate_rope(temp_content);
        }
        new_rope.elongate_rope(input);
        new_rope
    }

    //assing new rope head
    pub fn assing_head(&mut self, new_head: Box<RopeNode>) {
        self.head = Some(new_head);
    }

    //TODO add the  weights
    //scale the rope by adding the passed string at the end as a node
    pub fn elongate_rope(&mut self, input: String) {
        let new_node = Some(Box::new(RopeNode {
            left_node: None,
            right_node: None,
            layer: 0,
            weight: input.len() as u32,
            right_weight: 0,
            content: Some(input),
        }));
        let new_node_weight = new_node.as_ref().unwrap().weight;

        let mut new_path: Vec<Directions> = Vec::new();

        //Meddling with the rope
        if let Some(mut old_head) = self.head.take() {
            //Check if rope is empty
            for index in (0..self.path_to_last.len()).rev() {
                if self.path_to_last[index] == Directions::LEFT {
                    self.head = Some(old_head);
                    let slice = &self.path_to_last.clone()[0..index];
                    let junction_node = self
                        .follow_path_and_add_weights(slice, new_node_weight)
                        .unwrap();
                    junction_node.right_weight += new_node_weight;
                    new_path = slice.to_vec();
                    if junction_node.layer != 1 {
                        //Check if node to be added to the right is the node with content
                        junction_node.right_node = Some(Box::new(RopeNode::new_empty(
                            new_node_weight,
                            0,
                            junction_node.layer - 1,
                        )));
                        new_path.push(Directions::RIGHT);
                        let mut current_node = junction_node.right_node.as_mut().unwrap();
                        while current_node.layer != 1 {
                            current_node.left_node = Some(Box::new(RopeNode::new_empty(
                                new_node_weight,
                                0,
                                current_node.layer - 1,
                            )));
                            current_node = current_node.left_node.as_mut().unwrap();
                            new_path.push(Directions::LEFT);
                        }
                        current_node.left_node = new_node;
                        new_path.push(Directions::LEFT);
                        self.path_to_last = new_path;
                        return;
                    } else {
                        junction_node.right_node = new_node;
                        new_path.push(Directions::RIGHT);
                        self.path_to_last = new_path;
                        return;
                    }
                }
            } //If here then last path is full of rights (or nothing) and need a new head
              //Creating the new head
            self.head = Some(Box::new(RopeNode::new_empty(
                old_head.weight + old_head.right_weight,
                new_node_weight,
                old_head.layer + 1,
            )));
            //putting the old head with it's tree at the left of the new one
            self.head.as_mut().unwrap().left_node = Some(old_head);
            //check if the new head is not on level 1
            if self.head.as_ref().unwrap().layer != 1 {
                let mut current_node = self.head.as_mut().unwrap();
                current_node.right_node = Some(Box::new(RopeNode::new_empty(
                    new_node_weight,
                    0,
                    current_node.layer - 1,
                )));
                new_path.push(Directions::RIGHT);
                current_node = current_node.right_node.as_mut().unwrap();
                while current_node.layer != 1 {
                    current_node.left_node = Some(Box::new(RopeNode::new_empty(
                        new_node_weight,
                        0,
                        current_node.layer - 1,
                    )));
                    new_path.push(Directions::LEFT);
                    current_node = current_node.left_node.as_mut().unwrap();
                }
                current_node.left_node = new_node;
                new_path.push(Directions::LEFT);
            } else {
                self.head.as_mut().unwrap().right_node = new_node;
                new_path.push(Directions::RIGHT);
            }
        } else {
            self.head = new_node
        }
        self.path_to_last = new_path;
    }
}

impl RopeNode {
    pub fn new_empty(n_weight: u32, n_right_weight: u32, n_layer: u32) -> RopeNode {
        let node: RopeNode;
        node = Self {
            left_node: None,
            right_node: None,
            layer: n_layer,
            weight: n_weight,
            right_weight: n_right_weight,
            content: None,
        };
        node
    }

    pub fn new_with_content(string: String) -> RopeNode {
        let node: RopeNode;
        node = Self {
            left_node: None,
            right_node: None,
            layer: 0,
            weight: string.len() as u32,
            right_weight: 0,
            content: Some(string),
        };
        node
    }

    pub fn fill_content(&mut self, content: String) {
        self.content = Some(content);
    }
    pub fn empty_content(&mut self) {
        self.content = None;
    }
    pub fn assign_left_node(&mut self, node: Box<RopeNode>) {
        self.left_node = Some(node);
    }
    pub fn empty_left_node(&mut self) {
        self.left_node = None;
    }
    pub fn assign_right_node(&mut self, node: Box<RopeNode>) {
        self.right_node = Some(node);
    }
    pub fn empty_right_node(&mut self) {
        self.right_node = None;
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    #[test]
    //   o
    //  / \
    // o   o
    fn basic_creation() {
        let rope = Rope::string_to_rope(String::from_str("Hello World!").unwrap(), 12);
        assert_eq!(
            rope.head.as_ref().unwrap().content.as_ref().unwrap(),
            "Hello World!"
        );
    }
    #[test]
    fn three_nodes_rope() {
        let rope = Rope::string_to_rope(String::from_str("Hello World!").unwrap(), 6);
        assert_eq!(rope.head.as_ref().unwrap().content, None);
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "Hello "
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "World!"
        );
    }
    #[test]
    //      o
    //    _/ \_
    //   o     o
    //  / \   /
    // o   o o
    fn six_nodes_rope() {
        let rope = Rope::string_to_rope(String::from_str("Hello World!").unwrap(), 4);
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "rld!"
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "o Wo"
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "Hell"
        );
    }
    #[test]
    //      o
    //    _/ \_
    //   o     o
    //  / \   / \
    // o   o o   o
    fn seven_nodes_rope() {
        let rope = Rope::string_to_rope(String::from_str("Hello World!").unwrap(), 3);
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "ld!"
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "Wor"
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "lo "
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "Hel"
        );
    }
    #[test]
    //         ____o____
    //        /         \
    //      o            o
    //    _/ \_        _/
    //   o     o      o
    //  / \   / \    /
    // o   o o   o  o
    fn eleven_nodes_rope() {
        let rope = Rope::string_to_rope(String::from_str("HelloWorld").unwrap(), 2);
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "ld"
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "or"
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "oW"
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "ll"
        );
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .content
                .as_ref()
                .unwrap(),
            "He"
        );
    }

    #[test]
    fn basic_weight() {
        let rope = Rope::string_to_rope(String::from_str("Hello World!").unwrap(), 12);
        assert_eq!(rope.head.as_ref().unwrap().weight, 12);
    }

    #[test]
    fn right_weight() {
        let rope = Rope::string_to_rope(String::from_str("Hello World!").unwrap(), 6);
        assert_eq!(rope.head.as_ref().unwrap().right_weight, 6);
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .right_node
                .as_ref()
                .unwrap()
                .weight,
            6
        );
    }

    #[test]
    fn complex_weight() {
        let rope = Rope::string_to_rope(String::from_str("HelloWorld").unwrap(), 2);
        assert_eq!(
            rope.head
                .as_ref()
                .unwrap()
                .left_node
                .as_ref()
                .unwrap()
                .right_weight,
            4
        );

        assert_eq!(rope.head.as_ref().unwrap().right_weight, 2);

        assert_eq!(rope.head.as_ref().unwrap().weight, 8);
    }

    #[test]
    fn search() {
        let rope = Rope::string_to_rope(String::from_str("HelloWorld").unwrap(), 2);
        let ret = rope.search(3);
        assert_eq!(ret.unwrap().0, "ll");
        assert_eq!(ret.unwrap().1, 2);
    }
}
