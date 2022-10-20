type Bool = bool;
type Integer = i64;
type Float = f64;
type String = String;
type Array = Vec<Type>;
pub struct Dict{

}
pub struct Function{

}
pub struct Struct{

}
///registers:
/// sum


///caller scope:
/// push ret
/// push value1
/// push value2;
/// call function Add
/// pop value1
/// pop value2
///

///callee scope:
/// push temp
/// add value1 value2
/// move temp sum
///
///


pub enum VMInstruction{
    Push,
    Pop,
    PushN(usize),
    PopN(usize),
    Add(Value,Value),
    Sub(Value,Value),
    Mult(Value,Value),
    Div(Value,Value),
    Mod(Value,Value),
    Fact(Value,Value),
    EQ

}



pub struct BlockSegment{

}


pub enum Value {
    Bool(Bool),
    Integer(Integer),
    Float(Float),
    String(String),
    Array(Array),
    Dict(Dict),
    Function(Function),
    Struct(Struct),
    Nil,
}