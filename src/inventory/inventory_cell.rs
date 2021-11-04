use crate::inventory::Container;
use crate::inventory::InventoryType;

pub struct InventoryCell<T> {
    pub id          : String,
    pub content_id  : String,
    pub title       : Option<String>,
    max_count       : T,
    count           : T
}

impl<T> Container<T> for InventoryCell<T>
    where T: Copy {
    fn get_count(&self) -> T {
        return self.count;
    }

    fn get_max_count(&self) -> T {
        return self.count;
    }

    fn set_count(&mut self, value: T) {
        self.count = value;
    }

    fn set_max_count(&mut self, value: T) {
        self.max_count = value;
    }
}

impl<T> InventoryCell<T>
    where
        T: Copy
        + std::cmp::PartialEq
        + std::ops::AddAssign
        + std::cmp::PartialOrd
        + std::ops::Sub<Output = T> {

    pub fn new(id: String, cell_type: &InventoryType<T>, title: Option<String>, count: T) -> InventoryCell<T> {
        InventoryCell {
            id,
            content_id: cell_type.content_id.to_string(),
            title,
            max_count: cell_type.max_count,
            count
        }
    }

    pub fn is_full(&self) -> bool {
        self.count == self.max_count
    }

    pub fn inc(&mut self, value: T) -> Option<T> {
        self.count += value;

        if self.count > self.max_count {
            let result = self.count - self.max_count;
            self.count = self.max_count;
            return Some(result);
        }

        return None;
    }
}

#[cfg(test)]
mod tests {
    use crate::inventory::InventoryCell;
    use crate::inventory::InventoryType;
    use crate::inventory::Container;


    #[test]
    fn inventory_cell_create_test() {
        let wood_type = InventoryType {content_id: "wood".into(), max_count: 255};

        let cell = InventoryCell::new("0".into(), &wood_type, None, 0);

        assert_eq!(cell.id, "0");
        assert_eq!(cell.content_id, "wood");
        assert_eq!(cell.max_count, 255);
        assert_eq!(cell.count, 0);
        assert_eq!(cell.is_full(), false);
    }

    #[test]
    fn inventory_cell_incriment_test() {
        let wood_type = InventoryType {content_id: "wood".into(), max_count: 255};

        let mut cell = InventoryCell::new("0".into(), &wood_type, None, 0);
        cell.set_max_count(10);

        cell.inc(5);
        assert_eq!(cell.count, 5);
        assert_eq!(cell.is_full(), false);

        cell.inc(5);
        assert_eq!(cell.count, 10);
        assert_eq!(cell.is_full(), true);

        match cell.inc(5) {
            Some(left_over) => assert_eq!(left_over, 5),
            None => assert_eq!(false, true)
        }

        assert_eq!(cell.count, 10);
        assert_eq!(cell.is_full(), true);
    }
}