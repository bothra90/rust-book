#[test]
fn it_works() {
    {
        use super::vectors::foo;
        foo();
    }
    {
        use super::strings::foo;
        foo();
    }
    {
        use super::hash_maps::foo;
        foo();
    }
    {
        use super::exercises::one::ls;
        println!("{:?}", ls(vec![4, 3, 5, 3, 2, 4, 4, 1, 0]));
    }
    {
        use super::exercises::two::convert_to_piglatin;
        println!("{:?}", convert_to_piglatin("This may easiy be awesome!"));
        println!("{:?}", convert_to_piglatin("When was your first apple"));
        println!("{:?}", convert_to_piglatin("The cat sat   on the\n mat"));
    }
    {
        use super::exercises::three::Company;
        use std::collections::HashMap;
        let mut comp: Company = Company {
            departments: HashMap::new(),
        };
        comp.add_to_dept("Marketing", "Scott");
        comp.add_to_dept("Marketing", "Kaushik");
        comp.add_to_dept("Sales", "Brian");
        comp.add_to_dept("Engineering", "Abhishek");
        comp.add_to_dept("Coffee Makers", "Ajeet");
        println!(
            "All in marketing: {:?}",
            comp.get_all_from_dept("Marketing")
        );
        println!("All in company: {:?}", comp.get_all_sorted());
        println!(
            "All in marketing: {:?}",
            comp.get_all_from_dept("Marketing")
        );
        println!("All in company: {:?}", comp.get_all_sorted());
    }
}
