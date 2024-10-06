// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u64,
    unit_price: u64,
}

impl Order {
    //validator
    fn validate(product_name: &String, quantity: &u64, unit_price: &u64) {
        if product_name.is_empty() {
            panic!("product name cannot be empty!");
        }
        if product_name.len() > 300 {
            panic!("product name is too long!");
        }
        if *quantity <= 0 {
            panic!("quantity should be greater than zero!");
        }
        if *unit_price <= 0 {
            panic!("price should be greater than zero!");
        }
    }
    //constructor
    pub fn new(product_name: String, quantity: u64, unit_price: u64) -> Order {
        Self::validate(&product_name,&quantity, &unit_price);
        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    //getters
    pub fn product_name(&self) -> &String {
        &self.product_name
    }
    pub fn quantity(&self) -> &u64 {
        &self.quantity
    }
    pub fn unit_price(&self) -> &u64 {
        &self.unit_price
    }

    //setters
    pub fn set_product_name(&mut self, new_product_name: String) {
        Self::validate(&self.product_name, &self.quantity, &self.unit_price);
        self.product_name = new_product_name;
    }
    pub fn set_quantity(&mut self, new_quantuty: u64) {
        Self::validate(&self.product_name, &self.quantity, &self.unit_price);
        self.quantity = new_quantuty;
    }
    pub fn set_unit_price(&mut self, new_unit_price: u64) {
        Self::validate(&self.product_name, &self.quantity, &self.unit_price);
        self.unit_price = new_unit_price;
    }

    //total
    pub fn total(&self) -> u64 {
        self.quantity * self.unit_price
    }
}
