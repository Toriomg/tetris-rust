use crate::tetromino::TypeTetromino;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

/// Available generation algorithms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GeneratorMode {
    _Classic, // GB style
    _Modern,  // Shuffles a set of 7 pieces and deals them (7 bag style)
}

pub struct BagSystem {
    queue: VecDeque<TypeTetromino>,
    mode: GeneratorMode,
    preview_count: usize,
    internal_bag: Vec<TypeTetromino>,
}

impl BagSystem {
    pub fn new(mode: GeneratorMode, preview_count: usize) -> Self {
        let mut system = Self {
            queue: VecDeque::new(),
            mode,
            preview_count,
            internal_bag: Vec::new(),
        };

        // fill the queue with the required preview count + the active piece
        for _ in 0..=preview_count {
            let next_type = system.generate_next_type();
            system.queue.push_back(next_type);
        }

        system
    }

    /// Returns the next piece type and replenishes the queue
    pub fn pop_and_refill(&mut self) -> TypeTetromino {
        let next_piece = self.queue.pop_front().expect("Bag queue is empty");
        let new_type = self.generate_next_type();
        self.queue.push_back(new_type);
        next_piece
    }

    /// Provides read-only access to the preview queue
    pub fn get_preview(&self) -> &VecDeque<TypeTetromino> {
        &self.queue
    }

    /// Internal logic to generate a single type based on the selected mode
    fn generate_next_type(&mut self) -> TypeTetromino {
        match self.mode {
            GeneratorMode::_Classic => {
                let mut t_type = TypeTetromino::random();
                if Some(&t_type) == self.queue.back() {
                    t_type = TypeTetromino::random();
                }
                t_type
            }
            GeneratorMode::_Modern => {
                // If the internal bag is empty, refill and shuffle it
                if self.internal_bag.is_empty() {
                    self.internal_bag = vec![
                        TypeTetromino::I,
                        TypeTetromino::O,
                        TypeTetromino::T,
                        TypeTetromino::S,
                        TypeTetromino::Z,
                        TypeTetromino::J,
                        TypeTetromino::L,
                    ];
                    let mut rng = rand::thread_rng();
                    self.internal_bag.shuffle(&mut rng);
                }
                // Pop from the shuffled bag
                self.internal_bag.pop().unwrap()
            }
        }
    }
}
