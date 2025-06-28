use crate::compose::*;

pub struct Concat1<T1>(pub T1);
impl<T1, R1> Eat for Concat1<T1>
where
    T1: Eat<Output = R1>,
{
    type Output = (R1,);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        Ok((input, (r1,)))
    }
}

// Concat2 (equivalent to your original Concat)
pub struct Concat2<T1, T2>(pub T1, pub T2);
impl<T1, T2, R1, R2> Eat for Concat2<T1, T2>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
{
    type Output = (R1, R2);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        let (input, r2) = self.1.eat(input)?;
        Ok((input, (r1, r2)))
    }
}

// Concat3 (equivalent to your original Concat3)
pub struct Concat3<T1, T2, T3>(pub T1, pub T2, pub T3);
impl<T1, T2, T3, R1, R2, R3> Eat for Concat3<T1, T2, T3>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
{
    type Output = (R1, R2, R3);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        let (input, r2) = self.1.eat(input)?;
        let (input, r3) = self.2.eat(input)?;
        Ok((input, (r1, r2, r3)))
    }
}

// Concat4
pub struct Concat4<T1, T2, T3, T4>(pub T1, pub T2, pub T3, pub T4);
impl<T1, T2, T3, T4, R1, R2, R3, R4> Eat for Concat4<T1, T2, T3, T4>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
    T4: Eat<Output = R4>,
{
    type Output = (R1, R2, R3, R4);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        let (input, r2) = self.1.eat(input)?;
        let (input, r3) = self.2.eat(input)?;
        let (input, r4) = self.3.eat(input)?;
        Ok((input, (r1, r2, r3, r4)))
    }
}

// Concat5
pub struct Concat5<T1, T2, T3, T4, T5>(pub T1, pub T2, pub T3, pub T4, pub T5);
impl<T1, T2, T3, T4, T5, R1, R2, R3, R4, R5> Eat for Concat5<T1, T2, T3, T4, T5>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
    T4: Eat<Output = R4>,
    T5: Eat<Output = R5>,
{
    type Output = (R1, R2, R3, R4, R5);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        let (input, r2) = self.1.eat(input)?;
        let (input, r3) = self.2.eat(input)?;
        let (input, r4) = self.3.eat(input)?;
        let (input, r5) = self.4.eat(input)?;
        Ok((input, (r1, r2, r3, r4, r5)))
    }
}

// Concat6
pub struct Concat6<T1, T2, T3, T4, T5, T6>(pub T1, pub T2, pub T3, pub T4, pub T5, pub T6);
impl<T1, T2, T3, T4, T5, T6, R1, R2, R3, R4, R5, R6> Eat for Concat6<T1, T2, T3, T4, T5, T6>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
    T4: Eat<Output = R4>,
    T5: Eat<Output = R5>,
    T6: Eat<Output = R6>,
{
    type Output = (R1, R2, R3, R4, R5, R6);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        let (input, r2) = self.1.eat(input)?;
        let (input, r3) = self.2.eat(input)?;
        let (input, r4) = self.3.eat(input)?;
        let (input, r5) = self.4.eat(input)?;
        let (input, r6) = self.5.eat(input)?;
        Ok((input, (r1, r2, r3, r4, r5, r6)))
    }
}

// Concat7
pub struct Concat7<T1, T2, T3, T4, T5, T6, T7>(
    pub T1,
    pub T2,
    pub T3,
    pub T4,
    pub T5,
    pub T6,
    pub T7,
);
impl<T1, T2, T3, T4, T5, T6, T7, R1, R2, R3, R4, R5, R6, R7> Eat
    for Concat7<T1, T2, T3, T4, T5, T6, T7>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
    T4: Eat<Output = R4>,
    T5: Eat<Output = R5>,
    T6: Eat<Output = R6>,
    T7: Eat<Output = R7>,
{
    type Output = (R1, R2, R3, R4, R5, R6, R7);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        let (input, r2) = self.1.eat(input)?;
        let (input, r3) = self.2.eat(input)?;
        let (input, r4) = self.3.eat(input)?;
        let (input, r5) = self.4.eat(input)?;
        let (input, r6) = self.5.eat(input)?;
        let (input, r7) = self.6.eat(input)?;
        Ok((input, (r1, r2, r3, r4, r5, r6, r7)))
    }
}

// Concat8
pub struct Concat8<T1, T2, T3, T4, T5, T6, T7, T8>(
    pub T1,
    pub T2,
    pub T3,
    pub T4,
    pub T5,
    pub T6,
    pub T7,
    pub T8,
);
impl<T1, T2, T3, T4, T5, T6, T7, T8, R1, R2, R3, R4, R5, R6, R7, R8> Eat
    for Concat8<T1, T2, T3, T4, T5, T6, T7, T8>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
    T4: Eat<Output = R4>,
    T5: Eat<Output = R5>,
    T6: Eat<Output = R6>,
    T7: Eat<Output = R7>,
    T8: Eat<Output = R8>,
{
    type Output = (R1, R2, R3, R4, R5, R6, R7, R8);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        let (input, r2) = self.1.eat(input)?;
        let (input, r3) = self.2.eat(input)?;
        let (input, r4) = self.3.eat(input)?;
        let (input, r5) = self.4.eat(input)?;
        let (input, r6) = self.5.eat(input)?;
        let (input, r7) = self.6.eat(input)?;
        let (input, r8) = self.7.eat(input)?;
        Ok((input, (r1, r2, r3, r4, r5, r6, r7, r8)))
    }
}

// Concat9
pub struct Concat9<T1, T2, T3, T4, T5, T6, T7, T8, T9>(
    pub T1,
    pub T2,
    pub T3,
    pub T4,
    pub T5,
    pub T6,
    pub T7,
    pub T8,
    pub T9,
);
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, R1, R2, R3, R4, R5, R6, R7, R8, R9> Eat
    for Concat9<T1, T2, T3, T4, T5, T6, T7, T8, T9>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
    T4: Eat<Output = R4>,
    T5: Eat<Output = R5>,
    T6: Eat<Output = R6>,
    T7: Eat<Output = R7>,
    T8: Eat<Output = R8>,
    T9: Eat<Output = R9>,
{
    type Output = (R1, R2, R3, R4, R5, R6, R7, R8, R9);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        let (input, r2) = self.1.eat(input)?;
        let (input, r3) = self.2.eat(input)?;
        let (input, r4) = self.3.eat(input)?;
        let (input, r5) = self.4.eat(input)?;
        let (input, r6) = self.5.eat(input)?;
        let (input, r7) = self.6.eat(input)?;
        let (input, r8) = self.7.eat(input)?;
        let (input, r9) = self.8.eat(input)?;
        Ok((input, (r1, r2, r3, r4, r5, r6, r7, r8, r9)))
    }
}

// Concat10
pub struct Concat10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>(
    pub T1,
    pub T2,
    pub T3,
    pub T4,
    pub T5,
    pub T6,
    pub T7,
    pub T8,
    pub T9,
    pub T10,
);
impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10> Eat
    for Concat10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
where
    T1: Eat<Output = R1>,
    T2: Eat<Output = R2>,
    T3: Eat<Output = R3>,
    T4: Eat<Output = R4>,
    T5: Eat<Output = R5>,
    T6: Eat<Output = R6>,
    T7: Eat<Output = R7>,
    T8: Eat<Output = R8>,
    T9: Eat<Output = R9>,
    T10: Eat<Output = R10>,
{
    type Output = (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10);
    fn eat<'a>(&self, input: &'a str) -> EResult<'a, Self::Output> {
        let (input, r1) = self.0.eat(input)?;
        let (input, r2) = self.1.eat(input)?;
        let (input, r3) = self.2.eat(input)?;
        let (input, r4) = self.3.eat(input)?;
        let (input, r5) = self.4.eat(input)?;
        let (input, r6) = self.5.eat(input)?;
        let (input, r7) = self.6.eat(input)?;
        let (input, r8) = self.7.eat(input)?;
        let (input, r9) = self.8.eat(input)?;
        let (input, r10) = self.9.eat(input)?;
        Ok((input, (r1, r2, r3, r4, r5, r6, r7, r8, r9, r10)))
    }
}
