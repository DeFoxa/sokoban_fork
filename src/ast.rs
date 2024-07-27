use crate::{node_allocator::NodeAllocator, ZeroCopy, SENTINEL};
use bytemuck::{Pod, Zeroable};

// const MAX_SIZE = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ASTNode<T: Copy + Clone + Pod + Zeroable + Default> {
    data: T,
    l: u32,
    r: u32,
}

impl<T: Copy + Clone + Pod + Zeroable + Default> ASTNode<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            l: SENTINEL,
            r: SENTINEL,
        }
    }
}

impl<T: Copy + Clone + Pod + Zeroable + Default> Default for ASTNode<T> {
    fn default() -> Self {
        Self {
            data: T::default(),
            l: SENTINEL,
            r: SENTINEL,
        }
    }
}

unsafe impl<T: Copy + Clone + Pod + Zeroable + Default> Zeroable for ASTNode<T> {}

unsafe impl<T: Copy + Clone + Pod + Zeroable + Default> Pod for ASTNode<T> {}

impl<T: Copy + Clone + Pod + Zeroable + Default> ZeroCopy for ASTNode<T> {}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AST<T: Copy + Clone + Pod + Default + Zeroable, const MAX_SIZE: usize> {
    root: u32,
    allocator: NodeAllocator<ASTNode<T>, MAX_SIZE, 2>,
}
impl<T: Copy + Clone + Pod + Default + Zeroable, const MAX_SIZE: usize> AST<T, MAX_SIZE> {
    pub fn new() -> Self {
        Self {
            root: SENTINEL,
            allocator: NodeAllocator::new(),
        }
    }
    pub fn add_node(&mut self, data: T) -> u32 {
        let new_node = self.allocator.add_node(ASTNode::new(data));
        if self.root == SENTINEL {
            self.root = new_node;
        }

        new_node
    }

    pub fn set_left_child(&mut self, parent: u32, child: u32) {
        let node = self.allocator.get_mut(parent).get_value_mut();
        node.l = child;
    }

    pub fn set_right_child(&mut self, parent: u32, child: u32) {
        let node = self.allocator.get_mut(parent).get_value_mut();
        node.r = child;
    }

    pub fn get_node(&self, node: u32) -> Option<&T> {
        if node == SENTINEL {
            None
        } else {
            Some(&self.allocator.get(node).get_value().data)
        }
    }

    pub fn get_node_mut(&mut self, node: u32) -> Option<&mut T> {
        if node == SENTINEL {
            None
        } else {
            Some(&mut self.allocator.get_mut(node).get_value_mut().data)
        }
    }

    pub fn get_left_child(&self, node: u32) -> Option<u32> {
        let child = self.allocator.get(node).get_value().l;
        if child == SENTINEL {
            None
        } else {
            Some(child)
        }
    }

    pub fn get_right_child(&self, node: u32) -> Option<u32> {
        let child = self.allocator.get(node).get_value().r;
        if child == SENTINEL {
            None
        } else {
            Some(child)
        }
    }

    pub fn get_root(&self) -> Option<u32> {
        if self.root == SENTINEL {
            None
        } else {
            Some(self.root)
        }
    }

    pub fn len(&self) -> usize {
        self.allocator.size as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        MAX_SIZE
    }
}

unsafe impl<T: Copy + Clone + Pod + Default + Zeroable, const MAX_SIZE: usize> Zeroable
    for AST<T, MAX_SIZE>
{
}
unsafe impl<T: Copy + Clone + Pod + Default + Zeroable, const MAX_SIZE: usize> Pod
    for AST<T, MAX_SIZE>
{
}
impl<T: Copy + Clone + Pod + Default + Zeroable, const MAX_SIZE: usize> ZeroCopy
    for AST<T, MAX_SIZE>
{
}
