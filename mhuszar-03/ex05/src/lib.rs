/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/10 21:39:09 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/10 23:57:59 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::ops::{Mul, MulAssign, Div, DivAssign};
use std::cmp::{PartialEq, Eq};

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(Eq)]
#[derive(PartialEq)]
struct Vector<T> {
    x: T,
    y: T,
}

impl<T> Vector<T>
{
    fn new(x: T, y: T) -> Self
    {
        Vector
        {
            x,
            y
        }
    }
    // fn clone(&self) -> Self
    // where
    //     T : std::clone::Clone
    // {
    //     Vector::new(self.x.clone(), self.y.clone())
    // }
}

#[allow(dead_code)]
impl Vector<f32>
{
    fn length(&self) -> f32
    {
        f32::sqrt((self.x * self.x) + (self.y * self.y))
    }
}

#[allow(dead_code)]
impl Vector<f64>
{
    fn length(&self) -> f64
    {
        f64::sqrt((self.x * self.x) + (self.y * self.y))
    }
}

// impl Vector<String>
// {
//     fn clone(&self) -> Self
//     {
//         Vector::new(self.y.clone(), self.x.clone())
//     }
// }

impl<T> Add for Vector<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self
    {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self
    {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl<T> AddAssign for Vector<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, other: Self)
    {
            self.x += other.x;
            self.y += other.y;
    }
}

impl<T> SubAssign for Vector<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, other: Self)
    {
            self.x -= other.x;
            self.y -= other.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector<T>
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self
    {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vector<T>
where 
    T: MulAssign + Copy
{
    fn mul_assign(&mut self, rhs: T)
    {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vector<T>
{
    type Output = Self;
    fn div(self, rhs: T) -> Self
    {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector<T>
where 
    T: DivAssign + Copy
{
    fn div_assign(&mut self, rhs: T)
    {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// impl<T> PartialEq for Vector<T>
// where 
//     T: PartialEq
// {
//     fn eq(&self, other: &Self) -> bool
//     {
//         match (self.x == other.x, self.y == other.y)
//         {
//             (true, true) => true,
//             _ => false
//         }
//     }
// }

// impl<T> Eq for Vector<T>
// where
//     T: Eq
// {
    
// }

// impl<T> Clone for Vector<T>
// where
//     T: Clone
// {
//     fn clone(&self) -> Self
//     {
//         Vector::new(self.x.clone(), self.y.clone())
//     }
// }

#[cfg(test)]
#[test]
fn test_a() {
    let v = Vector {
        x: String::from("Hello, World!"),
        y: String::from("Hello, Rust!"),
    };

    let w = v.clone();

    assert_eq!(&v, &w);
}

#[cfg(test)]
#[test]
fn test_b() {
    let v = Vector::new("Hello, World!", "Hello, Rust!");
    let a = v;
    let b = v;
    assert_eq!(a, b);
}
