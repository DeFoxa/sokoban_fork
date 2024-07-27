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
