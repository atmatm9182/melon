const std = @import("std");
const ArrayList = std.ArrayList;

const expr = @import("./ast/expr.zig");
const stmt = @import("./ast/stmt.zig");
const Program = @import("./ast/Program.zig");
const ParamList = @import("./ast/ParamList.zig");

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

const ParseStmtError = error{OutOfMemory} || Error || std.fmt.ParseIntError;

fn parseStmt(self: *Self) ParseStmtError!stmt.Stmt {
    if (self.cur == null)
        return Error.Eof;

    const tok = self.cur.?;
    return switch (tok.ty) {
        .let => self.parseLet(),
        .def => self.parseFunctionDef(),
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

fn expectCur(self: *Self, tt: Token.Type) Error!Token {
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

fn parseIntLit(self: *Self) std.fmt.ParseIntError!i128 {
    const cur = self.cur.?;
    const i = try std.fmt.parseInt(i128, cur.lit, 10);
    self.advance();
    return i;
}

fn parseIdent(self: *Self) ![]const u8 {
    const tok = try self.expectCur(.ident);
    return tok.lit;
}

fn wrongCurrentToken(self: *Self, expected: []const Token.Type, got: Token) Error {
    self.wrong_cur = got;
    self.expected_cur = expected;
    return Error.WrongCurToken;
}

fn parseFunctionDef(self: *Self) !stmt.Stmt {
    const tok = self.cur.?;
    self.advance();

    const name = try self.parseIdent();
    const params = try self.parseParamList();
    _ = try self.expectCur(.colon);
    const ret_type = try self.parseIdent();
    const body = try self.parseBlock();

    const kind = stmt.StmtKind{ .fn_def = .{ .name = name, .params = params, .return_type = ret_type, .body = body } };
    return .{ .kind = kind, .token = tok };
}

fn parseParamList(self: *Self) !ParamList {
    _ = try self.expectCur(.open_paren);

    var list = ArrayList(ParamList.Pair).init(self.alloc);
    errdefer list.deinit();

    while (self.cur) |tok| {
        if (tok.ty == .close_paren)
            break;

        const name = try self.parseIdent();
        _ = try self.expectCur(.colon);
        const ty = try self.parseIdent();

        if (self.cur) |cur| {
            if (cur.ty == .comma)
                self.advance();
        } else {
            return Error.Eof;
        }

        try list.append(.{ .name = name, .ty = ty });
    }
    self.advance();

    return .{ .params = list };
}

fn parseBlock(self: *Self) !ArrayList(stmt.Stmt) {
    _ = try self.expectCur(.open_brace);

    var stmts = ArrayList(stmt.Stmt).init(self.alloc);
    errdefer stmts.deinit();

    while (self.cur) |cur| {
        if (cur.ty == .close_brace)
            break;
        const s = try self.parseStmt();
        try stmts.append(s);
    }

    self.advance();

    return stmts;
}
