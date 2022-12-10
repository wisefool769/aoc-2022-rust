use aoc_2022::read_input;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq)]
struct TreeNode {
  pub name: String,
  pub size: Option<u32>,
  pub is_directory: bool,
  pub children: Vec<Rc<RefCell<TreeNode>>>,
  pub parent: Option<Rc<RefCell<TreeNode>>>,
}

struct TreeNodeCtor {
    name: String,
    size: Option<u32>,
    is_directory: bool,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  pub fn new(ctor: TreeNodeCtor) -> TreeNode {
    return TreeNode {
      name: ctor.name,
      size: ctor.size,
      is_directory: ctor.is_directory,
      children: vec![],
      parent: ctor.parent,
    };
  }

  pub fn add_child(&mut self, new_node: Rc<RefCell<TreeNode>>) {
    self.children.push(new_node);
  }

  pub fn compute_size(&mut self) -> u32 {
    if !self.is_directory {
        return self.size.unwrap();
    }

    let mut size = 0;
    for child in self.children.iter() {
        size += child.borrow_mut().compute_size();
    }

    self.size = Some(size);
    return size;
  }

  pub fn print(&self, indentation_level: u32) {
    let mut indent = String::new();
    for _ in 0..indentation_level {
      indent.push_str(" ");
    }
    println!("{}{}: {}", indent, self.name, self.size.unwrap());
    for child in self.children.iter() {
      child.borrow().print(indentation_level + 1);
    }
  }
}

// returns a list of all directory sizes given a node
fn get_directory_sizes(node: &Rc<RefCell<TreeNode>>) -> Vec<u32> {
    let mut sizes = vec![];
    if !node.borrow().is_directory {
        return sizes;
    }
    for child in node.borrow().children.iter() {
        sizes.append(&mut get_directory_sizes(&child));
    }
    let size = node.borrow().size.unwrap();
    sizes.push(size);
    return sizes;
}

fn main() {
    let input = read_input(7);
    let root = Rc::new(RefCell::new(TreeNode::new(
        TreeNodeCtor { name: "/".to_owned(), size: None, is_directory: true, parent: None })));
    let mut current = Rc::clone(&root);

    // build up file system
    for line in input.split("\n") {
        println!("{}", line);
        if line == "$ cd .." {
            let parent = current.borrow().parent.as_ref().unwrap().clone();
            current = parent;
        } else if  line.starts_with("$ cd") {
            let dir_name = line.split(" ").last().unwrap();
            for child in current.clone().borrow().children.iter() {
                if child.borrow().name == dir_name{
                    current = child.clone();
                }
            }
        } else if line == "$ ls" {
            continue;
        } else if line.starts_with("dir") {
            let dir_name = line.split(" ").last().unwrap().to_owned();
            let ctor = TreeNodeCtor { name: dir_name, size: None, is_directory: true, parent: Some(current.clone())};
            current.borrow_mut().add_child(Rc::new(RefCell::new(TreeNode::new(ctor))));
        } else {
            let size = line.split(" ").next().unwrap().parse::<u32>().unwrap();
            let file_name = line.split(" ").last().unwrap().to_owned();
            let ctor = TreeNodeCtor { name: file_name, size: Some(size), is_directory: false, parent: Some(current.clone())};
            current.borrow_mut().add_child(Rc::new(RefCell::new(TreeNode::new(ctor))));
        }
    }

    // compute directory sizes
    root.borrow_mut().compute_size();
    root.borrow().print(0);
    let dir_sizes: Vec<u32> = get_directory_sizes(&root);
    let threshold = 100000;
    let answer1: u32 = dir_sizes.iter().filter(|s| s <= &&threshold).sum();
    println!("pt 1: {}", answer1);
    let capacity = 70000000;
    let needed = 30000000;
    let used = root.borrow().size.unwrap();
    let free = capacity - used;
    let to_delete = needed - free;
    println!("capacity: {}", capacity);
    println!("needed: {}", needed);
    println!("used: {}", used);
    println!("free: {}", free);
    println!("to_delete: {}", to_delete);
    let answer2 = itertools::sorted(dir_sizes.iter().filter(|s| s >= &&to_delete)).next().unwrap();
    println!("pt 2 answer: {}", answer2);
}
