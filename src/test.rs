mod test{

    fn private_fn(){
        println!("private Toto");
    }

    pub fn public_fn(){
        println!("toto");
    }

}

test::public_fn();