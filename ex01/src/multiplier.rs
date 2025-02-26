pub fn adder(a: u32, b: u32) -> u32 {
    let mut sum = a ^ b;
    let mut carry = (a & b) << 1;
    while carry != 0 {
        let next_sum = sum ^ carry;
        carry = (sum & carry) << 1;
        sum = next_sum;
    }
    sum
}

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut tmp_a = a;
    let mut tmp_b = b;
    let mut result = 0;
    while tmp_a != 0 {
	if tmp_a & 1 == 1 {
	  result = adder(result, tmp_b);
        }
        tmp_a = tmp_a >> 1;
        tmp_b = tmp_b << 1;
    }
    result
}

/**********************************/
/*           Example              */
/*                                */
/* A =   101 (5)                  */
/* B = 10110 (22)                 */
/*                                */
/* A * B =   1 *      10110       */
/*         + 0 *     101100       */
/*         + 1 *    1011000       */
/*                                */
/*       =          1101110 (110) */
/*                                */
/*                                */
/**********************************/

/*************************************/
/*          Complexity               */
/*                                   */
/* Time complexity: O(log(a)*log(b)) */
/* Space complexity: O(1)            */
/*************************************/
