use crate::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut tmp_a = a;
    let mut tmp_b = b;
    let mut result = 0;
    while tmp_a != 0 {
	if tmp_a & 1 == 1 {
	  result = adder::adder(result, tmp_b);
        }
        tmp_a = tmp_a >> 1;
        tmp_b = tmp_b << 1;
    }
    result
}

pub fn int_multiplier(a: i32, b: i32) -> i32 {
    let mut tmp_a = a;
    let mut tmp_b = b;
    let mut result = 0;
    for _ in 0..32 {
	if tmp_a & 1 == 1 {
          result = adder::int_adder(result, tmp_b);
        }
        tmp_a = tmp_a >> 1;
        tmp_b = tmp_b << 1;
    }
    result
}

/**********************************/
/*           Example              */
/*                                */
/* A = 00000101 (5)               */
/* B = 00010110 (22)              */
/*                                */
/* A * B =   1 *      10110       */
/*         + 0 *     101100       */
/*         + 1 *    1011000       */
/*                                */
/*       =          1101110 (110) */
/*                                */
/*                                */
/**********************************/

/**********************************/
/*          Complexity            */
/*                                */
/* Time complexity: O(1)          */
/* Space complexity: O(1)         */
/**********************************/
