const std = @import("std");
const ArrayList = std.ArrayList;

params: ArrayList(Pair),

pub fn deinit(self: *@This()) void {
    self.params.deinit();
}

pub const Pair = struct {
    name: []const u8,
    ty: []const u8,
};
