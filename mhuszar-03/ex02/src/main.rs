/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/10 17:03:22 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/10 21:27:44 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::fmt;
use std::fmt::Formatter;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Binary;

struct John;

impl Display for John
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        match f.align() {
            Some(fmt::Alignment::Left) => return write!(f, "{:<30}", format!("Hey! I'm John.                ")),
            Some(fmt::Alignment::Right) => return write!(f, "{:>30}", format!("                Hey! I'm John.")),
            Some(fmt::Alignment::Center) => return write!(f, "{:^30}", format!("        Hey! I'm John.        ")),
            _ => ()
        }
        let prec = f.precision();
        match prec
        {
            Some(6) => return write!(f, "Hey! I"),
            Some(0) => return write!(f, "Don't try to silence me!"),
            _ => ()
        }
        write!(f, "Hey! I'm John.")
    }
}

impl Debug for John
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        if f.alternate()
        {
            write!(f, "John, the man himself. He's handsome AND formidable.")
        }
        else
        {
            write!(f, "John, the man himself.")
        }
    }
}

impl Binary for John
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "Bip Boop?")
    }
}

fn main()
{
    let john = John;

    println!("1. {john}");
    println!("2. |{john:<30}|");
    println!("3. |{john:>30}|");
    println!("4. {john:.6}");
    println!("5. {john:.0}");
    println!("6. {john:?}");
    println!("7. {john:#?}");
    println!("8. {john:b}");
}
