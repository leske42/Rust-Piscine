/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/12 14:29:02 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/12 14:52:34 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::cell::Cell;

#[allow(dead_code)]
fn swap_u32<'a>(a: &'a Cell<u32>, b: &'a Cell<u32>)
{
    let mut hehe = a.get();
    hehe = b.replace(hehe);
    a.replace(hehe);
}

#[allow(dead_code)]
fn swap_string(a: &Cell<String>, b: &Cell<String>)
{
    let mut hehe = a.take();
    hehe = b.replace(hehe);
    a.replace(hehe);
}

#[cfg(test)]
#[test]
fn test_u32()
{
    let a = Cell::new(1);
    let b = Cell::new(3);

    swap_u32(&a, &b);

    assert_eq!(a.get(), 3);
    assert_eq!(b.get(), 1);
}

#[cfg(test)]
#[test]
fn test_string()
{
    let a = Cell::new("ABC".into());
    let b = Cell::new("DEF".into());

    swap_string(&a, &b);

    assert_eq!(a.into_inner(), "DEF");
    assert_eq!(b.into_inner(), "ABC");
}
