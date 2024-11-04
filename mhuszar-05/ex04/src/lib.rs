/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/12 22:22:21 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/12 22:46:11 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::sync::atomic::{AtomicU8, Ordering};

#[allow(dead_code)]
static IDX: AtomicU8 = AtomicU8::new(0);

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
struct Unique(u8);

#[allow(dead_code)]
impl Unique
{
    pub fn new() -> Self
    {
        let val = IDX.fetch_add(1, Ordering::Relaxed);
        if val == 255
        {
            panic!("no more free numbers available!")
        }
        Unique(val)
    }
    pub fn clone(self) -> Self
    {
        let val = IDX.fetch_add(1, Ordering::Relaxed);
        if val == 255
        {
            panic!("no more free numbers available!")
        }
        Unique(val)
    }
}

// #[allow(dead_code)]
// fn main()
// {
//     let a = Unique::new();
//     let b = Unique::new();
//     let c = Unique::new();

//     println!("{a:?}");
//     println!("{b:?}");
//     println!("{c:?}");

//     let d = a.clone();
//     let e = c.clone();

//     println!("{d:?}");
//     println!("{e:?}");
// }