const std = @import("std");

const expr = @import("./ast/expr.zig");
const stmt = @import("./ast/stmt.zig");
const Program = @import("./ast/Program.zig");

const Token = @import("./Token.zig");
const Lexer = @import("./Lexer.zig");

const Allocator = std.mem.Allocator;

cur: ?Token,
peek: ?Token,
lex: Lexer,

alloc: Allocator,

wrong_cur: ?Token = null,
expected_cur: ?[]const Token.Type = null,
wrong_peek: ?Token = null,
expected_peek: ?[]const Token.Type = null,

const Self = @This();

const Error = error{
    WrongCurToken,
    WrongPeekToken,
    Eof,
};

pub fn new(lex: Lexer, alloc: Allocator) Self {
    var lex_mut = lex;
    const cur = lex_mut.next();
    const peek = lex_mut.next();

    return .{
        .cur = cur,
        .peek = peek,
        .lex = lex_mut,
        .alloc = alloc,
    };
}

pub fn parseProgram(self: *Self) !Program {
    var stmts = std.ArrayList(stmt.Stmt).init(self.alloc);
    errdefer stmts.deinit();

    while (self.cur != null) {
        const s = try self.parseStmt();
        try stmts.append(s);
    }
    return Program{ .stmts = stmts };
}

fn parseStmt(self: *Self) !stmt.Stmt {
    if (self.cur == null)
        return Error.Eof;

    const tok = self.cur.?;
    return switch (tok.ty) {
        .let => self.parseLet(),
        // .def => self.parseDef(),
        else => self.wrongCurrentToken(&[_]Token.Type{.let}, tok),
    };
}

fn parseLet(self: *Self) !stmt.Stmt {
    const tok = self.cur.?;
    self.advance();

    const var_name = try self.expectCur(.ident);
    _ = try self.expectCur(.assign);
    const e = try self.parseExpr();
    _ = try self.expectCur(.semicolon);

    const kind = stmt.StmtKind{ .let = .{ .var_name = var_name.lit, .expr = e } };
    return .{ .kind = kind, .token = tok };
}

fn advance(self: *Self) void {
    self.cur = self.peek;
    self.peek = self.lex.next();
}

fn expectCur(self: *Self, tt: Token.Type) !Token {
    if (self.cur) |tok| {
        if (tok.ty == tt) {
            self.advance();
            return tok;
        }

        return self.wrongCurrentToken(&[_]Token.Type{tt}, tok);
    }

    return Error.Eof;
}

fn parseExpr(self: *Self) !expr.Expr {
    if (self.cur == null)
        return Error.Eof;

    const tok = self.cur.?;
    return switch (tok.ty) {
        .int_lit => int: {
            const i = try self.parseIntLit();
            const kind = expr.ExprKind{ .int_lit = i };
            break :int .{ .kind = kind, .token = tok };
        },
        else => self.wrongCurrentToken(&[_]Token.Type{ .int_lit, .ident }, tok),
    };
}

fn parseIntLit(self: *Self) !i128 {
    const cur = self.cur.?;
    const i = try std.fmt.parseInt(i128, cur.lit, 10);
    self.advance();
    return i;
}

fn wrongCurrentToken(self: *Self, expected: []const Token.Type, got: Token) Error {
    self.wrong_cur = got;
    self.expected_cur = expected;
    return Error.WrongCurToken;
}
