const std = @import("std");
const print = std.debug.print;
const testing = std.testing;
const Lexer = @import("./Lexer.zig");

pub fn main() !void {
    var args = std.process.args();
    _ = args.next();
    const program = args.next().?;

    var lex = Lexer.new(program);
    while (lex.next()) |token| {
        print("{any}\n", .{token});
    }
}

test {
    _ = Lexer;
}
