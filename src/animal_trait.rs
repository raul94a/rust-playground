

pub trait AnimalTrait<T> {

    fn makeSound(self, _:T);
}

impl<T,U> AnimalTrait<T> for U {
    fn makeSound(self,_:T){
        println!("I am making a sound!! And I am the generic implementation of a Generic Trait!!");
        
    }
}

