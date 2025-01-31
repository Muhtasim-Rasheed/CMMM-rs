use super::{emptycell::EmptyCell, generatorcell::GeneratorCell, movercell::MoverCell, pushcell::PushCell};

pub enum Cells {
    EmptyCell(EmptyCell),
    MoverCell(MoverCell),
    PushCell(PushCell),
    GeneratorCell(GeneratorCell),
}

impl Clone for Cells {
    fn clone(&self) -> Cells {
        match self {
            Cells::EmptyCell(cell) => Cells::EmptyCell(cell.clone()),
            Cells::MoverCell(cell) => Cells::MoverCell(cell.clone()),
            Cells::PushCell(cell) => Cells::PushCell(cell.clone()),
            Cells::GeneratorCell(cell) => Cells::GeneratorCell(cell.clone()),
        }
    }
}
