use self::solver::constants::GRID_SIZE;
use self::solver::constants::VARIATIONS;
use self::solver::Solver;
use eframe::egui::color::Color32;
use eframe::egui::Vec2;
use eframe::{egui, epi};
mod solver;
use std::convert::TryInto;
use std::fs;

// Image loading
extern crate image;
use image::DynamicImage;
use image::GenericImageView;

pub struct ProscylstiusUI {
	solver: solver::Solver,
	example: [[u8; GRID_SIZE]; GRID_SIZE],
	collapse_type: u32,
	loaded_images: Vec<Image>,
}

impl Default for ProscylstiusUI {
	fn default() -> Self {
		Self {
			solver: Solver::new(),
			example: [[0; GRID_SIZE]; GRID_SIZE],
			collapse_type: 0,
			loaded_images: Vec::new(),
		}
	}
}

fn load_images(frame: &mut epi::Frame<'_>, uri: &str) -> Vec<Image> {
	let mut images: Vec<Image> = Vec::new();
	let paths = fs::read_dir(uri).unwrap();
	for path in paths {
		let mut image = Image::new(&image::open(path.unwrap().path()).unwrap());
		assign_texture_id(frame, &mut image);
		images.push(image);
	}

	return images;
}

impl epi::App for ProscylstiusUI {
	fn name(&self) -> &str {
		"Proscylstius"
	}

	fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>, _storage: Option<&dyn epi::Storage>) {
		self.loaded_images = load_images(_frame, "./src/resources/images/");
	}

	fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
		let Self {
			solver,
			example,
			collapse_type,
			loaded_images,
		} = self;

		egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
			// The top panel is often a good place for a menu bar:
			egui::menu::bar(ui, |ui| {
				egui::menu::menu(ui, "File", |ui| {
					if ui.button("Quit").clicked() {
						frame.quit();
					}
				});
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.spacing_mut().item_spacing = egui::vec2(1.0, 1.0);
			ui.label("Collapse types");
			ui.horizontal(|ui| {
				for ident in 0..VARIATIONS {
					let image = &loaded_images[ident];
					let mut image_button = egui::widgets::ImageButton::new(image.texture_id.unwrap(), image.size).frame(false);
					if (*collapse_type == ident as u32) {
						image_button = image_button.tint(Color32::GREEN);
					}
					let tile = ui.add(image_button);

					if tile.clicked() {
						*collapse_type = ident as u32;
					}
				}
			});
			ui.label("Trainer");
			let width = GRID_SIZE;
			for x in 0..width {
				ui.horizontal(|ui| {
					for y in 0..width {
						let image = &loaded_images[example[x][y] as usize];
						let tile = ui.add(egui::widgets::ImageButton::new(image.texture_id.unwrap(), image.size * 2.0).frame(false));
						if tile.clicked() {
							example[x][y] = (*collapse_type) as u8;
						}
					}
				});
			}
			if ui.button("Train!").clicked() {
				solver.train(example, VARIATIONS as u32);
			}

			ui.label("World");
			if solver.is_trained() {
				let width = solver.world_width();
				for x in 0..width {
					ui.horizontal(|ui| {
						for y in 0..width {
							if solver.get_cell(x, y).is_stable() {
								let image = &loaded_images[solver.get_cell(x, y).get_stable_value() as usize];
								let tile = ui.add(egui::widgets::ImageButton::new(image.texture_id.unwrap(), image.size * 2.0).frame(false));
							} else {
								if ui.button(solver.get_cell(x, y).print()).clicked() {
									solver.propagate(x, y, *collapse_type);
								}
							}
						}
					});
				}
			}
		});
	}
}

fn assign_texture_id(frame: &mut epi::Frame<'_>, image: &mut Image) -> Option<egui::TextureId> {
	if let Some(texture_id) = image.texture_id.take() {
		frame.tex_allocator().free(texture_id);
	}
	image.texture_id = Some(frame.tex_allocator().alloc_srgba_premultiplied(image.dimensions, &image.pixels));
	image.texture_id
}

struct Image {
	size: Vec2,
	dimensions: (usize, usize),
	pixels: Vec<egui::Color32>,
	texture_id: Option<egui::TextureId>,
}

impl Image {
	fn new(image: &DynamicImage) -> Self {
		let image_buffer = image.to_rgba8();
		let dimensions = (image.width() as usize, image.height() as usize);
		let size = Vec2::new(image.width() as f32, image.height() as f32);
		let pixels = image_buffer.into_vec();
		assert_eq!(dimensions.0 * dimensions.1 * 4, pixels.len());
		let pixels = pixels.chunks(4).map(|p| egui::Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3])).collect();
		Self {
			size: size,
			dimensions: dimensions,
			pixels: pixels,
			texture_id: None,
		}
	}
}
