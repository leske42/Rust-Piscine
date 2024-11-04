/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   collatz.rs                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/07 11:25:00 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/07 14:54:26 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// #![allow(dead_code)]

fn collatz(start: u32)
{
    let mut n: u32 = start;
    if n == 0
    {
        return ;
    }
    std::println!("{}", n);
    while n != 1
    {
        if n % 2 != 0
        {
            n = (n * 3) + 1;
            std::println!("{}", n);
        }
        else
        {
            n /= 2;
            std::println!("{}", n);
        }
    }
}

// fn main()
// {
//     collatz(100);
// }