/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   overflow.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/07 15:48:46 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/07 17:49:48 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


fn main()
{
    let mut hehe: u8 = 250;
    for n in 1..20
    {
        hehe += 1;
        if n == 6
        {
            break ;
        }
    }
    std::println!("255u8 + 1u8 == {hehe}");
}