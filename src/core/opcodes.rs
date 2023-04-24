#[derive(Debug, Clone)]
pub enum OpCode {
    PushInt(i64),
    PushFloat(f64),
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    Pow,
    Xor,
    Or,
    Not,
    And,
    Neg,
    LNot,
    Shl,
    Shr,

    Br(u32),
    BrT(u32),
    BrF(u32),

    CmpEq,
    CmpNEq,
    CmpLt,
    CmpGt,
    CmpLtEq,
    CmpGtEq,

    Call(u32),

    VirtualCall(u32),
    ConstructorCall(u32),

    ClosureCall,

    CreateClosure(u32), // ty
    Return,

    LoadLocal(u32),
    StoreLocal(u32),

    LoadGlobal(u32),
    StoreGlobal(u32),

    CreateObject(u32),
    DestroyObject(u32),

    StoreField(u32, u32), // type index
    LoadField(u32, u32),  // type index

    LoadType(u32),

    CreateArray(u32, u32), // type size
    DestroyArray(u32, u32),

    ArrayGet(u32, u32),
    ArraySet(u32, u32),
}
