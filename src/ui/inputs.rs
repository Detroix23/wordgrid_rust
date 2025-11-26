// src/ui/inputs.rs  

use crate::{
	dictionaries,
	grid,
	modules,
	ui,
};
/// Execute the app from the given arguments.
/// Joining the help message will stop immediately and display help.
pub fn launch_from_arguments(arguments: Vec<String>) -> () {
	let mut dictionary_name: Option<String> = Option::None;
	let mut grid_name: Option<String> = Option::None;

	let mut warnings: Vec<String> = Vec::new();
	let mut recording: bool = false;
	let mut index: usize = 1;

	while index < arguments.len() {
		let argument: &str = &arguments[index];
		match argument {
			"-d" | "--dict" => {
				dictionary_name = Option::Some(arguments[index + 1].clone());
				index += 1;
			},
			"-g" | "--grid" => {
				grid_name = Option::Some(arguments[index + 1].clone());
				index += 1;
			},
			"-h" | "--help" => {
				println!("{}", ui::help::HELP_MESSAGE);
				return;
			},
			_ => {warnings.push(format!("Floating or invalid value ({}).", argument))}

		}

		index += 1;
	}

	let dictionary: Vec<String> = dictionaries::prepare_dictionary(match dictionary_name {
		Some(name) => name,
		None => {
			warnings.push(format!("No dictionary name given, falling to default ({}).", modules::defaults::DICTIONARY));
			modules::defaults::DICTIONARY.to_string()
		},
	});

    let grid: modules::maths::CharGrid = grid::files::read_from_data(match grid_name {
		Some(name) => name,
		None => {
			warnings.push(format!("No grid name given, falling to default ({}).", modules::defaults::GRID));
			modules::defaults::GRID.to_string()
		},
	});

	eprintln!("\n## Inputs warnings.");
	for warning in warnings {
		eprintln!("  (!) {}", warning);
	}
	eprintln!();

    let mut solver: grid::Grid = grid::Grid::new(
		"Solved grid.".to_string(),
        grid,
        modules::defaults::DIRECTION_ALL.to_vec(),
		dictionary.clone(),
    );

	solver.read(2);

    println!("{}", solver.report_all());

	solver.write_report();

}