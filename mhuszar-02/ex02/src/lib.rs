/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   lib.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: mhuszar <mhuszar@student.42vienna.com>     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2024/10/09 19:43:52 by mhuszar           #+#    #+#             */
/*   Updated: 2024/10/09 20:18:11 by mhuszar          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[allow(dead_code)]
enum PizzaStatus {
    Ordered,
    Cooking,
    Cooked,
    Delivering,
    Delivered,
}

#[allow(dead_code)]
impl PizzaStatus {
    fn from_delivery_time(ordered_days_ago: u32) -> Self
    {
        match ordered_days_ago
        {
            ordered_days_ago if ordered_days_ago < 2 => PizzaStatus::Ordered,
            ordered_days_ago if ordered_days_ago < 7 => PizzaStatus::Cooking,
            ordered_days_ago if ordered_days_ago < 10 => PizzaStatus::Cooked,
            ordered_days_ago if ordered_days_ago < 17 => PizzaStatus::Delivering,
            _ => PizzaStatus::Delivered
        }
    }
    fn get_delivery_time_in_days(&self) -> u32
    {
        match self
        {
            PizzaStatus::Ordered => 17,
            PizzaStatus::Cooking => 15,
            PizzaStatus::Cooked => 10,
            PizzaStatus::Delivering => 7,
            PizzaStatus::Delivered => 0
        }
    }
}

#[test]
#[cfg(test)]
fn status_test()
{
    let mut hihi = PizzaStatus::from_delivery_time(99);
    assert_eq!(hihi as u32, 4);
    hihi = PizzaStatus::from_delivery_time(0);
    assert_eq!(hihi as u32, 0);
    hihi = PizzaStatus::from_delivery_time(2);
    assert_eq!(hihi as u32, 1);
    hihi = PizzaStatus::from_delivery_time(9);
    assert_eq!(hihi as u32, 2);
    hihi = PizzaStatus::from_delivery_time(15);
    assert_eq!(hihi as u32, 3);
}

#[test]
#[cfg(test)]
fn time_test()
{
    let mut hihi = PizzaStatus::from_delivery_time(99);
    assert_eq!(PizzaStatus::get_delivery_time_in_days(&hihi), 0);
    hihi = PizzaStatus::from_delivery_time(5);
    assert_eq!(PizzaStatus::get_delivery_time_in_days(&hihi), 15);
}