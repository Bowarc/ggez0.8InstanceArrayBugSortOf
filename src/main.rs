#[derive(Clone, Copy, Eq, Hash, PartialEq)]
#[allow(dead_code)]
/// This is used to reference IAs (It's given to any entity that can be drawn to the screen using an IA)
enum SpriteId {
    TestingSprite,
    TestingSprite2,
    Other,
}

/// This holds all the IAs
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

/// The 'Game' structure
struct Testing {
    bank: Bank,
    show_bug: bool,
}

impl Testing {
    fn new(ctx: &mut ggez::Context) -> Self {
        let bank = Bank::new(ctx);

        Self {
            bank,
            show_bug: true,
        }
    }
}

impl ggez::event::EventHandler<ggez::GameError> for Testing {
    fn update(&mut self, _: &mut ggez::Context) -> ggez::GameResult {
        // yea

        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        // about this line, i diddn't found any better way to clear the screen with this drawing method
        ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::BLACK).finish(ctx)?;

        let position1 = (100., 100.);

        let position2 = (150., 100.);

        let first_sprite = SpriteId::TestingSprite;

        // If you clear and re-use an 'ggez::graphics::InstanceArray' multiple times in a single frame, only the last call is used

        // in other words if this is 'true' only position1 will be shown

        // In ggez0.7 (SpriteBatches) would not behave this way,
        // it would draw every calls no matter how many times it was 'cleared & re-used' per frame
        let seccond_sprite = match self.show_bug {
            true => first_sprite,
            false => SpriteId::TestingSprite2,
        };

        super_simple_render(ctx, position1, first_sprite, &mut self.bank);

        super_simple_render(ctx, position2, seccond_sprite, &mut self.bank);

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut ggez::Context,
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    ) -> Result<(), ggez::GameError> {
        if input.keycode == Some(ggez::input::keyboard::KeyCode::Space) && !repeated {
            self.show_bug = !self.show_bug;

            match self.show_bug {
                true => println!("This is the actual ggez0.8 result"),
                false => println!("This is what ggez0.7 would show"),
            }
        }
        Ok(())
    }
}

/// a way to render an a list of object to the screen
// (This is a very simplified version, in my game it has a layer system wherre a layer corresponds to a cavas and
// you give a list of draw command to execute per layer)
fn super_simple_render(
    ctx: &mut ggez::Context,
    position: (f32, f32),
    sprite: SpriteId,
    bank: &mut Bank,
) {
    let mut canvas = ggez::graphics::Canvas::from_frame(ctx, None);

    let ia = bank.inner.get_mut(&sprite).unwrap();

    // obviously don't add only one ggez::graphics::DrawParam before drawing, that's just for the example
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
    println!("Press [Space] to switch between the expected case and the actual result");
    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        std::path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("This is a test", "Bowarc").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build().unwrap();

    let game = Testing::new(&mut ctx);

    ggez::event::run(ctx, event_loop, game);
}
