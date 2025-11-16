use crate::bytecode::Value;
use super::VM;

impl VM {
    pub(crate) fn peek(&self, distance: usize) -> Option<&Value> {
        if distance >= self.stack.len() {
            None
        } else {
            Some(&self.stack[self.stack.len() - 1 - distance])
        }
    }
    
    pub(crate) fn stack_len(&self) -> usize {
        self.stack.len()
    }
}
