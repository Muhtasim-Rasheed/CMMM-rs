use std::collections::HashSet;

use macroquad::prelude::*;

use super::{cells::Cells, directions::Directions, emptycell::EmptyCell, movercell::MoverCell, pushcell::PushCell};

pub struct Grid {
    pub cells: Vec<Vec<Cells>>,
    pub draw_offset: (i32, i32),
    pub is_paused: bool,
    pub highlighted_cell: Option<(u32, u32)>,
}

impl Grid {
    pub async fn new(w: u32, h: u32) -> Grid {
        let mut cells = Vec::new();
        for i in 0..w {
            let mut row = Vec::new();
            for j in 0..h {
                row.push(Cells::EmptyCell(EmptyCell::new(i as f32 * 64.0, j as f32 * 64.0).await));
            }
            cells.push(row);
        }
        Grid {
            cells,
            draw_offset: (0, 0),
            is_paused: true,
            highlighted_cell: None,
        }
    }

    pub fn set_cell(&mut self, x: u32, y: u32, cell: Cells) {
        self.cells[x as usize][y as usize] = cell;
    }

    pub fn set_draw_offset(&mut self, x: i32, y: i32) {
        self.draw_offset = (x, y);
        for (i, row) in self.cells.iter_mut().enumerate() {
            for (j, cell) in row.iter_mut().enumerate() {
                match cell {
                    Cells::EmptyCell(emptycell) => emptycell.set_position(
                        (i as u32 * 64) as f32 + x as f32,
                        (j as u32 * 64) as f32 + y as f32,
                    ),
                    Cells::MoverCell(movercell) => movercell.set_position(
                        (i as u32 * 64) as f32 + x as f32,
                        (j as u32 * 64) as f32 + y as f32,
                    ),
                    Cells::PushCell(pushcell) => pushcell.set_position(
                        (i as u32 * 64) as f32 + x as f32,
                        (j as u32 * 64) as f32 + y as f32,
                    ),
                    _ => {}
                }
            }
        }
    }
    
    // pub async fn update(&mut self, tick: u32) {
    //     if tick % 10 != 0 {
    //         return;
    //     }
    //     if self.is_paused {
    //         return;
    //     }

    //     let grid_rows = self.cells.len();
    //     let grid_cols = self.cells[0].len();
    //     let mut moves = Vec::new();

    //     // First collect all potential moves
    //     for x in 0..grid_rows {
    //         for y in 0..grid_cols {
    //             if let Cells::MoverCell(mover) = &self.cells[x][y] {
    //                 let (dx, dy) = match mover.direction {
    //                     Directions::Up => (0, -1),
    //                     Directions::Down => (0, 1),
    //                     Directions::Left => (-1, 0),
    //                     Directions::Right => (1, 0),
    //                 };

    //                 let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);

    //                 // Check boundaries
    //                 if new_x < 0 || new_x >= grid_rows as i32 || new_y < 0 || new_y >= grid_cols as i32 {
    //                     continue;
    //                 }

    //                 let (ux, uy) = (new_x as usize, new_y as usize);
                    
    //                 // Check what's in the target cell
    //                 match &self.cells[ux][uy] {
    //                     Cells::EmptyCell(_) => {
    //                         moves.push((x, y, ux, uy, mover.direction.clone()));
    //                     }
    //                     Cells::PushCell(_) => {
    //                         // Check if we can push the entire chain
    //                         let mut push_chain = vec![(ux, uy)];
    //                         let mut current_x = new_x;
    //                         let mut current_y = new_y;
    //                         let mut can_push = true;

    //                         while can_push {
    //                             current_x += dx;
    //                             current_y += dy;

    //                             if current_x < 0 || current_x >= grid_rows as i32 ||
    //                                current_y < 0 || current_y >= grid_cols as i32 {
    //                                 can_push = false;
    //                                 break;
    //                             }

    //                             match &self.cells[current_x as usize][current_y as usize] {
    //                                 Cells::EmptyCell(_) => {
    //                                     push_chain.push((current_x as usize, current_y as usize));
    //                                     break;
    //                                 }
    //                                 Cells::MoverCell(_) => {
    //                                     push_chain.push((current_x as usize, current_y as usize));
    //                                     break;
    //                                 }
    //                                 Cells::PushCell(_) => {
    //                                     push_chain.push((current_x as usize, current_y as usize));
    //                                 }
    //                                 _ => {
    //                                     can_push = false;
    //                                     break;
    //                                 }
    //                             }
    //                         }

    //                         if can_push {
    //                             // Add mover cell move
    //                             moves.push((x, y, ux, uy, mover.direction.clone()));
                                
    //                             // Add push cell moves in reverse order
    //                             for i in (0..push_chain.len()).rev() {
    //                                 if i == 0 {
    //                                     continue;
    //                                 }
    //                                 let (from_x, from_y) = push_chain[i-1];
    //                                 let (to_x, to_y) = push_chain[i];
    //                                 moves.push((from_x, from_y, to_x, to_y, mover.direction.clone()));
    //                             }
    //                         }
    //                     }
    //                     _ => {} // Blocked by other cell type
    //                 }
    //             }
    //         }
    //     }

    //     // Apply moves in reverse order to prevent overwriting
    //     moves.reverse();

    //     // Track cells to clear
    //     let mut to_clear = HashSet::new();
        
    //     // Process all moves
    //     for (from_x, from_y, _to_x, _to_y, _) in &moves {
    //         to_clear.insert((*from_x, *from_y));
    //     }

    //     // Save the current state of the grid
    //     let grid_state = self.cells.clone();

    //     // Clear original positions
    //     for (x, y) in &to_clear {
    //         self.cells[*x][*y] = Cells::EmptyCell(EmptyCell::new(
    //             *x as f32 * 64.0 + self.draw_offset.0 as f32,
    //             *y as f32 * 64.0 + self.draw_offset.1 as f32
    //         ).await);
    //     }

    //     // Create new cells at target positions
    //     for (from_x, from_y, to_x, to_y, direction) in moves {
    //         if let Cells::MoverCell(_mover) = &grid_state[from_x][from_y] {
    //             self.cells[to_x][to_y] = Cells::MoverCell(MoverCell::new(
    //                 to_x as f32 * 64.0 + self.draw_offset.0 as f32,
    //                 to_y as f32 * 64.0 + self.draw_offset.1 as f32,
    //                 Some(direction)
    //             ).await);
    //         } else if let Cells::PushCell(_push) = &grid_state[from_x][from_y] {
    //             self.cells[to_x][to_y] = Cells::PushCell(PushCell::new(
    //                 to_x as f32 * 64.0 + self.draw_offset.0 as f32,
    //                 to_y as f32 * 64.0 + self.draw_offset.1 as f32
    //             ).await);
    //         }
    //     }
    // }

    pub async fn update(&mut self, tick: u32) {
        if tick % 10 != 0 {
            return;
        }
        if self.is_paused {
            return;
        }

        let grid_rows = self.cells.len();
        let grid_cols = self.cells[0].len();
        let mut moves = Vec::new();

        // First collect all potential moves
        for x in 0..grid_rows {
            for y in 0..grid_cols {
                if let Cells::MoverCell(mover) = &self.cells[x][y] {
                    let (dx, dy) = match mover.direction {
                        Directions::Up => (0, -1),
                        Directions::Down => (0, 1),
                        Directions::Left => (-1, 0),
                        Directions::Right => (1, 0),
                    };

                    let (new_x, new_y) = (x as i32 + dx, y as i32 + dy);

                    // Check boundaries
                    if new_x < 0 || new_x >= grid_rows as i32 || new_y < 0 || new_y >= grid_cols as i32 {
                        continue;
                    }

                    let (ux, uy) = (new_x as usize, new_y as usize);
                    
                    // Check what's in the target cell
                    match &self.cells[ux][uy] {
                        Cells::EmptyCell(_) => {
                            moves.push((x, y, ux, uy));
                        }
                        Cells::PushCell(_) | Cells::MoverCell(_) => {
                            // Check if we can push the entire chain
                            let mut push_chain = vec![(ux, uy)];
                            let mut current_x = new_x;
                            let mut current_y = new_y;
                            let mut can_push = true;

                            while can_push {
                                current_x += dx;
                                current_y += dy;

                                if current_x < 0 || current_x >= grid_rows as i32 ||
                                   current_y < 0 || current_y >= grid_cols as i32 {
                                    can_push = false;
                                    break;
                                }

                                match &self.cells[current_x as usize][current_y as usize] {
                                    Cells::EmptyCell(_) => {
                                        push_chain.push((current_x as usize, current_y as usize));
                                        break;
                                    }
                                    Cells::PushCell(_) | Cells::MoverCell(_) => {
                                        push_chain.push((current_x as usize, current_y as usize));
                                    }
                                    _ => {
                                        can_push = false;
                                        break;
                                    }
                                }
                            }

                            if can_push {
                                // Add mover cell move
                                moves.push((x, y, ux, uy));
                                
                                // Add push cell moves in reverse order
                                for i in (0..push_chain.len()).rev() {
                                    if i == 0 {
                                        continue;
                                    }
                                    let (from_x, from_y) = push_chain[i-1];
                                    let (to_x, to_y) = push_chain[i];
                                    moves.push((from_x, from_y, to_x, to_y));
                                }
                            }
                        }
                        _ => {} // Blocked by other cell type
                    }
                }
            }
        }

        // Apply moves in reverse order to prevent overwriting
        moves.reverse();

        // Save the current state of the grid
        let grid_state = self.cells.clone();

        // Clear original positions first
        let mut to_clear = HashSet::new();
        for (from_x, from_y, _to_x, _to_y) in &moves {
            to_clear.insert((*from_x, *from_y));
        }

        for (x, y) in &to_clear {
            self.cells[*x][*y] = Cells::EmptyCell(EmptyCell::new(
                *x as f32 * 64.0 + self.draw_offset.0 as f32,
                *y as f32 * 64.0 + self.draw_offset.1 as f32
            ).await);
        }

        // Create new cells at target positions with original properties
        for (from_x, from_y, to_x, to_y) in moves {
            match &grid_state[from_x][from_y] {
                Cells::MoverCell(mover) => {
                    self.cells[to_x][to_y] = Cells::MoverCell(MoverCell::new(
                        to_x as f32 * 64.0 + self.draw_offset.0 as f32,
                        to_y as f32 * 64.0 + self.draw_offset.1 as f32,
                        Some(mover.direction.clone())
                    ).await);
                }
                Cells::PushCell(_) => {
                    self.cells[to_x][to_y] = Cells::PushCell(PushCell::new(
                        to_x as f32 * 64.0 + self.draw_offset.0 as f32,
                        to_y as f32 * 64.0 + self.draw_offset.1 as f32
                    ).await);
                }
                _ => {}
            }
        }
    }

    pub fn draw(&self) {
        for row in &self.cells {
            for cell in row {
                // If the cell is out of the screen, do not draw it
                if let Cells::EmptyCell(emptycell) = cell {
                    if emptycell.x < -64.0
                        || emptycell.x > screen_width() + 64.0
                        || emptycell.y < -64.0
                        || emptycell.y > screen_height() + 64.0
                    {
                        continue;
                    }
                }
                if let Cells::MoverCell(movercell) = cell {
                    if movercell.x < -64.0
                        || movercell.x > screen_width() + 64.0
                        || movercell.y < -64.0
                        || movercell.y > screen_height() + 64.0
                    {
                        continue;
                    }
                }
                if let Cells::PushCell(pushcell) = cell {
                    if pushcell.x < -64.0
                        || pushcell.x > screen_width() + 64.0
                        || pushcell.y < -64.0
                        || pushcell.y > screen_height() + 64.0
                    {
                        continue;
                    }
                }

                match cell {
                    Cells::EmptyCell(emptycell) => emptycell.draw(),
                    Cells::MoverCell(movercell) => movercell.draw(),
                    Cells::PushCell(pushcell) => pushcell.draw(),
                    _ => {}
                }
            }
        }

        if let Some((x, y)) = self.highlighted_cell {
            draw_rectangle(
                x as f32 * 64.0 + self.draw_offset.0 as f32,
                y as f32 * 64.0 + self.draw_offset.1 as f32,
                64.0,
                64.0,
                Color::new(1.0, 1.0, 1.0, 0.25),
            );
        }
    }
}
