pub struct CPU {
    pub instruction_pointer: usize,
    pub code: Vec<u8>,
    pub stack: Vec<u8>,

    pub rax: i64,
    pub rbx: i64,
    pub rcx: i64,
    pub rdx: i64,
    pub rsp: i64,
    pub rbp: i64,
    pub rsi: i64,
    pub rdi: i64,

    pub r8: i64,
    pub r9: i64,
    pub r10: i64,
    pub r11: i64,
    pub r12: i64,
    pub r13: i64,
    pub r14: i64,
    pub r15: i64,

    pub rflags: u64,
}

impl CPU {
    pub fn new(code: Vec<u8>) -> CPU {
        let stack = vec![0; 8192];
        CPU {
            instruction_pointer: 0,
            rax: 0,
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsp: stack.len() as i64,
            rbp: 0,
            rsi: 0,
            rdi: 0,

            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,

            rflags: 0,

            stack: stack,
            code: code,
        }
    }
}
