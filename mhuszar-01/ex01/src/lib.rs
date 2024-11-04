/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/08 13:08:54 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/08 14:01:30 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
fn min<'alma>(a: &'alma i32, b: &'alma i32) -> &'alma i32
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

// fn main()
// {
//     let& c;
//     let b = 101;
//     {
//         let a = 30;
//         c = min(&a, &b);
//     }
//     // std::cout << *c << std::endl;
//     std::println!("{}", *c);
    
//     // let result = crate::min(&30, &101);
//     // assert_eq!(result, &30);
// }

#[cfg(test)]
mod tests
{
    #[test]
    fn min_test()
    {
        let result = crate::min(&30, &101);
        assert_eq!(result, &30);
    }
}