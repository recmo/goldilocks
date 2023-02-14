/// See <http://www.bealto.com/gpu-fft_group-2.html>
/// See <https://github.com/filecoin-project/ec-gpu/blob/master/ec-gpu-gen/src/cl/fft.cl>
/// See <https://github.com/DTolm/VkFFT> <https://ieeexplore.ieee.org/document/10036080>
/// See <https://github.com/philipturner/metal-fft>
/// See <https://developer.apple.com/library/archive/samplecode/OpenCL_FFT/Introduction/Intro.html>

// Input/Output buffer
@group(0)
@binding(0)
var<storage, read_write> values: array<Field>;


// NTT implementation
const WOKRGROUP_SIZE: vec3<u32> = vec3<u32>(1u, 1u, 1u);
@compute
@workgroup_size(1u, 1u, 1u)
fn main(
    @builtin(num_workgroups) num_workgroups: vec3<u32>,
    @builtin(global_invocation_id) global_id: vec3<u32>,
) {
    // Compute our place in the world
    let workgroup_index = global_id / WOKRGROUP_SIZE;
    let len = num_workgroups.x * num_workgroups.y * num_workgroups.z;
    let index = workgroup_index.x + workgroup_index.y * num_workgroups.x + workgroup_index.z * num_workgroups.x * num_workgroups.y;

    let a0 = values[4u * index + 0u];
    let a1 = values[4u * index + 1u];
    let a2 = values[4u * index + 2u];
    let a3 = values[4u * index + 3u];

    let b0 = add(a0, a2);
    let b1 = add(a1, a3);
    let b2 = sub(a0, a2);
    let b3 = sub(a1, a3);
    //let c3 = mul(b3, vec2<u32>(0x00000000u, 0x00010000u)); // 1 << 48
     let c3 = shift(b3, 48u);
    let d0 = add(b0, b1);
    let d1 = sub(b0, b1);
    let d2 = add(b2, c3);
    let d3 = sub(b2, c3);

    values[4u * index + 0u] = d0;
    values[4u * index + 1u] = d2;
    values[4u * index + 2u] = d1;
    values[4u * index + 3u] = d3;
}
