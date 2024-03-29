/*
 * The reason why I decided to implement this in the form of a struct is because it would be more of a challenge.  
 * Even though there are definitely better ways to accomplish this, I figured that this would be significantly more fun.
 * This can be modified to calculate the last digit numerous times in one program execution.
 */

pub struct Fibonacci {
    pub sequence: Vec<u128>,
    pub length: usize,
    pub last_value: Option<u128>
}

impl Fibonacci {
    pub fn new(length: usize) -> Self {
        let mut sequence = Vec::with_capacity(length + 1);
        sequence.push(0); // first index is always 0

        // constant sequence
        if length == 1 {
            return Fibonacci {
                sequence: vec![1],
                length,
                last_value: Some(0)
            };
        }

        // now that we have verified that the length is valid, push 1 as it is constant
        sequence.push(1);

        return Fibonacci {
            sequence,
            length: length + 1, // must return length + 1 to get correct calculation. this is due to rust indexing starting with 0, not 1.
            last_value: None
        }
    }

    // generates the fibonacci sequence up to user specified length
    pub fn update(&mut self) {
        while self.sequence.len() < self.length {
            self.sequence.push(self.calculate_next())
        }

        // update last value
        self.last_value = self.sequence.last().cloned();
    }

    // generate with new length
    pub fn update_with_length(&mut self, length: usize) {

        match length {
            // invalid input
            0 => panic!("Invalid input: Parsed length is less than 1"),
            // constant sequence
            1 => {
                self.sequence = vec![1];
                return;
            },
            _ => {
                // set back up
                self.sequence = Vec::with_capacity(length + 1);
                self.length = length + 1;
                self.sequence.push(0);
                self.sequence.push(1);
            }
        }
        
        while self.sequence.len() < self.length {
            self.sequence.push(self.calculate_next())
        }

        // update last value
        self.last_value = self.sequence.last().cloned();
    }

    // add x of n-1 and x of n-2
    fn calculate_next(&self) -> u128 {
        self.sequence[self.sequence.len() - 1] + self.sequence[self.sequence.len() - 2]
    }
}