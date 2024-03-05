const ArrayList = @import("std").ArrayList;
const Stmt = @import("./stmt.zig").Stmt;

stmts: ArrayList(Stmt),

pub fn deinit(self: *@This()) void {
    for (self.stmts.items) |*s| {
        s.deinit();
    }
    self.stmts.deinit();
}
