@group(0)
@binding(0)
var<storage, read_write> v_indices: array<Gold>; // this is used as both input and output for convenience

fn factorial(n: Gold) -> Gold {
    var i = ONE;
    var r = ONE;
    loop {
        if (i.x > n.x) {
            break;
        }
        r = mul(r, i);
        i.x += 1u;
    }
    return r;
}

@stage(compute)
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    v_indices[global_id.x] = factorial(v_indices[global_id.x]);
}
