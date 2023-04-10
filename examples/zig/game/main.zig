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

    updateEnemy(&enemies[0], &redBallSprite);
    updateEnemy(&enemies[1], &greenBallSprite);
    updateEnemy(&enemies[2], &blueBallSprite);
}

const EntityState = struct { x: i32, y: i32, dir: u2 };
var enemies: [3]EntityState = .{
    .{ .x = 16, .y = 16, .dir = 1 },
    .{ .x = 16 * 18, .y = 16, .dir = 3 },
    .{ .x = 16, .y = 16 * 12, .dir = 1 },
};

fn updateEnemy(enemy: *EntityState, sprite: [*]u8) void {
    switch (enemy.dir) {
        0 => enemy.y -= 1,
        1 => enemy.x += 1,
        2 => enemy.y += 1,
        3 => enemy.x -= 1,
    }
    if (((enemy.x | enemy.y) & 15) == 0) {
        const tx = @intCast(usize, enemy.x) / 16;
        const ty = @intCast(usize, enemy.y) / 16;
        var dir = enemy.dir;
        var count: u32 = 0;
        if (enemy.dir != 2 and levelData[ty - 1][tx] == ' ') {
            dir = 0;
            count += 1;
        }
        if (enemy.dir != 3 and levelData[ty][tx + 1] == ' ') {
            count += 1;
            if (uw8.random() % count == 0) {
                dir = 1;
            }
        }
        if (enemy.dir != 0 and levelData[ty + 1][tx] == ' ') {
            count += 1;
            if (uw8.random() % count == 0) {
                dir = 2;
            }
        }
        if (enemy.dir != 1 and levelData[ty][tx - 1] == ' ') {
            count += 1;
            if (uw8.random() % count == 0) {
                dir = 3;
            }
        }
        enemy.dir = dir;
    }

    uw8.blitSprite(sprite, 16, 8 + enemy.x, enemy.y, 0x100);
}
