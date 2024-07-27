use bytemuck::{Pod, Zeroable};
use NodeAllocator::NodeAllocator;

type ASTAllocator = NodeAllocator<ASTNode<T>, MAX_SIZE, 2>;
const MAX_SIZE = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ASTNode<T: Copy + Clone + Pod + Zeroable> {
    data: T,
    l: u32,
    r: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AST<T: Copy + Clone + Pod + Zeroable, const MAX_SIZE: usize> {
    root: u32,
    allocator: ASTAllocator,
}
