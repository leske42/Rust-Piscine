/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/08 13:19:03 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/08 15:26:08 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
const fn color_name(color: &[u8; 3]) -> &'static str
{
    match color
    {
        [0, 0, 0] => "pure black",
        [255, 255, 255] => "pure white",
        [255, 0, 0] => "pure red",
        [0, 255, 0] => "pure green",
        [0, 0, 255] => "pure blue",
        [128, 128, 128] => "perfect grey",
        [red, green, blue] => {
            match [*red < 31, *green < 31, *blue < 31]
            {
                [true, true, true] => "almost black",
                _ => {
                    match [*red < 128, *green < 128, *blue < 128]
                    {
                        [false, true, true] => { match *red > 128 { false => "unknown", _ => "redish"} },
                        [true, false, true] => { match *green > 128 { false => "unknown", _ => "greenish"} },
                        [true, true, false] => { match *blue > 128 { false => "unknown", _ => "blueish"} },
                        _ => "unknown"
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_lifetimes()
{
    let name_of_the_best_color;
    {
        let the_best_color = [42, 42, 42];
        name_of_the_best_color = color_name(&the_best_color);
    }
    assert_eq!(name_of_the_best_color, "unknown");
}

#[cfg(test)]
mod tests
{
    #[test]
    fn color_test()
    {
        let result = crate::color_name(&[0, 0, 0]);
        assert_eq!(result, "pure black");
        let result = crate::color_name(&[255, 255, 255]);
        assert_eq!(result, "pure white");
        let result = crate::color_name(&[255, 0, 0]);
        assert_eq!(result, "pure red");
        let result = crate::color_name(&[0, 0, 255]);
        assert_eq!(result, "pure blue");
        let result = crate::color_name(&[0, 255, 0]);
        assert_eq!(result, "pure green");
        let result = crate::color_name(&[128, 128, 128]);
        assert_eq!(result, "perfect grey");
        let result = crate::color_name(&[20, 20, 20]);
        assert_eq!(result, "almost black");
        let result = crate::color_name(&[170, 20, 20]);
        assert_eq!(result, "redish");
        let result = crate::color_name(&[0, 200, 20]);
        assert_eq!(result, "greenish");
        let result = crate::color_name(&[99, 2, 255]);
        assert_eq!(result, "blueish");
        let result = crate::color_name(&[0, 0, 128]);
        assert_eq!(result, "unknown");
    }
}