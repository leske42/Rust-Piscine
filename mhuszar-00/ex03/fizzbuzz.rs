/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   fizzbuzz.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/07 12:20:43 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/07 13:18:59 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#![allow(dead_code)]

fn main()
{
    for n in 1..101
    {
        match (n % 3, n % 5, n % 11)
        {
            (0, 0, _) => std::println!("fizzbuzz"),
            (0, _, _) => std::println!("fizz"),
            (_, 0, _) => std::println!("buzz"),
            (_, _, 3) => std::println!("FIZZ"),
            (_, _, 5) => std::println!("BUZZ"),
            (_, _, _) => std::println!("{n}"),
        }
        
    }
}