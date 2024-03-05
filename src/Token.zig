ty: Type,
lit: []const u8,
loc: @import("./Location.zig"),

pub const Type = enum {
    colon,
    semicolon,
    comma,

    open_paren,
    close_paren,
    open_brace,
    close_brace,

    plus,
    minus,
    star,
    slash,

    ident,
    illegal,
};
