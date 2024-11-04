/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/09 18:50:06 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/09 18:53:07 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

type Seconds = f32;
type Minutes = f32;

fn seconds_to_minutes(seconds: Seconds) -> Minutes {
    seconds / 60.0
}

fn main() {
    let s: Seconds = 120.0;
    let m: Minutes = seconds_to_minutes(s);

    println!("{s} seconds is {m} minutes");
}