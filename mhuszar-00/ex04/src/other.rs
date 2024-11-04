/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   other.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/07 15:48:44 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/07 17:50:01 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn main()
{
    #[cfg(debug_assertions)]
    {
        println!("Hey! I'm the other bin target!");
    }
    #[cfg(not(debug_assertions))]
    {
        println!("Hey! I'm the other bin target!\nI'm in release mode!");
    }
}
