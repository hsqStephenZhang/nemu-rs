#![allow(unused)]

use std::{borrow::Cow, rc::Rc};

use paste::paste;

pub type EResult<'a, T> = std::result::Result<(&'a str, T), &'a str>;

pub trait Eat {
    type Output;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output>;
}

impl<T> Eat for &T
where
    T: Eat,
{
    type Output = T::Output;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        (*self).eat(input)
    }
}

impl <T> Eat for Rc<T>
where
    T: Eat,
{
    type Output = T::Output;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        self.as_ref().eat(input)
    }
}

impl<T> Eat for Box<T>
where
    T: Eat,
{
    type Output = T::Output;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        self.as_ref().eat(input)
    }
}

pub trait Parser<R> {
    fn parse<'a>(&self, input: &'a str) -> std::result::Result<R, &'a str>;
}

impl<R, T: Eat<Output = R>> Parser<R> for T {
    fn parse<'a>(&self, input: &'a str) -> std::result::Result<R, &'a str> {
        let res = self.eat(input)?;
        if res.0.is_empty() {
            Ok(res.1)
        } else {
            Err(res.0)
        }
    }
}

pub struct Char(pub char);

impl Eat for Char {
    type Output = char;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        if input.is_empty() {
            return Err(input);
        }
        if input.chars().next().unwrap() == self.0 {
            Ok((&input[1..], self.0))
        } else {
            Err(input)
        }
    }
}

pub struct S(pub String);

impl Eat for S {
    type Output = String;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        if input.starts_with(&self.0) {
            Ok((&input[self.0.len()..], self.0.clone()))
        } else {
            Err(input)
        }
    }
}

pub struct Optional<T>(pub T);

impl<T> Eat for Optional<T>
where
    T: Eat,
{
    type Output = Option<T::Output>;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        match self.0.eat(input) {
            Ok((input, result)) => Ok((input, Some(result))),
            Err(_) => Ok((input, None)),
        }
    }
}

pub trait Compose<O> {
    fn compose<'a>(&'a self) -> impl Iterator<Item = &'a dyn Eat<Output = O>>
    where
        O: 'a;
}

macro_rules! impl_compose_for_tuples {
    ($($name:ident),+) => {
        paste! {
            impl<O, $($name),+> Compose<O> for ($($name,)+)
            where
                $($name: Eat<Output = O>,)+
            {
                fn compose<'a>(&'a self) -> impl Iterator<Item = &'a dyn Eat<Output = O>>
                where
                    O: 'a,
                {
                    let ($(ref [<$name:lower>],)+) = *self;
                    let vec: Vec<&'a dyn Eat<Output = O>> = vec![$([<$name:lower>] as &dyn Eat<Output = O>,)+];
                    vec.into_iter()
                }
            }
        }
    };
}

// Now invoke the macro for tuples from size 2 to 16
impl_compose_for_tuples!(T1);
impl_compose_for_tuples!(T1, T2);
impl_compose_for_tuples!(T1, T2, T3);
impl_compose_for_tuples!(T1, T2, T3, T4);
impl_compose_for_tuples!(T1, T2, T3, T4, T5);
impl_compose_for_tuples!(T1, T2, T3, T4, T5, T6);
impl_compose_for_tuples!(T1, T2, T3, T4, T5, T6, T7);
impl_compose_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_compose_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_compose_for_tuples!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);

pub struct Concats<O, T> {
    pub t: T,
    _phantom: std::marker::PhantomData<O>,
}

impl<O, T> Eat for Concats<O, T>
where
    T: Compose<O>,
{
    type Output = Vec<O>;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let mut result = Vec::new();
        let mut input = input;
        for parser in self.t.compose() {
            match parser.eat(input) {
                Ok((next_input, s)) => {
                    result.push(s);
                    input = next_input;
                }
                Err(_) => return Err(input),
            }
        }
        Ok((input, result))
    }
}

pub fn concats<O, T: Compose<O>>(t: T) -> Concats<O, impl Compose<O>> {
    Concats {
        t,
        _phantom: std::marker::PhantomData,
    }
}

pub struct Alts<O, T> {
    pub t: T,
    _phantom: std::marker::PhantomData<O>,
}

pub fn alt<O, T: Compose<O>>(t: T) -> Alts<O, impl Compose<O>> {
    Alts {
        t,
        _phantom: std::marker::PhantomData,
    }
}

impl<O, T> Eat for Alts<O, T>
where
    T: Compose<O>,
{
    type Output = O;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let Alts { t, _phantom } = self;
        for parser in t.compose() {
            if let Ok(result) = parser.eat(input) {
                return Ok(result);
            }
        }
        Err("No match")
    }
}

pub struct Concat<T1, T2>(pub T1, pub T2);

impl<T1, T2, R1, R2> Eat for Concat<T1, T2>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
{
    type Output = (R1, R2);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, s1) = self.0.eat(input)?;
        let (input, s2) = self.1.eat(input)?;
        Ok((input, (s1, s2)))
    }
}

pub struct Concat3<T1, T2, T3>(pub T1, pub T2, pub T3);

impl<T1, T2, T3, R1, R2, R3> Eat for Concat3<T1, T2, T3>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
{
    type Output = (R1, R2, R3);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, s1) = self.0.eat(input)?;
        let (input, s2) = self.1.eat(input)?;
        let (input, s3) = self.2.eat(input)?;
        Ok((input, (s1, s2, s3)))
    }
}

pub struct Alt<T1, T2>(pub T1, pub T2);

impl<T1, T2, R> Eat for Alt<T1, T2>
where
    T1: Eat<Output = R>,
    T2: Eat<Output = R>,
{
    type Output = R;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, R> {
        let Alt(t1, t2) = self;
        if let Ok(result) = t1.eat(input) {
            return Ok(result);
        }
        if let Ok(result) = t2.eat(input) {
            return Ok(result);
        }
        Err("No match")
    }
}

pub struct Many<T>(pub T);

impl<T, R> Eat for Many<T>
where
    T: Eat<Output = R>,
{
    type Output = Vec<R>;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Vec<R>> {
        let mut result = Vec::new();
        let mut input = input;
        loop {
            match self.0.eat(input) {
                Ok((next_input, s)) => {
                    result.push(s);
                    input = next_input;
                }
                Err(_) => break,
            }
        }
        Ok((input, result))
    }
}

// at least n times
pub struct Manyn<T> {
    inner: T,
    n: usize,
}

pub fn many<T>(inner: T, n: usize) -> Manyn<T> {
    Manyn { inner, n }
}

impl<T, R> Eat for Manyn<T>
where
    T: Eat<Output = R>,
{
    type Output = Vec<R>;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Vec<R>> {
        let mut result = Vec::new();
        let mut input = input;
        let mut count = 0;
        loop {
            match self.inner.eat(input) {
                Ok((next_input, s)) => {
                    result.push(s);
                    input = next_input;
                    count += 1;
                }
                Err(_) => break,
            }
        }
        if count < self.n {
            return Err(input);
        }

        Ok((input, result))
    }
}

pub struct Str(pub &'static str);

impl Eat for Str {
    type Output = &'static str;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        if input.starts_with(self.0) {
            Ok((&input[self.0.len()..], self.0))
        } else {
            Err(input)
        }
    }
}

pub struct Between<T1, T2, T3>(pub T1, pub T2, pub T3);

impl<T1, T2, T3, R1, R2, R3> Eat for Between<T1, T2, T3>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
{
    type Output = R2;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, _) = self.0.eat(input)?;
        let (input, result) = self.1.eat(input)?;
        // println!("step2, input:{}",input);
        let (input, _) = self.2.eat(input)?;
        // println!("step3, input:{}",input);
        Ok((input, result))
    }
}

pub struct SepBy<T, S>(pub T, pub S);

impl<T, S, R, SR> Eat for SepBy<T, S>
where
    T: Eat<Output = R>,
    S: Eat<Output = SR>,
{
    type Output = Vec<R>;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let mut result = Vec::new();
        let mut current_input = input;

        // Try to parse the first element
        match self.0.eat(current_input) {
            Ok((next_input, item)) => {
                result.push(item);
                current_input = next_input;

                // Parse remaining elements preceded by separator
                loop {
                    match self.1.eat(current_input) {
                        Ok((sep_input, _)) => match self.0.eat(sep_input) {
                            Ok((next_input, item)) => {
                                result.push(item);
                                current_input = next_input;
                            }
                            Err(_) => break,
                        },
                        Err(_) => break,
                    }
                }

                Ok((current_input, result))
            }
            Err(_) => Ok((current_input, Vec::new())), // Empty list is valid
        }
    }
}

pub struct Satisfy<F>(pub F);

impl<F> Eat for Satisfy<F>
where
    F: Fn(char) -> bool,
{
    type Output = char;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        if input.is_empty() {
            return Err(input);
        }
        let ch = input.chars().next().unwrap();
        if (self.0)(ch) {
            Ok((&input[ch.len_utf8()..], ch))
        } else {
            Err(input)
        }
    }
}

pub struct WS<T>(pub T);

impl<T, R> Eat for WS<T>
where
    T: Eat<Output = R>,
{
    type Output = R;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, R> {
        let input = skip_whitespace(input);
        let (input, result) = self.0.eat(input)?;
        let input = skip_whitespace(input);
        Ok((input, result))
    }
}

// Helper function to skip whitespace
fn skip_whitespace(input: &str) -> &str {
    let mut chars = input.chars();
    let mut count = 0;

    for c in &mut chars {
        if !c.is_whitespace() {
            break;
        }
        count += c.len_utf8();
    }

    &input[count..]
}

pub struct Recursion<R> {
    f: Box<dyn for<'call> Fn(&Recursion<R>, &'call str) -> EResult<'call, R>>,
}

impl<R> Recursion<R> {
    pub fn new<F>(f: F) -> Self
    where
        F: for<'call> Fn(&Recursion<R>, &'call str) -> EResult<'call, R> + 'static,
    {
        Self { f: Box::new(f) }
    }
}

impl<R: Clone> Eat for Recursion<R> {
    type Output = R;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        (self.f)(self, input)
    }
}

pub struct Mapper<T, F>(pub T, pub F);

impl<R, R1, F, T> Eat for Mapper<T, F>
where
    T: Eat<Output = R>,
    F: Fn(T::Output) -> R1,
{
    type Output = R1;
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, R1> {
        let (input, result) = self.0.eat(input)?;
        Ok((input, (self.1)(result)))
    }
}

pub trait MapperExt: Eat {
    fn to<F, R1>(self, f: F) -> Mapper<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> R1;
}

impl<T: Eat> MapperExt for T {
    fn to<F, R1>(self, f: F) -> Mapper<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Output) -> R1,
    {
        Mapper(self, f)
    }
}

#[cfg(test)]
mod json_test {

    use super::*;
    use std::collections::*;

    pub struct Debugger<T>(pub T);

    impl<R, T> Eat for Debugger<T>
    where
        R: std::fmt::Debug,
        T: Eat<Output = R>,
    {
        type Output = R;
        fn eat<'a>(&self, input: &'a str) -> EResult<'a, R> {
            let orig_input = input;
            let (input, result) = self.0.eat(input)?;
            println!(
                "orig input: \"{}\", current input: \"{}\", result: {:?}",
                orig_input, input, result
            );
            Ok((input, result))
        }
    }

    trait DebugExt: Eat {
        fn dbg(self) -> Debugger<Self>
        where
            Self: Sized;
    }

    impl<T: Eat> DebugExt for T
    where
        T::Output: std::fmt::Debug,
    {
        fn dbg(self) -> Debugger<Self>
        where
            Self: Sized,
        {
            Debugger(self)
        }
    }

    // JSON Value enum
    #[derive(Debug, Clone, PartialEq)]
    enum JsonValue {
        Null,
        Bool(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(HashMap<String, JsonValue>),
    }

    // JSON Parser implementation
    fn json_parser() -> impl Eat<Output = JsonValue> {
        json_object()
    }

    fn json_null() -> impl Eat<Output = JsonValue> {
        Mapper(Str("null"), |_| JsonValue::Null)
    }

    fn json_bool() -> impl Eat<Output = JsonValue> {
        Mapper(alt((Str("true"), Str("false"))), |s| {
            JsonValue::Bool(s == "true")
        })
    }

    fn json_number() -> impl Eat<Output = JsonValue> {
        number_parser().to(|s: String| JsonValue::Number(s.parse::<f64>().unwrap()))
    }

    fn number_parser() -> impl Eat<Output = String> {
        let maybe_minus = Optional(Char('-')).to(|minus| minus.and_then(|_| Some("-".to_owned())));
        let integer_parser =
            many(digit_parser(), 1).to(|digits| Some(digits.iter().collect::<String>()));
        let fraction_parser = fraction_parser();
        concats((maybe_minus, integer_parser, Optional(fraction_parser))).to(|parts| {
            let mut result = String::new();
            for part in &parts {
                if let Some(part) = part {
                    result.push_str(part);
                }
            }
            result
        })
    }

    fn digit_parser() -> impl Eat<Output = char> {
        let f = |c: char| c.is_ascii_digit();
        Satisfy(f)
    }

    fn fraction_parser() -> impl Eat<Output = String> {
        concats((Char('.').to(|_| vec![]), many(digit_parser(), 1)))
            .to(|digits| format!(".{}", digits[1].iter().collect::<String>()))
    }

    fn json_string() -> impl Eat<Output = JsonValue> {
        string_parser().to(|s| JsonValue::String(s))
    }

    fn string_parser() -> impl Eat<Output = String> {
        Between(Char('"'), Many(string_char_parser()), Char('"'))
            .to(|chars| chars.into_iter().collect())
    }

    fn string_char_parser() -> impl Eat<Output = char> {
        alt((
            escaped_char_parser(),
            Satisfy(|c| c != '"' && c != '\\' && c > '\u{001F}'),
        ))
    }

    fn escaped_char_parser() -> impl Eat<Output = char> {
        let escape = Char('\\');
        let escaped = alt((
            Mapper(Char('"'), |_| '"'),
            Mapper(Char('\\'), |_| '\\'),
            Mapper(Char('/'), |_| '/'),
            Mapper(Char('b'), |_| '\u{0008}'),
            Mapper(Char('f'), |_| '\u{000C}'),
            Mapper(Char('n'), |_| '\n'),
            Mapper(Char('r'), |_| '\r'),
            Mapper(Char('t'), |_| '\t'),
        ));
        concats((escape, escaped)).to(|v| v[1])
    }

    fn json_object() -> impl Eat<Output = JsonValue> {
        let json_value_ref = Recursion::new(|_rec, input| json_object().eat(input));

        let field_name = string_parser();
        let recursive_value = WS(json_value_ref);

        #[derive(Debug)]
        enum KV {
            K(String),
            V(JsonValue),
        }

        let all_fields = SepBy(
            concats((
                field_name.to(|k| KV::K(k)).dbg(),
                WS(Char(':')).to(|_| KV::K("".into())).dbg(),
                alt((
                    json_null(),
                    json_bool(),
                    json_string(),
                    json_number(),
                    json_array(),
                    recursive_value,
                ))
                .to(|v| KV::V(v))
                .dbg(),
            )),
            WS(Char(',')),
        );
        Between(
            WS(Char('{')).dbg(),
            all_fields.to(|s| {
                println!("parsing s");
                s
            }),
            WS(Char('}')).dbg(),
        )
        .dbg()
        .to(|pairs| {
            let mut map = HashMap::new();
            for pair in pairs {
                let mut it = pair.into_iter();
                let field_name = match it.next() {
                    Some(KV::K(k)) => k,
                    _ => panic!("Invalid field name"),
                };
                let _ = it.next();
                let field_value = match it.next() {
                    Some(KV::V(v)) => v,
                    _ => panic!("Invalid field value"),
                };
                map.insert(field_name, field_value);
            }
            JsonValue::Object(map)
        })
    }

    fn json_array() -> impl Eat<Output = JsonValue> {
        let json_value_ref = Recursion::new(|rec, input| json_parser().eat(input));

        Mapper(
            Between(
                WS(Char('[')),
                SepBy(WS(json_value_ref), WS(Char(','))),
                WS(Char(']')),
            ),
            |values| JsonValue::Array(values),
        )
    }

    #[test]
    fn test_json_fields() {
        // null
        let parser = json_null();
        let result = parser.parse("null");
        assert_eq!(result, Ok(JsonValue::Null));

        let result = parser.parse("aaanull");
        assert_eq!(result, Err("aaanull"));

        // bool
        let parser = json_bool();
        let result = parser.parse("true");
        assert_eq!(result, Ok(JsonValue::Bool(true)));
        let result = parser.parse("false");
        assert_eq!(result, Ok(JsonValue::Bool(false)));
        let result = parser.parse("aafalse");
        assert!(result.is_err());

        // string
        let parser = json_string();
        let result = parser.parse(r#""hello""#);
        assert_eq!(result, Ok(JsonValue::String("hello".to_string())));
        let result = parser.parse(r#""hello\nworld""#);
        assert_eq!(result, Ok(JsonValue::String("hello\nworld".to_string())));

        // number
        let parser = json_number();
        let result = parser.parse("123");
        assert_eq!(result, Ok(JsonValue::Number(123.0)));
        let result = parser.parse("-123.45");
        assert_eq!(result, Ok(JsonValue::Number(-123.45)));

        // object
        let parser = json_object();
        let result = parser.parse(r#"{"key": {"key1": {"key2": "value"}}}"#);
        assert_eq!(
            format!("{:?}", result.unwrap()),
            r#"Object({"key": Object({"key1": Object({"key2": String("value")})})})"#
        );

        let result = parser.parse(r#"{"key": {"key1": [{"key2": "value"}, {"key2": "value"}]}}"#);
        assert_eq!(
            format!("{:?}", result.unwrap()),
            r#"Object({"key": Object({"key1": Array([Object({"key2": String("value")}), Object({"key2": String("value")})])})})"#
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    enum Res {
        Rec(Box<Option<Res>>),
    }

    #[test]
    fn test_recursion() {
        let parser: Recursion<Res> = Recursion {
            f: Box::new(|this, input| {
                let (input, _a) = Char('a').eat(input)?;
                let (input, res) = Optional(this).eat(input)?;
                let (input, _b) = Char('b').eat(input)?;
                Ok((input, Res::Rec(Box::new(res))))
            }),
        };
        let result = parser.eat("aaaabbbb".into());
        println!("Result: {:?}", result);

        let result = parser.eat("aab".into());
        println!("Result: {:?}", result);
        let result = parser.eat("aabbb".into());
        println!("Result: {:?}", result);

        let result = parser.eat("aabbbbbb".into());
        println!("Result: {:?}", result);
    }

    #[test]
    fn test_basic() {
        let parser = Char('a');
        let result = parser.parse("a");
        assert_eq!(result, Ok('a'));

        let parser = Concat(Char('a'), Char('b'));
        let result = parser.parse("ab");
        assert_eq!(result, Ok(('a', 'b')));

        let parser = Concat(Char('a'), Char('b'));
        let result = parser.parse("abc");
        assert_eq!(result, Err("c"));

        let parser = Concat(Char('a'), Char('b'));
        let result = parser.parse("a");
        assert_eq!(result, Err(""));

        let parser = Alt(Char('a'), Char('b'));
        let result = parser.parse("a");
        assert_eq!(result, Ok('a'));
        let result = parser.parse("b");
        assert_eq!(result, Ok('b'));

        let parser = Many(Concat(Char('a'), Char('b')));
        let result = parser.parse("ababab");
        assert_eq!(result, Ok(vec![('a', 'b'); 3]));

        let parser = Many(Alt(Char('a'), Char('b')));
        let result = parser.parse("bbbbbaaaa");
        assert_eq!(
            result,
            Ok(vec!['b', 'b', 'b', 'b', 'b', 'a', 'a', 'a', 'a'])
        );
    }

    #[test]
    fn test_json_fields() {
        let parser = concats((Char('n'), Char('u'), Char('l'), Char('l')));
        let result = parser.parse("null");
        assert_eq!(result, Ok(vec!['n', 'u', 'l', 'l']));

        let parser = Many(alt((Char('n'), Char('u'), Char('l'), Char('l'))));
        let result = parser.eat("nnnuuulltt");
        println!("Result: {:?}", result);
    }
}
