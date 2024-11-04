/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/10 16:57:29 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/10 17:02:25 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T
{
    if a < b
    {
        a
    }
    else
    {
        b
    }
}

#[test]
#[cfg(test)]
fn nothing_test()
{
    assert_eq!(min(12i32, -14i32), -14);
    assert_eq!(min(12f32, 14f32), 12f32);
    assert_eq!(min("abc", "def"), "abc");
    assert_eq!(min(String::from("abc"), String::from("def")), "abc");
}