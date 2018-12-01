use derive_more::*;
use num_traits::Num;

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

    pub fn at_x(self, x: X<Value>) -> Self {
        Self { x, y: self.y }
    }

    pub fn at_y(self, y: Y<Value>) -> Self {
        Self { x: self.x, y }
    }

    pub fn up(self, y: Y<Value>) -> Self {
        Self { x: self.x, y: self.y - y }
    }

    pub fn down(self, y: Y<Value>) -> Self {
        Self { x: self.x, y: self.y + y }
    }

    pub fn left(self, x: X<Value>) -> Self {
        Self { x: self.x - x, y: self.y }
    }

    pub fn right(self, x: X<Value>) -> Self {
        Self { x: self.x + x, y: self.y }
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