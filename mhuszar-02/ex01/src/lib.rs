/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/09 18:55:46 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/09 19:58:53 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
impl Point {
    fn new(x: f32, y: f32) -> Self
    {
        Point
        {
            x,
            y,
        }
    }
    fn zero() -> Self
    {
        Point {
            x : 0.0,
            y : 0.0,
        }
    }
    fn distance(&self, other: &Self) -> f32
    {
        f32::sqrt((self.y - other.y) * (self.y - other.y) + (self.x - other.x) * (self.x - other.x))
    }
    fn translate(&mut self, dx: f32, dy: f32)
    {
        self.x += dx;
        self.y += dy;
    }
}

#[test]
#[cfg(test)]
fn new_test()
{
    let hehe: Point = Point::new(42.0, 99.0);
    assert_eq!(hehe.x, 42.0);
    assert_eq!(hehe.y, 99.0);
}

#[test]
#[cfg(test)]
fn zero_test()
{
    let hehe: Point = Point::zero();
    assert_eq!(hehe.x, 0.0);
    assert_eq!(hehe.y, 0.0);
}

#[test]
#[cfg(test)]
fn distance_test()
{
    let hehe1: Point = Point::new(0.0, 0.0);
    let hehe2: Point = Point::new(3.0, 3.0);
    let val = Point::distance(&hehe1, &hehe2);
    assert_eq!(val, 4.2426405);
}

#[test]
#[cfg(test)]
fn translate_test()
{
    let mut hehe: Point = Point::new(0.0, 0.0);
    Point::translate(&mut hehe, 9.5, 100.6);
    assert_eq!(hehe.x, 9.5);
    assert_eq!(hehe.y, 100.6);
}

