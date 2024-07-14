use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct RecycledListRef {
    pub id: u32,
    pub index: usize,
}

impl RecycledListRef {
    pub fn null_ref() -> Self {
        RecycledListRef { id: 0, index: 0 }
    }
}

#[derive(Clone)]
pub struct RecycledListItem<T> {
    pub item_ref: RecycledListRef,
    pub data: T,
}

#[derive(Clone)]
pub struct RecycledList<T> {
    current_id: u32,
    items: Vec<RecycledListItem<T>>,
    free_list: Vec<usize>,
}

impl<T: Clone> RecycledList<T> {
    pub fn new() -> Self {
        RecycledList {
            current_id: 0,
            items: vec![],
            free_list: vec![],
        }
    }

    pub fn get(&self, item_ref: RecycledListRef) -> Option<&T> {
        if item_ref.id == 0 {
            return None;
        }

        let item = self.items.get(item_ref.index);
        match item {
            Some(c) => {
                if c.item_ref.id == item_ref.id {
                    return Some(&c.data);
                }
                None
            }
            _ => None,
        }
    }

    pub fn get_clone(&self, item_ref: RecycledListRef) -> Option<T> {
        if item_ref.id == 0 {
            return None;
        }

        let item = self.items.get(item_ref.index);
        match item {
            Some(c) => {
                if c.item_ref.id == item_ref.id {
                    return Some(c.data.clone());
                }
                None
            }
            _ => None,
        }
    }

    pub fn get_mut(&mut self, item_ref: RecycledListRef) -> Option<&mut T> {
        if item_ref.id == 0 {
            return None;
        }

        let item = self.items.get_mut(item_ref.index);
        match item {
            Some(c) => {
                if c.item_ref.id == item_ref.id {
                    return Some(&mut c.data);
                }
                None
            }
            _ => None,
        }
    }

    pub fn remove(&mut self, item_ref: RecycledListRef) {
        let opt_item = self.items.get_mut(item_ref.index);

        if let Some(item) = opt_item {
            if item.item_ref.id == item_ref.id {
                item.item_ref.id = 0;
                self.free_list.push(item_ref.index);
            }
        }
    }

    pub fn add(&mut self, data: T) -> RecycledListRef {
        self.current_id += 1;
        if self.free_list.is_empty() {
            let item_ref = RecycledListRef {
                id: self.current_id,
                index: self.items.len(),
            };
            self.items.push(RecycledListItem { item_ref, data });
            item_ref
        } else {
            let free_index = self.free_list.pop().unwrap();
            let item_ref = RecycledListRef {
                id: self.current_id,
                index: free_index,
            };
            self.items[free_index] = RecycledListItem { item_ref, data };
            item_ref
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.len() == self.free_list.len()
    }

    pub fn clear(&mut self) {
        if self.is_empty() {
            return;
        }
        let mut items_to_remove: Vec<RecycledListRef> =
            Vec::with_capacity(self.items.len() - self.free_list.len());
        self.enumerate()
            .for_each(|x| items_to_remove.push(x.item_ref));
        items_to_remove.iter().for_each(|x| self.remove(*x));
    }

    pub fn enumerate(&self) -> impl Iterator<Item = &RecycledListItem<T>> {
        self.items.iter().filter(|x| x.item_ref.id != 0)
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = &mut RecycledListItem<T>> {
        self.items.iter_mut().filter(|x| x.item_ref.id != 0)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.enumerate().map(|x| &x.data)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.enumerate_mut().map(|x| &mut x.data)
    }
}

impl<T: Clone> Default for RecycledList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[test]
fn test_test() {
    let mut v: RecycledList<String> = RecycledList::new();
    let ref1 = v.add(String::from("test1"));
    let ref2 = v.add(String::from("test2"));

    v.remove(ref1);
    assert!(Option::is_none(&v.get_mut(ref1)));
    assert_eq!(*v.get_mut(ref2).unwrap(), String::from("test2"));

    let ref3 = v.add(String::from("test3"));
    let ref4 = v.add(String::from("test4"));

    assert_eq!(*v.get_mut(ref2).unwrap(), String::from("test2"));
    assert_eq!(*v.get_mut(ref3).unwrap(), String::from("test3"));
    assert_eq!(*v.get_mut(ref4).unwrap(), String::from("test4"));
}
