print("hey")

reads = {"Color"}

function run(colors)
    for count = 1, #colors do
        print(colors[count]:get_r())
    end
end

-- Escrito pelo usuário
-- #[derive(Component)]
-- pub struct Color {
--     r: u8,
--     g: u8,
--     b: u8
-- }


-- Gerado pelo Rust com base no component definido
-- typedef struct Color {
--     byte r,
--     byte g,
--     byte b
-- }Color;

-- byte get_r(*Color c) {
--     return c->r;
-- }
