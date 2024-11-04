/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/13 14:19:36 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/13 20:32:37 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
use std::alloc::{alloc, dealloc, Layout, handle_alloc_error};
use std::ops::{Deref, DerefMut};
use std::clone::Clone;
// use std::marker::PhantomData;
// use std::ptr::{NonNull, drop_in_place};
use std::ptr::drop_in_place;
use std::ptr::read;

#[allow(dead_code)]
#[derive(Clone)]
struct Carton<T>
{
    value: *mut T
}

impl<T> Deref for Carton<T>
{
    // SAFETY:
    // 1.0 Well here i dereference a pointer but i already know its valid cause
    // i handled the alloc error in the allocation part (hopefully)
    type Target = T;
    fn deref(&self) -> &T
    {
        unsafe
        {
            &*self.value
        }
    }
}

impl<T> DerefMut for Carton<T>
{
    fn deref_mut(&mut self) -> &mut T
    {
        // SAFETY:
        // Look at safety 1.0
        unsafe
        {
            &mut *self.value
        }
    }
}

impl<T> Drop for Carton<T>
{
    fn drop(&mut self)
    {
        // SAFETY:
        // Look at safety 1.0
        unsafe
        {
            drop_in_place(self.value);
            let layout = Layout::new::<T>();
            dealloc(self.value as *mut u8, layout);
        }
    }
}

// #[allow(dead_code)]
// // # Safety:
// // dst has to have enough space to accomodate src. This function gives
// // 0 fucks if dst is shorter, it will try to copy out of bounds
// unsafe fn ft_strcpy(dst: *mut u8, src: *const u8)
// {
//     // SAFETY:
//     // this is very safe
//     unsafe
//     {
//         let mut ctr = 0;
//         while *src.add(ctr) != 0
//         {
//             *dst.add(ctr) = *src.add(ctr);
//             ctr += 1;
//         }
//     }
// }

#[allow(dead_code)]
impl<T> Carton<T>
// where 
//     T : Clone
{
    fn new(value: T) -> Self
    {
        // SAFETY:
        // I do some allocation but i protect it
        unsafe
        {
            let layout = Layout::new::<T>();
            let hehe = alloc(layout);
            if hehe.is_null()
            {
                handle_alloc_error(layout);
            }
            let a = hehe as *mut T;
            a.write(value);
            Carton { value : a }
        }
    }

    fn clone(&self) -> Self
    {
        // SAFETY:
        // Here I also do some allocation but i protect it again
        unsafe
        {
            let layout = Layout::new::<T>();
            let hehe = alloc(layout);
            if hehe.is_null()
            {
                handle_alloc_error(layout);
            }
            let a = hehe as *mut T;
            a.write(read(self.value));
            Carton { value : a }
        }
    }

    fn into_inner(self) -> T
    {
        // SAFETY:
        // Look at safety 1.0
        unsafe
        {
            // let layout = Layout::new::<T>();
            let a = read(self.value);
            // let a = (*self.value).clone();
            drop_in_place(self.value);
            // dealloc(self.value as *mut u8, layout);
            a
        }
    }
}

#[cfg(test)]
#[test]
fn test_carton()
{
    #[derive(Clone)]
    struct Point { x: u32, y: u32 }

    let point_in_carton = Carton::new(Point { x: 1, y: 2 });
    assert_eq!(point_in_carton.x, 1);
    assert_eq!(point_in_carton.y, 2);

    let mut another_point = point_in_carton.clone();
    another_point.x = 2;
    another_point.y = 3;
    assert_eq!(another_point.x, 2);
    assert_eq!(another_point.y, 3);
    assert_eq!(point_in_carton.x, 1);
    assert_eq!(point_in_carton.y, 2);
}