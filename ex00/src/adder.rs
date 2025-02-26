pub fn int_adder(a: i32, b: i32) -> i32 {
    let mut sum = a ^ b;
    let mut carry = (a & b) << 1;
    while carry != 0 {
        let next_sum = sum ^ carry;
        carry = (sum & carry) << 1;
        sum = next_sum;
    }
    sum
}

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

/**********************************/
/*           Example              */
/*                                */
/* A = 01100101 (101)             */
/* B = 00010110 (22)              */
/*                                */
/* A ^ B = 01110011 (115)         */
/* A & B = 00000100 (4)           */
/* A & B << 1 = 0000100 (8)       */
/*                                */
/* A + B = A ^ B + (A & B << 1)   */
/*       = 115 + 8                */
/*       = 223                    */
/**********************************/

/**********************************/
/*          Complexity            */
/*                                */
/* Time complexity: O(n)          */
/* Space complexity: O(1)         */
/**********************************/
