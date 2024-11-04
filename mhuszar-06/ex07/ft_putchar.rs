/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ft_putchar.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/13 20:39:12 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/13 21:45:07 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![no_std]
#![no_main]
#![no_implicit_prelude]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> !
{
    ft_exit(101);
}

use core::arch::asm;

fn ft_putchar(c: u8)
{
    unsafe
    {
        asm!(
            "xor rsi, rsi",
            "mov rsi, {}",
            "mov rdi, 1",
            "mov rdx, 1",
            "mov rax, 1",
            "syscall",
            in(reg) &c,
        );
    }
}

fn ft_exit(_code: u8) -> !
{
    loop
    {
        unsafe 
        {
            asm!(
                "mov rax, 60",
                "syscall",
            );
        }
    }
}

#[no_mangle]
pub extern "C" fn _start()
{
    ft_putchar(b'4');
    ft_putchar(b'2');
    ft_putchar(b'\n');
    ft_exit(42);
}

// fn main()
// {
//     ft_putchar(b'4');
//     ft_putchar(b'2');
//     ft_putchar(b'\n');
//     ft_exit(42);
// }