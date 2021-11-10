use eframe::{egui, epi};

use self::solver::Solver;
mod solver;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct ProscylstiusUI {
	solver: solver::Solver,
	collapse_type: u32,
}

impl Default for ProscylstiusUI {
	fn default() -> Self {
		Self {
			solver: Solver::new(),
			collapse_type: 0,
		}
	}
}

impl epi::App for ProscylstiusUI {
	fn name(&self) -> &str {
		"Proscylstius"
	}

	fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
		let Self { solver, collapse_type } = self;

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
				for ident in 0..4 {
					if ui.button(format!("{}", ident)).clicked() {
						*collapse_type = ident;
					}
				}
			});

			ui.label("World");
			let width = solver.world.len();
			for x in 0..width {
				ui.horizontal(|ui| {
					for y in 0..width {
						if ui.button(solver.world[x][y].print()).clicked() {
							solver.propagate(x, y, *collapse_type);
						}
					}
				});
			}
		});
	}
}
