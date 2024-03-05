const Token = @import("../Token.zig");

pub const Expr = struct {
    kind: ExprKind,
    token: Token,
};

pub const ExprKind = union(enum) {
    int_lit: i128,
    ident: []const u8,
};
