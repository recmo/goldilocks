// Goldilocks field elements are represented as a pair of u32s in little-endian order.
type Field = vec2<u32>;

// The Goldilocks modulus
const MODULUS = vec2<u32>(0xffffffffu, 0x00000001u);

// The 2^64 mod MODULUS
const EPSILON = 0xffffffffu;

// The constant zero
const ZERO = Field(0u,0u);

// The constant one
const ONE = Field(1u,0u);

fn reduce(n: vec4<u32>) -> vec2<u32> {
    // Compute 
    // n.x + n.y * 2^32 + n.z * 2^64 + n.w * 2^96 mod p
    // which equals
    // n.x - n.z - n.w + (n.y + n.z) * 2^32 mod p

    var r = n.xy;

    // Substract n.z
    if (r.x < n.z) {
        if (r.y == 0u) {
            // Add p
            r.x += 1u; // Can not overflow
            r.y = EPSILON;
        }
        r.y -= 1u;
    }
    r.x -= n.z;

    // Substract n.w
    if (r.x < n.w) {
        if (r.y == 0u) {
            // Add p
            r.x += 1u; // Can not overflow
            r.y = EPSILON;
        }
        r.y -= 1u;
    }
    r.x -= n.w;

    // Add n.z * 2^32
    r.y += n.z;
    if (r.y < n.z) {
        // Add 2**64 mod p = 0xffffffff
        r.x += EPSILON;
        if (r.x < EPSILON) {
            r.y += 1u; // Can not overflow
        }
    }

    // Reduce mod p
    if (r.y == EPSILON && r.x != 0u) {
        r.y = 0u;
        r.x -= 1u; // Can not underflow
    }

    return r;
}

fn add(a: Field, b: Field) -> Field {
    var r = a + b;
    r.y += u32(r.x < a.x);
    if (r.y < a.y) {
        // Subtract 2^64 mod p
        r.y += u32(r.x == 0u);
        r.x -= 1u;
    }
    return r;
}

fn sub(a: Field, b: Field) -> Field {
    var r = a - b;
    r.y -= u32(r.x > a.x);
    if (r.y > a.y) {
        // Add 2^64 mod p
        r.x += 1u;
        r.y -= u32(r.x != 0u);
    }
    return r;
}

fn mul(a: Field, b: Field) -> Field {
    return reduce(mul128(a, b));
}

fn shift(a: Field, s: u32) -> Field {
    if (s == 0u) {
        return a;
    } else if (s <= 32u) {
        let f = vec4<u32>(a.x >> s, (a.x << (32u - s)) | (a.y >> s), a.y << (32u - s), 0u);
        return reduce(f);
    } else {
        let s = s - 32u;
        let f = vec4<u32>(0u, a.x >> s, (a.x << (32u - s)) | (a.y >> s), a.y << (32u - s));
        return reduce(f);
    }
}
