#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[allow(dead_code)]
enum SpriteId {
    TestingSprite,
    TestingSprite2,
    Other,
}

struct Bank {
    inner: std::collections::HashMap<SpriteId, ggez::graphics::InstanceArray>,
}

impl Bank {
    fn new(ctx: &mut ggez::Context) -> Self {
        let mut bank = Self {
            inner: std::collections::HashMap::new(),
        };

        bank.inner.insert(
            SpriteId::TestingSprite,
            ggez::graphics::InstanceArray::new_ordered(
                ctx,
                ggez::graphics::Image::from_path(ctx, "/image.png").unwrap(),
            ),
        );
        bank.inner.insert(
            SpriteId::TestingSprite2,
            ggez::graphics::InstanceArray::new_ordered(
                ctx,
                ggez::graphics::Image::from_path(ctx, "/image2.png").unwrap(),
            ),
        );
        bank
    }
}

struct Testing {
    bank: Bank,
}

impl Testing {
    fn new(ctx: &mut ggez::Context) -> Self {
        let bank = Bank::new(ctx);

        Self { bank }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for Testing {
    fn update(&mut self, _: &mut ggez::Context) -> ggez::GameResult {
        // yea

        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::BLACK).finish(ctx)?;

        let position1 = (100., 100.);

        let position2 = (150., 100.);

        super_simple_render(ctx, position1, SpriteId::TestingSprite, &mut self.bank);

        super_simple_render(ctx, position2, SpriteId::TestingSprite2, &mut self.bank);

        Ok(())
    }
}

fn super_simple_render(
    ctx: &mut ggez::Context,
    position: (f32, f32),
    sprite: SpriteId,
    bank: &mut Bank,
) {
    let mut canvas = ggez::graphics::Canvas::from_frame(ctx, None);

    let ia = bank.inner.get_mut(&sprite).unwrap();

    ia.push(
        ggez::graphics::DrawParam::default()
            .dest(ggez::mint::Point2 {
                x: position.0,
                y: position.1,
            })
            .scale(ggez::mint::Point2 { x: 1., y: 1. }),
    );

    canvas.draw(ia, ggez::graphics::DrawParam::default());
    ia.clear();

    canvas.finish(ctx).unwrap();
}

fn main() {
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("This is a test", "Bwrc").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build().unwrap();

    let game = Testing::new(&mut ctx);

    ggez::event::run(ctx, event_loop, game);
}
