use bevy_inspector_egui::Inspectable;

// Enum for all the different sprite entities you need
// Maybe convert to a generic object type? idk
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SpriteType {
	Lab,
	Thermometer,
    LogbookButton,
	Molecule(MoleculeType)
}

// Enum for all the different molecules
#[derive(Inspectable, Debug, PartialEq, Clone, Copy)]
pub enum MoleculeType {
	Red,
	Blue,
	Orange,
    Purple,
    Gray,
}

// Check pls
pub struct Reaction {
	pub products: Vec<MoleculeType>,
    pub reaction_type: ReactionType,
    pub power_generated: f32,
    pub temp_generated: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ReactionType {
	RedBlue,
	GrayPurple,
    BlueBlue,
    OrangeOrange,
    GrayOrange,
    GrayRed,
}

// Implement whatever decision mechanisms for molecule types
impl MoleculeType {
	pub fn can_react(a: MoleculeType, b: MoleculeType) -> Option<Reaction> {
        if (a == MoleculeType::Red && b == MoleculeType::Blue) || (a == MoleculeType::Blue && b == MoleculeType::Red) {
            Some({ Reaction {
                products: vec![MoleculeType::Purple],
                reaction_type: ReactionType::RedBlue,
                power_generated: 10.0,
                temp_generated: 15.0}})
        } else if (a == MoleculeType::Red && b == MoleculeType::Gray) || (a == MoleculeType::Gray && b == MoleculeType::Red) {
            Some({ Reaction {
                products: vec![MoleculeType::Gray],
                reaction_type: ReactionType::GrayRed,
                power_generated: 2.0,
                temp_generated: 15.0}})
        } else if a == MoleculeType::Blue && b == MoleculeType::Blue {
            Some({ Reaction {
                products: vec![MoleculeType::Gray],
                reaction_type: ReactionType::BlueBlue,
                power_generated: 7.0,
                temp_generated: -15.0}})
        } else if a == MoleculeType::Orange && b == MoleculeType::Orange {
            Some({ Reaction {
                products: vec![MoleculeType::Gray, MoleculeType::Gray],
                reaction_type: ReactionType::OrangeOrange,
                power_generated: 50.0,
                temp_generated: 30.0}})
        } else if (a == MoleculeType::Gray && b == MoleculeType::Orange) || (a == MoleculeType::Orange && b == MoleculeType::Gray) {
            Some({ Reaction {
                products: vec![MoleculeType::Red, MoleculeType::Red],
                reaction_type: ReactionType::GrayOrange,
                power_generated: 15.0,
                temp_generated: -5.0}})
        } else if (a == MoleculeType::Purple && b == MoleculeType::Gray) || (a == MoleculeType::Gray && b == MoleculeType::Purple) {
            Some({ Reaction {
                products: vec![MoleculeType::Orange],
                reaction_type: ReactionType::GrayPurple,
                power_generated: 30.0,
                temp_generated: -5.0}})
        } else {
		    None
        }
	}

	pub fn animation_frames(&self) -> usize {
		match self {
			MoleculeType::Red => 8,
			MoleculeType::Blue => 8,
			MoleculeType::Orange => 8,
            MoleculeType::Purple => 8,
            MoleculeType::Gray => 8,
		}
	}

	pub fn mass(&self) -> f32 {
		match self {
			MoleculeType::Red => 0.8,
			MoleculeType::Blue => 1.6,
			MoleculeType::Orange => 2.4,
            MoleculeType::Purple => 3.2,
            MoleculeType::Gray => 0.4,
		}
	}

	pub fn base_cost(&self) -> f32 {
		match self {
			MoleculeType::Red => 2.0,
			MoleculeType::Blue => 4.0,
			MoleculeType::Orange => 20.0,
            MoleculeType::Purple => 8.0,
            MoleculeType::Gray => 1.0,
		}
	}

	pub fn name(&self) -> &'static str {
		match self {
			MoleculeType::Red => "Red Molecule",
			MoleculeType::Blue => "Blue Molecule",
			MoleculeType::Orange => "Orange Molecule",
            MoleculeType::Purple => "Purple Molecule",
            MoleculeType::Gray => "Gray Molecule"
		}
	}
}
