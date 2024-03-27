// Health component
pub struct Health {
    value: u32,
}

impl Health {
    pub fn new(initial_health: u32) -> Self {
        Health { value: initial_health }
    }

    pub fn take_damage(&mut self, damage: u32) {
        if self.value > damage {
            self.value -= damage;
        } else {
            self.value = 0;
        }
    }

    pub fn heal(&mut self, amount: u32) {
        self.value += amount;
    }

    pub fn is_alive(&self) -> bool {
        self.value > 0
    }
}