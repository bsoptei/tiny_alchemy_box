use derive_more::*;
use num_traits::Num;
use photonix::*;

#[derive(Clone, Copy, Add, Div, Mul, Sub, From, PartialEq, PartialOrd, Debug)]
pub struct X<Value: Num>(pub Value);

#[derive(Clone, Copy, Add, Div, Mul, Sub, From, PartialEq, PartialOrd, Debug)]
pub struct Y<Value: Num>(pub Value);

#[derive(Clone, Copy, Add, Div, Mul, Sub, From, PartialEq, PartialOrd, Debug)]
pub struct Radius<Value: Num>(pub Value);

#[derive(Clone, Copy, Add, Div, Mul, Sub, From, PartialEq, PartialOrd, Debug)]
pub struct LineWidth<Value: Num>(pub Value);

#[derive(Clone, Copy, Add, Div, Mul, Sub, From, PartialEq, PartialOrd, Debug)]
pub struct Coordinate2D<Value: Num> {
    pub x: X<Value>,
    pub y: Y<Value>
}

impl<Value: Num> Coordinate2D<Value> {
    pub fn new(x: X<Value>, y: Y<Value>) -> Self {
        Self { x, y }
    }
}

impl<Value: Num> Set<X<Value>> for Coordinate2D<Value> {
    fn set(self, x: X<Value>) -> Self {
            Self { x, y: self.y }
    }
}

impl<Value: Num> Set<Y<Value>> for Coordinate2D<Value> {
    fn set(self, y: Y<Value>) -> Self {
        Self { x: self.x, y }
    }
}

impl<Value: Num> Modify<X<Value>> for Coordinate2D<Value> {
    fn modify(self, f: impl FnOnce(X<Value>) -> X<Value>) -> Self {
        Self { x: f(self.x), y: self.y }
    }
}

impl<Value: Num> Modify<Y<Value>> for Coordinate2D<Value> {
    fn modify(self, f: impl FnOnce(Y<Value>) -> Y<Value>) -> Self {
        Self { x: self.x, y: f(self.y) }
    }
}

impl<Value: Num> Into<X<Value>> for Radius<Value> {
    fn into(self) -> X<Value> {
        X(self.0)
    }
}

impl<Value: Num> Into<Y<Value>> for Radius<Value> {
    fn into(self) -> Y<Value> {
        Y(self.0)
    }
}

impl<Value: Num> Into<Radius<Value>> for X<Value> {
    fn into(self) -> Radius<Value> {
        Radius(self.0)
    }
}

impl<Value: Num> Into<Radius<Value>> for Y<Value> {
    fn into(self) -> Radius<Value> {
        Radius(self.0)
    }
}