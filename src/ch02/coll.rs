use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

#[test]
fn test_vec() {
    let mut v1 = vec![];
    v1.push(1);
    v1.push(2);
    v1.push(3);

    assert_eq!(v1, [1, 2, 3]);
    assert_eq!(v1[0], 1);

    let mut v2 = vec![0; 10];

    let mut v2 = Vec::new();
    v2.push(4);
    v2.push(5);
    v2.push(6);
    // v2[3]; // 边界溢出
}

#[test]
fn test_vecdeque() {
    let mut buf = VecDeque::new();
    buf.push_front(1);
    buf.push_front(2);

    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(1), Some(&1));

    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);

    assert_eq!(buf.get(2), Some(&3));
    assert_eq!(buf.get(3), Some(&4));
    assert_eq!(buf.get(4), Some(&5))
}

#[test]
fn test_linked_list() {
    let mut list1 = LinkedList::new();
    list1.push_back('a');

    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');
    list1.append(&mut list2);

    println!("{:?}", list1);
    println!("{:?}", list2);

    list1.pop_front();
    println!("{:?}", list1);

    list1.push_front('e');
    println!("{:?}", list1);

    list2.push_front('f');
    println!("{:?}", list2);
}

#[test]
fn test_hashmap_btreemap() {
    let mut hmap = HashMap::new();
    hmap.insert(3, "c");
    hmap.insert(1, "a");
    hmap.insert(2, "b");
    hmap.insert(5, "d");
    hmap.insert(4, "e");
    println!("{:?}", hmap);

    let mut bmap = BTreeMap::new();
    bmap.insert(3, "c");
    bmap.insert(2, "b");
    bmap.insert(1, "a");
    bmap.insert(5, "e");
    bmap.insert(4, "d");
    println!("{:?}", bmap);
}

#[test]
fn test_hashset_btreeset() {
    let mut hset = HashSet::new();
    hset.insert("A Song Of Ice and Fire");
    hset.insert("The Emerald City");
    hset.insert("The Odyssey");

    if !hset.contains("The Emerald City") {
        println!(
            "I have {} books, but The Emerald City isn't one.",
            hset.len()
        );
    }
    println!("{:?}", hset);

    let mut bset = BTreeSet::new();
    bset.insert("西游记");
    bset.insert("三国演义");
    bset.insert("红楼梦");
    bset.insert("水浒传");
    bset.insert("阿甘正传");
    println!("{:?}", bset);
}

#[test]
fn test_binary_heap() {
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);

    let arr = [93, 80, 48, 53, 72, 30, 18, 36, 15, 35, 45, 90];
    for &i in arr.iter() {
        heap.push(i);
    }

    assert_eq!(heap.pop(), Some(93));
    assert_eq!(heap.peek(), Some(&90));
}
