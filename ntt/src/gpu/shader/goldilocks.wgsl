// TODO: Rewrite when <https://github.com/gpuweb/gpuweb/issues/1565> resolves.

// Goldilocks field elements are represented as a pair of u32s in little-endian order.
type Gold = vec2<u32>;

// The Goldilocks modulus
let MODULUS = vec2<u32>(0xffffffffu, 0x00000001u);

// The 2^64 mod MODULUS
let EPSILON = 0xffffffffu;

// The constant zero
let ZERO = Gold(0u,0u);

// The constant one
let ONE = Gold(1u,0u);

////////////////////////////////////
// U T I L I T I E S              //
////////////////////////////////////

// See <https://github.com/google/angle/blob/main/src/compiler/translator/BuiltInFunctionEmulatorHLSL.cpp#L69>

// Compute the top 32 bit of the addition
fn hadd(a: u32, b: u32) -> u32 {
    return (a >> 1u) + (b >> 1u) + ((a & b) & 1u);
}

fn mul64(a: u32, b: u32) -> vec2<u32> {
    // Split into 16 bit parts
    var a0 = (a << 16u) >> 16u;
    var a1 = a >> 16u;
    var b0 = (b << 16u) >> 16u;
    var b1 = b >> 16u;

    // Compute 32 bit half products
    // Each of these is at most 0xfffe0001
    var a0b0 = a0 * b0;
    var a0b1 = a0 * b1;
    var a1b0 = a1 * b0;
    var a1b1 = a1 * b1;

    // Sum the half products
    var r: vec2<u32>;
    r.x = a0b0 + (a1b0 << 16u) + (a0b1 << 16u);
    r.y = a1b1 + (hadd((a0b0 >> 16u) + a0b1, a1b0) >> 15u);
    return r;
}

fn mul128(a: vec2<u32>, b: vec2<u32>) -> vec4<u32> {
    // Compute 64 bit half products
    // Each of these is at most 0xfffffffe00000001
    var a0b0 = mul64(a.x, b.x);
    var a0b1 = mul64(a.x, b.y);
    var a1b0 = mul64(a.y, b.x);
    var a1b1 = mul64(a.y, b.y);

    var r = vec4<u32>(a0b0, a1b1);

    // Add a0b1
    r.y += a0b1.x;
    if (r.y < a0b1.x) {
        a0b1.y += 1u; // Can not overflow
    }
    r.z += a0b1.y;
    if (r.z < a0b1.y) {
        r.w += 1u;
    }

    // Add a1b0
    r.y += a1b0.x;
    if (r.y < a1b0.x) {
        a1b0.y += 1u; // Can not overflow
    }
    r.z += a1b0.y;
    if (r.z < a1b0.y) {
        r.w += 1u;
    }

    return r;
}

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
            r.y = 0xffffffffu;
        }
        r.y -= 1u;
    }
    r.x -= n.z;

    // Substract n.w
    if (r.x < n.w) {
        if (r.y == 0u) {
            // Add p
            r.x += 1u; // Can not overflow
            r.y = 0xffffffffu;
        }
        r.y -= 1u;
    }
    r.x -= n.w;

    // Add n.z * 2^32
    r.y += n.z;
    if (r.y < n.z) {
        // Add 2**64 mod p = 0xffffffff
        r.x += 0xffffffffu;
        if (r.x < 0xffffffffu) {
            r.y += 1u; // Can not overflow
        }
    }

    // Reduce mod p
    if (r.y == 0xffffffffu && r.x != 0u) {
        r.y = 0u;
        r.x -= 1u; // Can not underflow
    }

    return r;
}

///////////////////////////////////////////////////
// F I E L D   O P E R A T I O N S               //
///////////////////////////////////////////////////

// Addition in the Goldilocks field
fn add(a: Gold, b: Gold) -> Gold {
    var r = a + b;
    if (r.x < a.x) {
        r.y += 1u;
    }
    if (r.y < a.y) {
        r.x += EPSILON;
        if (r.x < EPSILON) {
            r.y += 1u;
        }
    }
    return r;
}

fn mul(a: Gold, b: Gold) -> Gold {
    return reduce(mul128(a, b));
}
