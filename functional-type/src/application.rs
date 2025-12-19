pub trait FunctionDefinition<Arg> {
    type Output;
}

pub trait FunctionDefinition2<Arg1, Arg2> {
    type Output;
}

pub trait FunctionDefinition3<Arg1, Arg2, Arg3> {
    type Output;
}

pub type Application<F, Arg> = <F as FunctionDefinition<Arg>>::Output;
pub type Application2<F, Arg1, Arg2> = <F as FunctionDefinition2<Arg1, Arg2>>::Output;
pub type Application3<F, Arg1, Arg2, Arg3> = <F as FunctionDefinition3<Arg1, Arg2, Arg3>>::Output;
