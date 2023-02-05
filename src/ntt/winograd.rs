use crate::Field;

/// Winograd NTT for 5 elements.
/// 
/// TODO. TACP p. 710.
/// 
/// 17 additions, 5 multiplications.
pub fn ntt_5() {
    debug_assert_eq!(values.len() % 5, 0);
    for values in values.chunks_exact_mut(5) {
        let a0 = values[0];
        let a1 = values[1];
        let a2 = values[2];
        let a3 = values[3];
        let a4 = values[4];

        let s1 = a1 + a3;
        let s2 = a2 + a4;
        let s3 = s1 + s2;

        let t = a0;
        let a0 = a0 + s3;

        let s4 = s1 - s2;
        let s5 = a1 - a3;
        let s6 = a4 - a2;
        let s7 = s5 - s6;

        let m1 = s3 * k1;
        let m2 = s4 * k2;
        let m3 = s5 * k3;
        let m4 = s6 * k4;
        let m5 = s7 * k5;

        let m1 = m1 + t;

        let t1 = m1 + m2;
        let t2 = m3 + m5;
        let t3 = m1 - m2;
        let t4 = m4 - m5;

        let a1 = t1 + t2;
        let a2 = t3 + t4;
        let a3 = t1 - t2;
        let a4 = t3 - t4;

        values[0] = a0;
        values[1] = a1;
        values[2] = a2;
        values[3] = a3;
        values[4] = a4;
    }
}

/// Rader NTT for 5 elements.
/// 
/// 18 additions, 5 multiplications and 2 shifts.
pub fn rader_5(values: &mut [Field]) {
    debug_assert_eq!(values.len() % 5, 0);
    for values in values.chunks_exact_mut(5) {
        let a0 = values[0];
        let a1 = values[1];
        let a2 = values[2];
        let a3 = values[3];
        let a4 = values[4];
        let (a1, a4) = (a1 + a4, a1 - a4);
        let (a3, a2) = (a3 + a2, a3 - a2);
        let a2 = (a2 << 48);
        let (a1, a3) = (a1 + a3, a1 - a3);
        let (a4, a2) = (a4 + a2, a4 - a2);
        let t = a0;
        let a0 = a0 + a1;
        let a1 = a1 * Field::new(4611686017353646080);
        let a4 = a4 * Field::new(16181989089180173841);
        let a3 = a3 * Field::new(5818851782451133869);
        let a2 = a2 * Field::new(11322249509082494407);
        let a1 = a1 + t;
        let (a1, a3) = (a1 + a3, a1 - a3);
        let (a2, a4) = (a2 + a4, a2 - a4);
        let a4 = (a4 << 48);
        let (a1, a2) = (a1 + a2, a1 - a2);
        let (a3, a4) = (a3 + a4, a3 - a4);
        values[0] = a0;
        values[1] = a1;
        values[2] = a3;
        values[3] = a4;
        values[4] = a2;
    }
}
