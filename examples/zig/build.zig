const std = @import("std");

pub fn build(b: *std.build.Builder) void {
    const mode = std.builtin.Mode.ReleaseSmall;

    const lib = b.addSharedLibrary("cart", "main.zig", .unversioned);
    lib.setBuildMode(mode);
    lib.setTarget(.{
        .cpu_arch = .wasm32,
        .os_tag = .freestanding,
        .cpu_features_add = std.Target.wasm.featureSet(&.{ .nontrapping_fptoint })
    });
    lib.import_memory = true;
    lib.initial_memory = 262144;
    lib.max_memory = 262144;
    lib.global_base = 81920;
    lib.stack_size = 8192;
    lib.install();

    if (lib.install_step) |install_step| {
        const run_filter_exports = b.addSystemCommand(&[_][]const u8{
            "uw8", "filter-exports", "zig-out/lib/cart.wasm", "zig-out/lib/cart-filtered.wasm"
        });
        run_filter_exports.step.dependOn(&install_step.step);

        const run_wasm_opt = b.addSystemCommand(&[_][]const u8{
            "wasm-opt", "-Oz", "-o", "zig-out/cart.wasm", "zig-out/lib/cart-filtered.wasm"
        });
        run_wasm_opt.step.dependOn(&run_filter_exports.step);

        const run_uw8_pack = b.addSystemCommand(&[_][]const u8{
            "uw8", "pack", "-l", "9", "zig-out/cart.wasm", "zig-out/cart.uw8"
        });
        run_uw8_pack.step.dependOn(&run_wasm_opt.step);

        const make_opt = b.step("make_opt", "make size optimized cart");
        make_opt.dependOn(&run_uw8_pack.step);

        b.default_step = make_opt;
    }
}
