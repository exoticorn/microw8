const uw8 = @import("uw8.zig");

var redBallSprite: [16 * 16]u8 = undefined;
var greenBallSprite: [16 * 16]u8 = undefined;
var blueBallSprite: [16 * 16]u8 = undefined;
var wallSprite: [24 * 24]u8 = undefined;

const SphereConfigStep = struct { size: u8, color: u8 };
// zig fmt: off
const redSphereConfig: [4]SphereConfigStep = .{
    .{ .size = 0, .color = 0x3d },
    .{ .size = 2, .color = 0x48 },
    .{ .size = 6, .color = 0x65 },
    .{ .size = 9, .color = 0x55 }
};
const greenSphereConfig: [4]SphereConfigStep = .{
    .{ .size = 0, .color = 0x7d },
    .{ .size = 2, .color = 0x88 },
    .{ .size = 6, .color = 0x96 },
    .{ .size = 9, .color = 0xa3 }
};
const blueSphereConfig: [4]SphereConfigStep = .{
    .{ .size = 0, .color = 0x2e },
    .{ .size = 2, .color = 0x19 },
    .{ .size = 6, .color = 0x17 },
    .{ .size = 9, .color = 0x24 }
};

const levelData: [14] *const [19]u8 = .{
  "xxxxxxxxxxxxxxxxxxx",
  "x      x   x      x",
  "x x xx   x   xx x x",
  "x x  xxx x xxx  x x",
  "x xx x       x xx x",
  "x xx   xxxxx   xx x",
  "x    x       x    x",
  "x xx xx xxx xx xx x",
  "x  x xx     xx x  x",
  "xx x    x x    x xx",
  "x  xx xxx xxx xx  x",
  "x xx   x   x   xx x",
  "x    x   x   x    x",
  "xxxxxxxxxxxxxxxxxxx",
};
// zig fmt: on

export fn start() void {
    blitSphere(&redBallSprite, &redSphereConfig);
    blitSphere(&greenBallSprite, &greenSphereConfig);
    blitSphere(&blueBallSprite, &blueSphereConfig);

    createWallSprite();
}

fn blitSphere(sprite: [*]u8, config: []const SphereConfigStep) void {
    for (config) |circle| {
        uw8.circle(8, 8, 8, circle.color);
        uw8.circle(5, 6, @intToFloat(f32, circle.size), 0);
        uw8.grabSprite(sprite, 16, 0, 0, 0x100);
    }
}

fn createWallSprite() void {
    uw8.cls(0xe4);
    var i: i32 = 0;
    while (i < 50) : (i += 1) {
        const x = uw8.randomf() * 16;
        const y = uw8.randomf() * 16;
        const radius = uw8.randomf() * 2 + 1;
        const c = @intCast(u8, (uw8.random() & 3)) + 0x95;
        var j: i32 = 0;
        while (j < 9) : (j += 1) {
            uw8.circle(x + @intToFloat(f32, @rem(j, 3) * 16), y + @intToFloat(f32, @divFloor(j, 3) * 16), radius, c);
        }
    }
    uw8.grabSprite(&wallSprite, 16, 16, 16, 0);
}

export fn upd() void {
    uw8.cls(0);

    var y: usize = 0;
    while (y < levelData.len) : (y += 1) {
        var x: usize = 0;
        while (x < levelData[y].len) : (x += 1) {
            if (levelData[y][x] == 'x') {
                uw8.blitSprite(&wallSprite, 16, 8 + @intCast(i32, x) * 16, @intCast(i32, y) * 16, 0);
            }
        }
    }

    uw8.blitSprite(&redBallSprite, 16, 100, 100, 0x100);
    uw8.blitSprite(&greenBallSprite, 16, 130, 100, 0x100);
    uw8.blitSprite(&blueBallSprite, 16, 160, 100, 0x100);
}
