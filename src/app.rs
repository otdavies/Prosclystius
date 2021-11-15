use self::solver::constants::GRID_SIZE;
use self::solver::constants::VARIATIONS;
use self::solver::Solver;
use eframe::{egui, epi};
mod solver;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct ProscylstiusUI {
	solver: solver::Solver,
	example: [[u8; GRID_SIZE]; GRID_SIZE],
	collapse_type: u32,
}

impl Default for ProscylstiusUI {
	fn default() -> Self {
		Self {
			solver: Solver::new(),
			example: [[0; GRID_SIZE]; GRID_SIZE],
			collapse_type: 0,
		}
	}
}

impl epi::App for ProscylstiusUI {
	fn name(&self) -> &str {
		"Proscylstius"
	}

	fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
		let Self { solver, example, collapse_type } = self;

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
			ui.label(format!("Collapse type: {}", collapse_type));
			ui.horizontal(|ui| {
				for ident in 0..VARIATIONS {
					if ui.button(format!("{}", ident)).clicked() {
						*collapse_type = ident as u32;
					}
				}
			});
			ui.label("Trainer");
			let width = GRID_SIZE;
			for x in 0..width {
				ui.horizontal(|ui| {
					for y in 0..width {
						if ui.button(example[x][y]).clicked() {
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
							if ui.button(solver.get_cell(x, y).print()).clicked() {
								solver.propagate(x, y, *collapse_type);
							}
						}
					});
				}
			}
		});
	}
}
