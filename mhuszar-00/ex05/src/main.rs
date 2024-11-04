/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/07 18:03:12 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/07 21:10:15 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// #[allow(dead_code)]

fn is_leap_year(year: u32) -> bool
{
    if year == 0
    {
        panic!("year cant be 0");
    }
    match (year % 4, year % 100, year % 400)
    {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        (_, _, _) => false
    }
}

fn num_days_in_month(year: u32, month: u32) -> u32
{
    if month == 2
    {
        if is_leap_year(year)
        {
            29
        }
        else
        {
            28
        }
    }
    else
    {
        match month
        {
            4 | 6 | 9 | 11 => 30,
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => panic!("wrong month")
        }
    }
}

fn main()
{
    let mut cur_wday: u32 = 1;
    let mut cur_day: u32 = 1;
    let mut max_day: u32;
    let mut cur_year: u32 = 1;
    let mut cur_mon: u32 = 1;
    let mut cur_umon: usize = 1;
    let arr: [&str; 13] = ["Hehe", "January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    while cur_year < 2024
    {
        while cur_mon < 13
        {
            max_day = num_days_in_month(cur_year, cur_mon);
            while cur_day <= max_day
            {
                if cur_wday % 7 == 5 && cur_day == 13
                {
                    println!("Friday, {} {cur_day}, {cur_year}", arr[cur_umon]);
                }
                cur_day += 1;
                cur_wday += 1;
            }
            cur_day = 1;
            cur_mon += 1;
            cur_umon += 1;
        }
        cur_mon = 1;
        cur_umon = 1;
        cur_year += 1;
    }
    println!("Friday, September 13, 2024");
    // println!("{}", num_days_in_month(1999, 20));
}

#[cfg(test)]
mod tests
{
    #[test]
    fn test1()
    {
        let result = crate::is_leap_year(1600);
        assert_eq!(result, true);
    }
    #[test]
    fn test2()
    {
        let result = crate::is_leap_year(1500);
        assert_eq!(result, false);
    }
    #[test]
    fn test3()
    {
        let result = crate::is_leap_year(2004);
        assert_eq!(result, true);
    }
    #[test]
    fn test4()
    {
        let result = crate::is_leap_year(2003);
        assert_eq!(result, false);
    }
    #[test]
    fn test5()
    {
        let mut result = crate::num_days_in_month(1600, 2);
        assert_eq!(result, 29);
        result = crate::num_days_in_month(1500, 2);
        assert_eq!(result, 28);
    }
    #[test]
    fn test6()
    {
        let mut result = crate::num_days_in_month(1600, 1);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1500, 1);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1600, 3);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1500, 3);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1600, 4);
        assert_eq!(result, 30);
        result = crate::num_days_in_month(1500, 4);
        assert_eq!(result, 30);
        result = crate::num_days_in_month(1600, 5);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1500, 5);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1600, 6);
        assert_eq!(result, 30);
        result = crate::num_days_in_month(1500, 6);
        assert_eq!(result, 30);
        result = crate::num_days_in_month(1600, 7);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1500, 7);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1600, 8);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1500, 8);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1600, 9);
        assert_eq!(result, 30);
        result = crate::num_days_in_month(1500, 9);
        assert_eq!(result, 30);
        result = crate::num_days_in_month(1600, 10);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1500, 10);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1600, 11);
        assert_eq!(result, 30);
        result = crate::num_days_in_month(1500, 11);
        assert_eq!(result, 30);
        result = crate::num_days_in_month(1600, 12);
        assert_eq!(result, 31);
        result = crate::num_days_in_month(1500, 12);
        assert_eq!(result, 31);
    }
    #[test]
    #[should_panic]
    fn test7()
    {
        crate::num_days_in_month(1600, 20);
    }
    #[test]
    #[should_panic]
    fn test8()
    {
        crate::is_leap_year(0);
    }
}
