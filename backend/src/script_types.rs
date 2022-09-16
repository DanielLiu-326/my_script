use std::collections::HashMap;

pub type ScriptInteger = u64;

pub type ScriptFloat = f64;

pub type ScriptDict<'input> = HashMap<&'input str,Reference<'input>>;


pub enum ScriptString<'input>{
    Owned(String),
    Ref(&'input str)
}


pub struct ScriptArray<'input>{

}

impl ScriptArray{

}

pub struct ScriptFunction<'input>{
    function:ast::Function
}
impl ScriptFunction{

}

pub struct LeftReference<'input,'left>{
    pub reference:&'left RightReference<'input>,
}

pub enum RightReference<'input>{
    String(ScriptString<'input>),
    Integer(ScriptInteger),
    Float(ScriptFloat),
    Array(ScriptArray<'input>),
    Function(ScriptFunction<'input>),
    Dictionary(ScriptDict<'input>)
}