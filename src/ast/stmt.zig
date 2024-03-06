const ArrayList = @import("std").ArrayList;

const Expr = @import("./expr.zig").Expr;
const Token = @import("../Token.zig");
const ParamList = @import("./ParamList.zig");

pub const Stmt = struct {
    kind: StmtKind,
    token: Token,

    pub fn deinit(self: *Stmt) void {
        switch (self.kind) {
            .fn_def => |*def| {
                def.params.deinit();
                def.body.deinit();
            },
            .let => {},
            .@"return" => {},
        }
    }
};

pub const StmtKind = union(enum) {
    let: LetStmt,
    fn_def: FunctionDefinitionStmt,
    @"return": Expr,
};

pub const LetStmt = struct {
    var_name: []const u8,
    expr: Expr,
};

pub const FunctionDefinitionStmt = struct {
    name: []const u8,
    params: ParamList,
    return_type: []const u8,
    body: ArrayList(Stmt),
};
