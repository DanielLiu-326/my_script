type OpReg             = u8;    //寄存器编号类型
type ConstAddr         = u16;   //常量指针类型
type RelativeAddr      = u16;   //相对地址类型
pub struct SegAddr(u8,u8,u8);   //代码段指针类型

#[derive(Debug)]
#[repr(u8)]
pub enum OpCode {
    /// 运算指令(结果,左操作数,右操作数)
    Or(OpReg,OpReg,OpReg),
    And(OpReg,OpReg,OpReg),
    BitOr(OpReg,OpReg,OpReg),
    BitXor(OpReg,OpReg,OpReg),
    BitAnd(OpReg,OpReg,OpReg),
    NE(OpReg,OpReg,OpReg),
    EQ(OpReg,OpReg,OpReg),
    RefEQ(OpReg,OpReg,OpReg),
    RefNE(OpReg,OpReg,OpReg),
    LT(OpReg,OpReg,OpReg),
    GT(OpReg,OpReg,OpReg),
    LE(OpReg,OpReg,OpReg),
    GE(OpReg,OpReg,OpReg),
    // RefLT(OpReg,OpReg,OpReg),
    // RefGT(OpReg,OpReg,OpReg),
    // RefLE(OpReg,OpReg,OpReg),
    // RefGE(OpReg,OpReg,OpReg),
    LMov(OpReg,OpReg,OpReg),
    RMov(OpReg,OpReg,OpReg),
    Add(OpReg,OpReg,OpReg),
    Sub(OpReg,OpReg,OpReg),
    Mul(OpReg,OpReg,OpReg),
    Div(OpReg,OpReg,OpReg),
    Mod(OpReg,OpReg,OpReg),
    Fact(OpReg,OpReg,OpReg),

    BitNot(OpReg,OpReg),
    Not(OpReg,OpReg),
    Neg(OpReg,OpReg),
    Pos(OpReg,OpReg),

    // /// 数组访问操作 postponed
    // ArrayVisit(OpReg,OpReg,OpReg), //(存储结果的寄存器,数组所在的寄存器,index)
    //
    // /// 结构体成员访问操作
    // MemberGet(OpReg,OpReg,OpReg), //(存储结果的寄存器，struct所在寄存器，成员ident字符串常量)
    // MemberSet(OpReg,OpReg,OpReg), //(struct所在的寄存器,成员ident字符串常量,设置的值所在的寄存器)

    /// 赋值操作
    RefAssign(OpReg,OpReg),       //引用赋值
    ValAssign(OpReg,OpReg),       //值赋值

    /// 变量创建


    /// Load constant from constant table
    // LoadAsConst(OpReg, ConstAddr),
    // LoadConst1(OpReg, ConstAddr), //从常量区1加载数据
    // LoadConst2(OpReg, ConstAddr), //从常量区2加载数据
    // LoadConst3(OpReg, ConstAddr), //从常量区3加载数据

    /// Create variable and get it's mutable reference
    LoadAsMutRef(OpReg,ConstAddr),
    /// Create variable and get it's constant reference
    LoadAsConstRef(OpReg,ConstAddr),
    // LoadFromConst1(OpReg,ConstAddr),    //从常量区1加载数据
    // LoadFromConst2(OpReg,ConstAddr),    //从常量区2加载数据
    // LoadFromConst3(OpReg,ConstAddr),    //从常量区3加载数据


    // //创建内嵌式Bool
    // LoadTrue(OpReg),
    // LoadFalse(OpReg),
    //
    // //创建内嵌式整数 -65535~65535
    // LoadPosShort(OpReg,u16),
    // LoadNegShort(OpReg,u16),

    // //创建数组 postponed
    // LoadNewArray(OpReg,u16),	    //存储寄存器，初始大小

    // //创建结构体 //postponed
    // LoadStruct(OpReg),
    // InitMember(OpReg,OpReg,OpReg),

    // //创建闭包，添加捕获变量 postponed
    // CapValue(OpReg,OpReg,OpReg),

    // //创建Nil
    // LoadNil(OpReg),

    //相对跳跃
    JmpPrev(u8,u8,u8),
    JmpPost(u8,u8,u8),
    //判断是否为True，如果为True则跳过下一行代码，如果为False则执行下一行代码
    Chk(u8),

    ///函数调用
    /// - 压入PC寄存器
    /// - 压入基地址寄存器
    /// - 将基地址设置为栈顶-4
    Call(u8),
    // CallConst0(ConstAddr),      //调用Const函数

    Ret,                        //弹出到基地址寄存器，弹出到程序计数

}

impl Clone for OpCode {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for OpCode{}


impl OpCode{

    #[inline(always)]
    pub fn get_u24(&self) ->u32 { unsafe {
        *(self as *const Self).cast::<u32>() & 0x00_ff_ff_ff
    }}

    #[inline(always)]
    pub fn get_uncut(&self) ->u32 {
        unsafe{
            *(self as *const Self).cast()
        }
    }

    #[inline(always)]
    pub fn set_u24(&mut self,val:u32){unsafe{
        *(self as *mut Self).cast::<u32>() |= val;
    }}
}
