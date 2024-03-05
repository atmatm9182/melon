const std = @import("std");
const print = std.debug.print;
const testing = std.testing;
const Lexer = @import("./Lexer.zig");
const Parser = @import("./Parser.zig");

pub fn main() !void {
    var args = std.process.args();
    _ = args.next();
    const program = args.next().?;

    const lex = Lexer.new(program);

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const alloc = gpa.allocator();

    defer {
        _ = gpa.deinit();
    }

    var parser = Parser.new(lex, alloc);

    var prog = parser.parseProgram() catch |err| switch (err) {
        error.WrongCurToken => {
            print("expected any of {any}, but got {any} instead\n", .{ parser.expected_cur, parser.wrong_cur });
            std.process.exit(1);
        },
        else => unreachable, // TODO: report errors
    };

    for (prog.stmts.items) |stmt| {
        print("{any}\n", .{stmt.kind});
    }

    prog.deinit();
}

test {
    _ = Lexer;
}
