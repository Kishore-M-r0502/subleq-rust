// This file is part of SUBLEQ Rust.
//
// Copyright (C) 2026 Kishore M R
//
// SUBLEQ Rust is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// SUBLEQ Rust is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with SUBLEQ Rust. If not, see <https://www.gnu.org/licenses/>.
//src/core/math/basic.rs
use crate::core::subleq::Computer;

trait Arithmetic {
    fn add(&mut self,a:usize,b:usize)->usize;
}
// impl Arithmetic for Computer {
//     fn add(&mut self,a:usize,b:usize)->usize {
//         let zero=self.clear(255);

        
//     }
    
// }