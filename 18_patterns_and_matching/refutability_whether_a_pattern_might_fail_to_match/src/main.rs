/**
Refutability: Whether a Pattern Might Fail to Match
patterns的两种形式
irrefutable pattern：能够匹配所有情形的pattern，诸如let x = 5; x能够匹配任何类型，因此不会出现不匹配情况
refutable pattern：有些情形会不匹配的pattern，比如if let Some(x) = a_value，如果a_value为None，那么就会匹配不上

什么场景下能够使用irrefutable或refutable pattern
函数参数、let语句、for循环只接受irrefutable pattern，因为一旦匹配不上，这些场景就不能继续正常运行
if let、while let能够同时接受irrefutable和refutable pattern
 */
fn main() {
    let some_option_value = Some(123);
    // let Some(x) = some_option_value; 编译错误，因为let只能接受irrefutable pattern，然而Some(x)不能匹配None的情况
    if let Some(x) = some_option_value { // if let同时接受两种pattern，即便没匹配上，只是不运行花括号里的代码
        println!("{}", x);
    }

    /*
    也能正常运行，因为x肯定能匹配上，并且运行花括号里的代码，但是会有编译警告，因为这里总是会匹配上运行括号里代码，属于无用代码
     */
    if let x = 5 {
        println!("{}", x);
    };


}
