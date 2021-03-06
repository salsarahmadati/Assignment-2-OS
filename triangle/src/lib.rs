pub struct Triangle{
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides[0] == 0 && sides[1] == 0 && sides[2] ==0{
          None
      } else if sides[2] > sides[1] + sides[0] || sides[1] > sides[2] + sides[0] || sides[0] > sides[1] + sides[2]{
        None

      }else {
          Some(Triangle{a:sides[0], b:sides[1], c:sides[2]})
      }
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.b == self.c
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.a == self.c
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }

    
}

//Problem link: https://exercism.io/tracks/rust/exercises/triangle/solutions/e58fe344a16c49acb84a1d83fd3477e9
