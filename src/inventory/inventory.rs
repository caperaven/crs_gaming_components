use crate::inventory::InventoryCell;

pub struct Inventory<T> {
    pub id          : String,
    pub title       : Option<String>,
    pub slot_count  : i32,
    slots           : Vec<InventoryCell<T>>
}

impl<T> Inventory<T> {
    pub fn new(id: String, title: Option<String>, slot_count: i32) -> Inventory<T> {
        Inventory {
            id,
            title,
            slot_count,
            slots: Vec::new()
        }
    }

    pub fn add(&mut self, item: InventoryCell<T>) -> bool {
        if self.is_full() == false {
            self.slots.push(item);
            return true;
        }
        
        return false;
    }

    pub fn is_full(&self) -> bool {
        (self.slots.len() as i32) == self.slot_count
    }

    pub fn open_slots(&self) -> i32 {
        self.slot_count - self.slots.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::inventory::InventoryCell;
    use crate::inventory::Inventory;
    use crate::inventory::InventoryType;

    #[test]
    fn inventory_create_test() {
        let inventory: Inventory<i8> = Inventory::new("bag 1".into(), None, 10);
        assert_eq!(inventory.id, "bag 1");
        assert_eq!(inventory.is_full(), false);
    }

    #[test]
    fn inventory_add_test() {
        let wood_type = InventoryType {content_id: "wood".into(), max_count: 255};
        let mut inventory: Inventory<i32> = Inventory::new("bag 1".into(), None, 10);

        for i in 1..10 {
            inventory.add(InventoryCell::new(format!("{}", i), &wood_type, None, 10));
            assert_eq!(inventory.open_slots(), 10 - i);
        }
    }
}
