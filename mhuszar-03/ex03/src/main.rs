/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/10 18:58:46 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/10 20:00:54 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fmt::Debug;

trait FortyTwo
{
    fn forty_two() -> Self;
}

impl FortyTwo for u32
{
    fn forty_two() -> Self
    {
        42
    }
}

impl FortyTwo for String
{
    fn forty_two() -> Self
    {
        "Forty Two".to_string()
    }
}

fn print_forty_two<T: Debug + FortyTwo>()
{
    std::println!("{:?}", T::forty_two());
}

fn main()
{
    print_forty_two::<u32>();
    print_forty_two::<String>();
}