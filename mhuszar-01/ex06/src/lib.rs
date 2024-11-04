/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/08 19:22:38 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/08 23:21:09 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
fn big_add(a: &[u8], b: &[u8]) -> Vec<u8>
{
    let mut ctr = 0;
    let mut ctr2 = 0;
    let mut remainder = 0;
    if a.is_empty() || b.is_empty()
    {
        panic!("input empty!");
    }
    while ctr < a.len()
    {
        if !a[ctr].is_ascii_digit()
        {
            panic!("not ascii input!");
        }
        ctr += 1;
    }
    ctr = a.len() - 1;
    while ctr2 < b.len()
    {
        if !b[ctr2].is_ascii_digit()
        {
            panic!("not ascii input!");
        }
        ctr2 += 1;
    }
    ctr2 = b.len() - 1;
    let mut result: Vec<u8> = Vec::new();
    loop 
    {
        result.push((((a[ctr] - 48) + (b[ctr2] - 48) + remainder) % 10) + 48);
        remainder = ((a[ctr] - 48) + (b[ctr2] - 48) + remainder) / 10;
        if ctr2 == 0 || ctr == 0
        {
            break ;
        }
        ctr -= 1;
        ctr2 -= 1;
    }
    if ctr2 == 0 && ctr == 0
    {
        result.push(remainder + 48);
    }
    loop
    {
        if ctr == 0
        {
            break ;
        }
        ctr -= 1;
        result.push((((a[ctr] - 48) + remainder) % 10) + 48);
        remainder = ((a[ctr] - 48) + remainder) / 10;
    }
    loop
    {
        if ctr2 == 0
        {
            break ;
        }
        ctr2 -= 1;
        result.push((((b[ctr2] - 48) + remainder) % 10) + 48);
        remainder = ((b[ctr2] - 48) + remainder) / 10;
    }
    ctr = result.len() - 1;
    while result[ctr] == 48
    {
        if ctr == 0
        {
            break ;
        }
        ctr -= 1;
    }
    let mut result2: Vec<u8> = Vec::new();
    loop
    {
        result2.push(result[ctr]);
        if ctr == 0
        {
            break;
        }
        ctr -= 1;
    }
    result2
}

// fn main()
// {
//     big_add(b"1234500", b"455500");
// }

#[test]
#[cfg(test)]
fn test_add()
{
    assert_eq!(big_add(b"8", b"4"), b"12");
    assert_eq!(big_add(b"2345", b"4555"), b"6900");
    assert_eq!(big_add(b"1234500", b"455500"), b"1690000");
    assert_eq!(big_add(b"0010", b"0200"), b"210");
    assert_eq!(big_add(b"00000", b"00"), b"0");
    assert_eq!(big_add(b"0", b"12"), b"12");
    assert_eq!(big_add(b"5000", b"5000"), b"10000");
    assert_eq!(big_add(b"42", b"59"), b"101");
}

#[test]
#[cfg(test)]
#[should_panic]
fn add_empty()
{
    big_add(b"", b"");
}

#[test]
#[cfg(test)]
#[should_panic]
fn add_wrong()
{
    big_add(b"999", b" ");
}
