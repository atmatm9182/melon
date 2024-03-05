const std = @import("std");
const testing = std.testing;

const Token = @import("./Token.zig");
const Location = @import("./Location.zig");

contents: []const u8,
pos: usize,
location: Location,

const Self = @This();

pub fn new(buf: []const u8) Self {
    return .{ .contents = buf, .pos = 0, .location = .{
        .file = "",
        .row = 1,
        .col = 1,
    } };
}

pub fn next(self: *Self) ?Token {
    self.skipWhitespace();

    const char = self.curChar();
    if (char == null)
        return null;

    const loc = self.location;

    const token: Token = switch (char.?) {
        ';' => .{ .ty = .semicolon, .lit = self.sliceLen(1), .loc = loc },
        ':' => .{ .ty = .colon, .lit = self.sliceLen(1), .loc = loc },
        ',' => .{ .ty = .comma, .lit = self.sliceLen(1), .loc = loc },
        '+' => .{ .ty = .plus, .lit = self.sliceLen(1), .loc = loc },
        '-' => .{ .ty = .minus, .lit = self.sliceLen(1), .loc = loc },
        '*' => .{ .ty = .star, .lit = self.sliceLen(1), .loc = loc },
        '/' => .{ .ty = .slash, .lit = self.sliceLen(1), .loc = loc },
        '(' => .{ .ty = .open_paren, .lit = self.sliceLen(1), .loc = loc },
        ')' => .{ .ty = .close_paren, .lit = self.sliceLen(1), .loc = loc },
        '{' => .{ .ty = .open_brace, .lit = self.sliceLen(1), .loc = loc },
        '}' => .{ .ty = .close_brace, .lit = self.sliceLen(1), .loc = loc },
        else => blk: {
            if (std.ascii.isAlphabetic(char.?)) {
                const lit = self.readIdent();
                break :blk .{ .ty = .ident, .lit = lit, .loc = loc };
            }

            break :blk .{ .ty = .illegal, .lit = self.sliceLen(1), .loc = loc };
        },
    };

    self.advance();
    return token;
}

fn advance(self: *Self) void {
    self.pos += 1;
}

fn retreat(self: *Self) void {
    self.pos -= 1;
}

fn skipWhitespace(self: *Self) void {
    while (self.curChar()) |char| {
        if (!std.ascii.isWhitespace(char))
            break;
        self.advance();
    }
}

fn isAtEnd(self: *const Self) bool {
    return self.pos >= self.contents.len;
}

fn curChar(self: *const Self) ?u8 {
    if (self.isAtEnd())
        return null;

    return self.contents[self.pos];
}

fn slice(self: *const Self, from: usize, to: usize) []const u8 {
    return self.contents[from..to];
}

fn sliceLen(self: *const Self, len: usize) []const u8 {
    return self.slice(self.pos, self.pos + len);
}

fn readWhile(self: *Self, pred: *const fn (u8) bool) []const u8 {
    const start = self.pos;

    while (self.curChar()) |char| {
        if (!pred(char))
            break;
        self.advance();
    }

    const res = self.slice(start, self.pos);
    self.retreat();
    return res;
}

fn readIdent(self: *Self) []const u8 {
    return self.readWhile(isValidIdentChar);
}

fn isValidIdentChar(c: u8) bool {
    return c == '_' or std.ascii.isAlphanumeric(c);
}

test "basic tokens" {
    const code = "+-  * /:; ({})";
    var lex = Self.new(code);

    try testing.expectEqual(.plus, lex.next().?.ty);
    try testing.expectEqual(.minus, lex.next().?.ty);
    try testing.expectEqual(.star, lex.next().?.ty);
    try testing.expectEqual(.slash, lex.next().?.ty);
    try testing.expectEqual(.colon, lex.next().?.ty);
    try testing.expectEqual(.semicolon, lex.next().?.ty);
    try testing.expectEqual(.open_paren, lex.next().?.ty);
    try testing.expectEqual(.open_brace, lex.next().?.ty);
    try testing.expectEqual(.close_brace, lex.next().?.ty);
    try testing.expectEqual(.close_paren, lex.next().?.ty);
    try testing.expectEqual(null, lex.next());
}
