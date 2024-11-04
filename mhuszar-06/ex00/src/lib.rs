/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/13 12:55:12 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/13 16:43:22 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![forbid(unsafe_op_in_unsafe_fn)]
use std::ptr::{write, read};

#[allow(dead_code)]
fn ft_swap<T>(a: &mut T, b: &mut T)
{
    // SAFETY:
    // Whatever i read must be properly initalized & aligned, but since i
    // take these variables over from the safe part of the code i assume
    // compiler would cry anyway if somebody wants to pass me stupid code
    unsafe
    {
        let a_val = read(a);
        let b_val = read(b);
        write(a, b_val);
        write(b, a_val);
    }
}

#[allow(dead_code)]
// # Safety:
// Only use this strlen with a pointer to a proper null terminated string pls.
// otherwise it will iterate out of bounds thanks
unsafe fn ft_strlen(s: *const u8) -> usize
{
    // SAFETY:
    // this is very safe
    unsafe
    {
        let mut ctr = 0;
        while *s.add(ctr) != 0
        {
            ctr += 1;
        }
        ctr
    }
}

#[allow(dead_code)]
// # Safety:
// dst has to have enough space to accomodate src. This function gives
// 0 fucks if dst is shorter, it will try to copy out of bounds
unsafe fn ft_strcpy(dst: *mut u8, src: *const u8)
{
    // SAFETY:
    // this is very safe
    unsafe
    {
        let mut ctr = 0;
        while *src.add(ctr) != 0
        {
            *dst.add(ctr) = *src.add(ctr);
            ctr += 1;
        }
    }
}

#[cfg(test)]
#[test]
fn test_string()
{
    let mut a = String::from("Hello, World!");
    let mut b = String::from("Goodby, World!");
    ft_swap(&mut a, &mut b);
    assert_eq!(a, "Goodby, World!");
    assert_eq!(b, "Hello, World!");

    let s = b"Hello, World!\0";
    let hehe = s.as_ptr();
    // SAFETY:
    //  /* ... */
    let len = unsafe { ft_strlen(hehe) };
    assert_eq!(len, 13);

    let mut dst = [0u8; 14];
    // SAFETY:
    //  /* ... */
    unsafe { ft_strcpy(dst.as_mut_ptr(), hehe) };
    assert_eq!(s, b"Hello, World!\0");
}