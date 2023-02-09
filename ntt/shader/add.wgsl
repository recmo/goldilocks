@group(0)
@binding(0)
var<storage, read_write> v_indices: array<vec2<u32>>; // this is used as both input and output for convenience

let modulus = vec2<u32>(0xffffffffu, 0x00000001u);
let epsilon = 0xffffffffu;
let zero = vec2<u32>(0u,0u);
let one = vec2<u32>(1u,0u);

// Addition mod modulus
fn add(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    var r = a + b;
    if (r.x < a.x) {
        r.y += 1u;
    }
    if (r.y < a.y) {
        r.x += epsilon;
        if (r.x < epsilon) {
            r.y += 1u;
        }
    }
    return r;
}

fn fibonacci(n_base: vec2<u32>) -> vec2<u32> {
    var n: u32 = n_base[0];
    var a: vec2<u32> = vec2<u32>(0u, 0u);
    var b: vec2<u32> = vec2<u32>(1u, 0u);
    loop {
        if (n <= 1u) {
            break;
        }
        var t = add(a, b);
        a = b;
        b = t;
        n -= 1u;
    }
    return b;
}

@stage(compute)
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices[global_id.x] = fibonacci(v_indices[global_id.x]);
}
