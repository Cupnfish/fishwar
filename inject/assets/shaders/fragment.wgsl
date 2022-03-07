struct type_3 {
    member: f32;
    member_1: f32;
    member_2: f32;
    member_3: f32;
    member_4: vec4<f32>;
    member_5: f32;
};

struct type_7 {
    member: type_3;
};

var<private> global: vec2<f32>;
var<private> global_1: vec4<f32>;
@group(1) @binding(0) 
var<uniform> global_2: type_7;
var<private> global_3: vec4<f32>;

fn function_() {
    let _e17 = global;
    let _e18 = global_1;
    let _e24 = global_2.member.member_4[3u];
    if (_e24 == 0.0) {
    } else {
        let _e28 = global_2.member.member_2;
        let _e31 = global_2.member.member_5;
        let _e35 = global_2.member.member;
        let _e38 = global_2.member.member_1;
        let _e46 = global_2.member.member_3;
        if (_e17.y < ((_e35 * sin(((_e38 + _e17.x) + (_e28 * _e31)))) + _e46)) {
        } else {
            let _e50 = global_2.member.member_4;
            let _e60 = (1.0 / sqrt((((_e18.x * _e18.x) + (_e18.y * _e18.y)) + (_e18.z * _e18.z))));
            global_3 = (_e50 + vec4<f32>((((_e18.x * _e60) - _e50.x) * 0.20000000298023224), (((_e18.y * _e60) - _e50.y) * 0.20000000298023224), (((_e18.z * _e60) - _e50.z) * 0.20000000298023224), ((1.0 - _e50.w) * 0.20000000298023224)));
        }
    }
    return;
}

@stage(fragment) 
fn fragment(@location(2) param: vec2<f32>, @builtin(position) param_1: vec4<f32>) -> @location(0) vec4<f32> {
    global = param;
    global_1 = param_1;
    function_();
    let _e5 = global_3;
    return _e5;
}
