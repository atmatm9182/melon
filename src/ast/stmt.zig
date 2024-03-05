const Expr = @import("./expr.zig").Expr;
const Token = @import("../Token.zig");

pub const Stmt = struct {
    kind: StmtKind,
    token: Token,
};

pub const StmtKind = union(enum) {
    let: LetStmt,
};

pub const LetStmt = struct {
    var_name: []const u8,
    expr: Expr,
};
