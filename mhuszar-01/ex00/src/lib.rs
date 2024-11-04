/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/08 12:57:02 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/08 13:07:36 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
fn add(a: &i32, b: i32) -> i32
{
    a + b
}

#[allow(dead_code)]
fn add_assign(a: &mut i32, b: i32)
{
    *a += b;
}


#[cfg(test)]
mod tests
{
    #[test]
    fn add_test()
    {
        let result = crate::add(&30, 101);
        assert_eq!(result, 131);
    }
    #[test]
    fn assign_test()
    {
        let mut a: i32 = 9;
        crate::add_assign(&mut a, 1500);
        assert_eq!(a, 1509);
    }
}