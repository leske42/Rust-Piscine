/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/10 20:04:24 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/11 00:05:01 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::str::FromStr;
use std::fmt::{Display, Debug, Formatter};

struct Time {
    hours: u32,
    minutes: u32,
}

#[derive(Debug)]
enum TimeParseError {
    MissingColon,
    InvalidLength,
    InvalidNumber,
}

impl FromStr for Time
{
    type Err = TimeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s.len() != 5
        {
            return Err(TimeParseError::InvalidLength);
        }
        let bytes = s.as_bytes();
        match (bytes[0].is_ascii_digit(), bytes[1].is_ascii_digit(), bytes[2] == b':', bytes[3].is_ascii_digit(), bytes[4].is_ascii_digit())
        {
            (true, true, true, true, true) => (),
            (_, _, false, _, _) => return Err(TimeParseError::MissingColon),
            _ => return Err(TimeParseError::InvalidNumber)
        }
        let hours = (bytes[0] as u32 - 48) * 10 + (bytes[1] as u32 - 48);
        let minutes = (bytes[3] as u32 - 48) * 10 + (bytes[4] as u32 - 48);
        if hours >= 24 || minutes >= 60
        {
            return Err(TimeParseError::InvalidNumber);
        }
        Ok(Time { hours, minutes })
    }
}

impl Display for Time
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
    {
        match (self.hours, self.minutes)
        {
            (1, 1) => write!(f, "{} hour, {} minute", self.hours, self.minutes),
            (_, 1) => write!(f, "{} hours, {} minute", self.hours, self.minutes),
            (1, _) => write!(f, "{} hour, {} minutes", self.hours, self.minutes),
            _ => write!(f, "{} hours, {} minutes", self.hours, self.minutes)
        }
    }
}

impl Debug for Time
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
    {
        write!(f, "")
    }
}

impl Display for TimeParseError
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result
    {
        match self
        {
            TimeParseError::MissingColon => write!(f, "missing ':'"),
            TimeParseError::InvalidLength => write!(f, "invalid length"),
            TimeParseError::InvalidNumber => write!(f, "invalid number"),
        }
    }
}

fn main() {
    let a: Time = "00:01".parse().unwrap();
    let b: Time = "15:14".parse().unwrap();

    println!("{a}");
    println!("{b}");

    let err1: TimeParseError = "12.20".parse::<Time>().unwrap_err();
    let err2: TimeParseError = "12:2".parse::<Time>().unwrap_err();
    let err3: TimeParseError = "12:2a".parse::<Time>().unwrap_err();
    println!("error: {err1}");
    println!("error: {err2}");
    println!("error: {err3}");
}