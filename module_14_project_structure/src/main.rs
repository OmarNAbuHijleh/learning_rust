/*
 * Packages and Crates
 *
 So far, whenever we make a rust program we do so with just the "main.rs" file. We can create a new rust package using the "cargo new" command.
 A package is a folder that contains a Cargo.toml file and a src folder. The Cargo.toml file is used to configure the package and the src folder contains the source code for the package. It holds metadata about the package, such as the ame, version, and dependencies.

 We can also say that a package is a collection of one or more crates. A crate is a collection of rust code that produces a binary executable or a library. A crate is the smallest amount of code that the rust compiler considers at a time.


 There are two types of crates. A binary crate and a library crate. A binary crate is a crate that produces a binary executable. A library crate is a crate that produces a library, which can export its functionality for use by other rust programs.

 Every crate we've written so far is a binary crate.

 The "cargo new" command will default to creating a binary crate. The Cargo.toml file's field sets the name of the package (in our case - warehouse). Cargo will look for a src/main.rs file to compile and run, and if it exists it will infer the name of the crate from the file name. Cargo will also look for a src/lib.rs file to compile and run, and if it exists it will infer the name of the crate from the file name. Cargo will also look for a src/bin directory to compile and run, and if it exists it will infer the name of the crate from the file name.
 NOTE: We can have a warehouse library crate AND a warehouse binary crate, but we need to have one at minimum.
*/


/*
 * Intro to Modules
 * A module is an organizational container that encapsulates related code. Inside a module we can store constants, functions, structs, enums and others. A module collects several pieces of code together. Much like a file!
 *
 * We declare a new module using the "mod" keyword with the name in snake_case
 *
 * By default, Rust considers every module name in the module as private. To make it public, we need to use the "pub" keyword.
 */

 /*
  * The Benefits of Namespaces
  * We can use the same name in different modules without conflicts!
  */

  /*
   * Module as a File
   * Rust considers all of the modules defined here as part of the Warehouse binary crate! Rust knows it's a binary crate because we have a main.rs in our source directory.
   *
   * main.rs is called a (crate root). A crate root is the base/foundation of a crate and is the starting point for the compiler to build the crate.
   *
   * We can define module in a crate root and it's contents elsewhere! This keeps the code in the main.rs file light and organized
   */

/*
 * Module as a Folder
 *
 * We can create a directory for a module instead of a file. The folder needs to share the same name as the module. That directory also needs to contain a file named "mod.rs".
 *
 */

 /*
  * Module Ambiguity
  * The compiler will raise an error if it finds multiple definitions for the same module
  */

  /*
   * Public Enums, Public Structs, and Public Fields
   * For items in a module to be accessible from outside the module, they have to be declared public. This goes for items in a struct as well!
   */

   /*
    * Submodules
    * A submodule is a module that lives within another. It's the same idea as before. By default, the contents of a submodule are contained to the parent module unless made public by that parent module. This ensure that a sub module is only accessible if the parent module allows it
    */

    /*
     * The Crate Prefix
     * We can specify "paths" to a module using the crate prefix. This is useful when we want to access items in a module from outside the crate.
     * The first method is to use the "absolute path", which starts with the crate name followed by the module path.
     * The second method is to use the "relative path", which starts with the module name followed by the module path.
     */

     /*
      * The "use" keyword
      * The "use" keyword is used to bring items into scope. It creates a shortcut to a name in a nested module.
      * "use inventory::MANAGER" would make it so that instead of writing the whole path to MANAGER, we can just write MANAGER. We make a name in another namespace part of the current namespace!
      * "use inventory::{MANAGER, PRODUCT_CATEGORY}" This is an example of how we would bring in multiple names from a module.
      *
      * When the same name is used in multiple modules, we can use the "as" keyword to rename the item.
      */

      /*
       * The "self" keyword
       * We can make an entire module available in the current scope by using the following format:
       * "use inventory::products;" - this would make all items in the products module available in the current scope. By doing so, we can access the items by using the "products::" prefix
       *
       * We can also provide a module and item. For example:
       * use inventory::products;
       * use inventory::products::ProductCategory;
       *
       * this is valid, but we can also use the "self" keyword to bring in all items from a module and sub items.
       * use inventory::products::{self, ProductCategory};
       */

       /*
        * The "super" keyword
        * We use this to bring in items from the parent module. For example:
        * use super::products;
        * use super::products::ProductCategory;
        *
        * this is valid, but we can also use the "self" keyword to bring in all items from a module and sub items.
        * use super::products::{self, ProductCategory};
        *
        * The child module can access the parent module's items using the "super" keyword, regardless of if the parent module item is public
        */
    /*
     * Create Aliases with the "as" keyword
     * This allows us to bring in items from a module and give them a different name.
     * For example:
     * use inventory::products as prod;
     * use inventory::products::ProductCategory as Category;
     *
     * This prevents conflicts in the namespace!
     */

     /*
      * Using pub use to Export Names from Submodules
      * We can use the "use" keyword in any module, not just main.rs. We can also add the "pub" keyword in front of the "use" keyword so that the items are accessible from outside the module.
      *
      */

      /*
       * External Crates
       * A dependency is an external library crate that we pull into our project. Our code depends on it to run. We can add dependencies to our project by adding them to the Cargo.toml file.
       * For example:
       * [dependencies]
       * serde = "1.0"
       * serde_json = "1.0"
       * fake = { version = "2.9.2", features = ["derive"]}
       *
       * When we use the module from the external crate, we use the "use" keyword
       */

    /*
     * The Standard Library
     * This is a collection of modules that are part of the Rust programming language itself. We can use the "use" keyword to bring items into scope.
     * The standard library begins with "std"
     * For example:
     * "use std::fmt;"
     * "use std::io;"
     */

     /*
      * The Glob Operator
      * The glob operator is "*" and is used to bring all items into scope from a module. For example:
      * "use std::collections::*;"
      * It's generally not recommended to use the glob operator in large projects because it can lead to naming conflicts and make it harder to understand where a particular item comes from.

      * The rust prelude is a collection of items that are automatically brought into scope without needing the "use" keyword.
      Things like enums, vector, string, etc.
      */

      /*
       * Create Library Crate
       * A binary crate is one that's meant to be compiled into an executable. We can create a library crate by creating a "lib.rs" file in the root of a module.
       *
       * Remember, a package can contain a binary crate, a library crate, or both. But not neither.
       * A library crate is one that we want to be able to export and use in other libraries.
       * Remember "lib.rs" --> library crate and "main.rs" --> binary crate
       *
       * A library crate is treated as a separate set of source code compared to a binary crate.
       * We can import our library crate by writing "use <crate name>::;"
       */
    /*
     * Multiple Binary Crates
     * A rust package requires at least one binary or library crate. We can make multiple binary crates by creating multiple different folders named "bin". It will have it's own "main" function
     */
    /*
     * Documentation Comments
     * A documentation comment is a comment that is used to generate documentation for a crate, module, function, or other item.
     * Documentation comments are written using the /// syntax. This goes directly above the line that we want to document
     *
     * "cargo doc --no-deps" will give us a bunch of html files we can use to view our documentation in browser. The "--no-deps" flag will prevent cargo from generating documentation for dependencies.
     *
     *
     */


mod inventory; // If we define a module in main.rs like this, we can define the modules contents in a separate file!!
mod orders; // Folder definition of a module
mod products;

/* NOTE: Below is the other way of defining a module in main.rs
mod inventory {
    // Everything within here is part of the inventory module
    const FLOOR_SPACE: i32 = 10000;
    pub const MANAGER: &str = "Ivan Inventory";
    #[derive (Debug)]
    enum ProductCategory {
        Ladder,
        Hammer,
    }

    #[derive (Debug)]
    struct Item {
        name: String,
        category: ProductCategory,
        quantity: u32,
    }
}
*/

// mod orders {
//     pub const MANAGER: &str = "Oliver Orderson"; // MANAGER Can be used again because it's in a different namespace!
// }

fn main() {
    println!("Our managers are {} and {}. We have {} square feet of floor space", inventory::MANAGER, orders::MANAGER, inventory::FLOOR_SPACE);
    println!("Our managers are {} and {}. We have {} square feet of floor space", crate::inventory::MANAGER, orders::MANAGER, inventory::FLOOR_SPACE); // The "crate" keyword is being used to access the inventory module's MANAGER constant (absolute path). The others are relative paths
    // println!("The manager of our inventory is {}", inventory::FLOOR_SPACE); // NOTE: This won't work because FLOOR_SPACE is private
    inventory::talk_to_manager();

    let favorite_category = products::ProductCategory::Ladder;
    println!("My favorite category is {:?}", favorite_category);
    // NOTE: Even struct fields are private by default from a module! The struct itself is public but the fields are private! We have to declare individual struct fields as public if we want them to be accessible outside the module.
    let tall_ladder = products::Item {
        name: String::from("Tall Ladder"),
        category: favorite_category,
        quantity: 100,
    };
    println!("Tall ladder: {:?}", tall_ladder);


}
