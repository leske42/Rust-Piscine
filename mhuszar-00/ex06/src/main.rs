/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/07 22:07:19 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/07 22:14:08 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn main() {
    let target: i32 = ftkit::random_number(0..100);
    std::println!(
        "Me and my infinite wisdom have found an appropriate secret you shall yearn for."
    );
    let mut hehe: i32 = ftkit::read_number();
    loop {
        match target.cmp(&hehe)
        {
            std::cmp::Ordering::Less => std::println!("Sometimes I wonder whether I should retire. I would have guessed higher."),
            std::cmp::Ordering::Equal => { std::println!("That is right! The secret was indeed the number {target}, which you have brilliantly discovered!"); break; },
            std::cmp::Ordering::Greater => std::println!("This student might not be as smart as I was told. This answer is obviously too weak.")
        }
        hehe = ftkit::read_number();
    }
}
