use std::boxed::Box;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

pub fn main() -> std::io::Result<()> {
    rc_arc_properties();
    Ok(())
}

/*
Box<T> for allocating values on the heap
Rc<T>, a reference counting type that enables multiple ownership
Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

    What's the diff between refs and RCs?
      - Refs don't own and their lifetime is restricted to be shorter than the main value.
        Then the main value is dropped in the prrogram flow.
      - Rcs take ownership such that the orig creator of the value can go out of scope.
*/
#[test]
pub fn test_rc_arc_properties() {
    rc_arc_properties();
}
pub fn rc_arc_properties() {
    println!("rc_arc");
    {
        let o = 100;
        let r = &o;
        let r2 = &o;
        assert_eq!(r, r2);
        assert_eq!(r, &o);
        assert_eq!(o, *r2);
        // println!("{:p} = {:p}", r, r2);
    }
    // refs can't outlive their scope
    /*
    {
        let r;
        {
            let o = 44;
            r = &o; // err: `o` does not live long enough
        }
        assert_eq!(*r, 44);
        }
    */
    //
    // but Rc's can outlive the creator's scope.
    // I.e. God is dead.
    {
        let rc;
        {
            let o = 44;
            rc = Rc::new(o); // rc takes ownership
        }
        assert_eq!(*rc, 44);
    }
    // But Rc's are read-only
    /*
    let rc2 = rc;
    assert_eq!(*rc2, 44);
    *rc = 45;
    assert_eq!(*rc2, 45);
    */

    // Mutation
    {
        let mut o = 41;
        let r1 = &mut o;
        *r1 = 42;
        assert_eq!(o, 42);
    }

    // Refs aren't thread-safe (values can only be moved)
    {
        let mut o = 41;

        // thread::spawn(|| {
        //     // closure may outlive the current function,
        //     //but it borrows `o`, which is owned by the current function
        //     o = 42;
        // });
        // we can only move the value, creating a second one.
        thread::spawn(move || {
            o = 43; // this 'o' is a copy, not a pointer.
        });
        assert_eq!(o, 41); // this 'o' is the original and will be unchanged
    }

    // RC's aren't thread-safe either
    {
        // local
        let o = 41;
        let rc = Rc::new(o);
    }

    /*
    {
        let mut o = 41;
        let rc = Rc::new(o);
        thread::spawn(move || {
            // Error: `std::rc::Rc<i32>` cannot be sent between threads safely
            // the trait `std::marker::Send` is not implemented for `std::rc::Rc<i32>`

            *rc = 43; // this 'o' is a copy, not a pointer.
        });
        assert_eq!(o, 41); // this 'o' is the original and will be unchanged
    }
    */

    // But Arc's are.  We can share the values.
    {
        let o = 41;
        let arc = Arc::new(o);
        let arc2 = arc.clone();
        thread::spawn(move || {
            assert_eq!(*arc2, 41);
        });
        assert_eq!(*arc, 41);
    }

    // But not mutate them
    /*
    {
        let o = 41;
        let arc = Arc::new(o);
        let arc2 = arc.clone();
        thread::spawn(move || {
            *arc2 = 42; //cannot assign to data in an `Arc`
            assert_eq!(*arc2, 41);
        });
        assert_eq!(*arc, 41);
    }
    */

    // In order to mutate across threds, we need:
    // 1. Arc for lifetime management
    // 2. A Box that can be mutated. (It's allocated on the heap.)
    //    TODO: is a Box the only (safe) way?

    // First, let's look at (and into!) the box...
    {
        let o = 41;
        // note that we use 'mut' just like with refs.
        let mut bx = Box::new(o);

        // look into the box
        assert_eq!(*bx, 41);
        // put a different value in there
        *bx = 42;
        assert_eq!(*bx, 42);
    }

    // As mentioned, we need a Arc and a Box to
    // mutate across threads. Here is the code
    // before adding the threads.

    /*
        When to use Boxes:

        - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
        - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
        - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    */

    /*

     TODO: this is not yet correct

    {
        let o = 41;
        // note that we use 'mut' just like with refs.
        let arc_box = Arc::new(Box::new(o));
        // look into the box
        // .. the first '*' derefs the arc, the second
        //    derefs the box
        assert_eq!(**arc_box, 41);
        // put a different value in there
        let bx = &mut *arc_box;
        **bx = 42;
        // **arc_box = 42;
        assert_eq!(**arc_box, 42);
    }
    */

    // My use case.  For the text-based Markov Models, the hash is large.
    // At time #1, I need update it during training across threads that
    // are processing text.
    // Then I need to use the final hash, or a one-time copy, as a model
    // that can be accessed across threads when using the chain.  (Though
    // the threads are less important when using the chain, but we could
    // imagine a scenario where we need to produce a massive amount of
    // generated data.  Or want to produce a large amount in a small time.)
    {
        #[derive(Debug)]
        struct BigThing {
            size: usize,
        }
        impl Drop for BigThing {
            fn drop(&mut self) {
                println!("Dropping BigThing of size `{}`!", self.size);
            }
        }

        // inner closure
        let arc_of_box;
        {
            let singleton = BigThing { size: 1_000_000 };
            arc_of_box = Arc::new(Box::new(singleton));
        }
        assert_eq!(1_000_000, arc_of_box.size);

        let clone1 = arc_of_box.clone();
        // we have the arc->box->"big thing", now let's use it in threads
        thread::spawn(move || {
            // println!("{:?}", arc_of_box.clone());
            println!("Clone1: {:?}", clone1);
            // let big_thing = &mut **clone1;
            // big_thing.size = 1_000_001;
        });
        let clone2 = arc_of_box.clone();

        thread::spawn(move || {
            // println!("{:?}", arc_of_box);
            println!("Clone2: {:?}", clone2);
        });
    }
}
