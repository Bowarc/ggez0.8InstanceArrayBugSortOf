This is a bug that appeared with ggez0.8

when clearing and re-using a `ggez::graphics::InstanceArray` in a single frame, only the last call would actually be rendered to the screen

In ggez0.7 you could use the same system with `ggez::graphics::sprite_batch::SpriteBatch` and it would draw every calls no matter how many time it would have been used this frame