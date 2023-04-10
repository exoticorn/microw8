extern fn atan2(x: f32, y: f32) f32;
extern fn time() f32;

pub const FRAMEBUFFER: *[320 * 240]u8 = @intToPtr(*[320 * 240]u8, 120);

export fn upd() void {
    var i: u32 = 0;
    while (true) {
        var t = time() * 63.0;
        var x = @intToFloat(f32, (@intCast(i32, i % 320) - 160));
        var y = @intToFloat(f32, (@intCast(i32, i / 320) - 120));
        var d = 40000.0 / @sqrt(x * x + y * y);
        var u = atan2(x, y) * 512.0 / 3.141;
        var c = @intCast(u8, (@floatToInt(i32, d + t * 2.0) ^ @floatToInt(i32, u + t)) & 255) >> 4;

        FRAMEBUFFER[i] = c;
        i += 1;
        if (i >= 320 * 240) {
            break;
        }
    }
}
