pub type Step = [bool; 4];

// pub type Path<'a> = std::slice::Iter<'a, [bool; 4]>;
pub type Path<'a> = std::vec::IntoIter<[bool; 4]>;
