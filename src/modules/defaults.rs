// src/defaults.rs

use crate::modules::maths;

pub const DIRECTION_ALL: [maths::Size; 8] = [
   maths::Size { x: 1, y: 0 },
   maths::Size { x: 1, y: 1 },
   maths::Size { x: 0, y: 1 },
   maths::Size { x: -1, y: 1 },
   maths::Size { x: -1, y: 0 },
   maths::Size { x: -1, y: -1 },
   maths::Size { x: 0, y: -1 },
   maths::Size { x: 1, y: -1 },

];

pub const DIRECTION_RIGHT_TO_LEFT: [maths::Size; 4] = [
   maths::Size { x: 1, y: 0 },
   maths::Size { x: 1, y: 1 },
   maths::Size { x: 0, y: -1 },
   maths::Size { x: 1, y: -1 },

];