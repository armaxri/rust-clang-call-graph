pub mod argument_splitter;
pub mod clang_compile2ast_call;
pub mod dummy_process;
pub mod terminal_process;

pub trait Process {
    fn process(&mut self) -> bool;

    fn has_next_line(&self) -> bool;
    fn fetch_next_line(&self) -> String;
    fn get_next_line(&mut self) -> String;
}
