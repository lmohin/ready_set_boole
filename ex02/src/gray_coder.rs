pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

/**********************************/
/*           Example              */
/*                                */
/* n = 00100101 (37)              */
/* n >> 1 = n / 2 = 00010010 (18) */
/*                                */
/* n ^ (n >> 1) =   00100101      */
/*                ^ 00010010      */
/*                                */
/*              =   00110111 (55) */
/*                                */
/**********************************/

/**********************************/
/*     Gray Code first values     */
/*                                */
/*     000 -> 000                 */
/*     001 -> 001                 */
/*     010 -> 011                 */
/*     011 -> 010                 */
/*     100 -> 110                 */
/*     101 -> 111                 */
/*     110 -> 101                 */
/*     111 -> 100                 */
/*                                */
/**********************************/

/**********************************/
/*          Complexity            */
/*                                */
/* Time complexity: O(1)          */
/* Space complexity: O(1)         */
/*                                */
/**********************************/
